from __future__ import annotations

import json
import asyncio
import contextlib
import multiprocessing as mp
from enum import Enum, IntEnum
from queue import Empty as SyncQueueEmpty
from typing import TYPE_CHECKING, Any, Optional, Annotated, Generator, ParamSpec, AsyncGenerator
from asyncio import Task, TaskGroup
from pathlib import Path
from itertools import batched
from threading import Event
from collections import deque
from dataclasses import dataclass
from concurrent.futures import ProcessPoolExecutor

import httpx
import janus
import typer
import polars as pl
from httpx import Response
from pydantic import BaseModel
from tenacity import retry, wait_exponential, stop_after_attempt
from rich.console import Console
from rich.progress import Progress, BarColumn, TextColumn, TimeElapsedColumn
from selectolax.parser import HTMLParser

from painlezz_async.task_manager.task_manager import TaskManager

if TYPE_CHECKING:
  import concurrent.futures

P = ParamSpec("P")


async def concurrent_future_to_coroutine[T](future: "concurrent.futures.Future[T]") -> T:
  return await asyncio.wrap_future(future)


@retry(stop=stop_after_attempt(7), wait=wait_exponential(multiplier=1, min=4, max=10))
async def handle_task(package_name: str, client: httpx.AsyncClient) -> tuple[str, "Response"] | tuple[str, None]:
  with contextlib.suppress(httpx.TimeoutException):
    return package_name, await client.get(f"https://pypi.org/pypi/{package_name}/json")
  return package_name, None


async def aget_responses(chunk: tuple[str, ...]) -> list[tuple[str, "Response"] | tuple[str, None]]:
  tasks = list[Task[tuple[str, Response] | tuple[str, None]]]()
  async with httpx.AsyncClient(timeout=30) as client, TaskManager[Any, tuple[str, "Response"] | tuple[str, None]]() as tg:
    tasks = [tg.create_task(handle_task(package_name, client)) for package_name in chunk]
  return [task.result() for task in tasks]


def get_responses(chunk: tuple[str, ...]) -> list[tuple[str, "Response"] | tuple[str, None]]:
  return asyncio.run(aget_responses(chunk))


class ResultDTO(BaseModel):
  name: str
  code: int
  raw: str


def yield_package_names(resp: Response) -> Generator[str, None, None]:
  for link in HTMLParser(resp.text).css("a"):
    if href := link.attributes.get("href"):
      yield href.strip("/").split("/")[-1]


class EProgressKind(Enum):
  REQUEST = "REQUEST"
  WRITE = "WRITE"


class ProgressUpdateDTO(BaseModel):
  kind: EProgressKind
  advance: int


@dataclass(frozen=True, slots=True)
class ProducersCallBack:
  results_async_q: janus.AsyncQueue[ResultDTO]
  update_async_q: janus.AsyncQueue[ProgressUpdateDTO]
  producers: TaskManager[None, None]

  def __call__(self, task: "Task[list[tuple[str, Response] | tuple[str, None]]]") -> "Task[list[tuple[str, Response] | tuple[str, None]]]":
    results = [
      ResultDTO(name=package_name, code=resp.status_code, raw=json.dumps(resp.json()) if resp.status_code == 200 else resp.text)
      if resp is not None
      else ResultDTO(name=package_name, code=-1, raw="")
      for package_name, resp in task.result()
    ]
    self.producers.create_task(self.push_to_queue(results))
    return task

  async def push_to_queue(self, results: list[ResultDTO]) -> None:
    for result in results:
      await self.results_async_q.put(result)
    await self.update_async_q.put(ProgressUpdateDTO(kind=EProgressKind.REQUEST, advance=len(results)))


def write_queue_consumer(
  file_path: Path,
  write_sync_q: janus.SyncQueue[ResultDTO],
  update_sync_q: janus.SyncQueue[ProgressUpdateDTO],
  keep_queue_alive: Event,
  *,
  batch_size: int = 100,
) -> None:
  with file_path.open(mode="a") as f:
    batch_deque = deque[ResultDTO]()
    while not write_sync_q.closed:
      batch_deque.clear()
      for _ in range(batch_size):
        while keep_queue_alive.is_set() or (not write_sync_q.closed and write_sync_q.qsize()):
          with contextlib.suppress(SyncQueueEmpty):
            batch_deque.append(write_sync_q.get(timeout=10))
            write_sync_q.task_done()
            break
      batch_df = pl.DataFrame(batch_deque)
      batch_df.write_csv(f, include_header=False)
      update_sync_q.put(ProgressUpdateDTO(kind=EProgressKind.WRITE, advance=len(batch_df)))


def update_queue_consumer(update_sync_q: janus.SyncQueue[ProgressUpdateDTO], keep_queue_alive: Event) -> None:
  with Progress(TextColumn("[bold blue]{task.description}", justify="right"), BarColumn(), "•", TimeElapsedColumn()) as progress:
    kind_to_id = {progress_kind: progress.add_task(progress_kind.name, start=False) for progress_kind in EProgressKind}
    while keep_queue_alive.is_set() or (not update_sync_q.closed and update_sync_q.qsize()):
      with contextlib.suppress(SyncQueueEmpty):
        update_dto = update_sync_q.get(timeout=10)
        progress.update(kind_to_id[update_dto.kind], advance=update_dto.advance)
        update_sync_q.task_done()


class ECpuProfile(IntEnum):
  MIN = 1
  SM = 4
  MD = 8
  LG = 16
  MAX = mp.cpu_count()

  @classmethod
  def get(cls) -> ECpuProfile:
    match cpu_count := mp.cpu_count():
      case _ if cpu_count < cls.SM:
        default_profile = cls.MIN
      case _ if cpu_count < cls.MD:
        default_profile = cls.SM
      case _ if cpu_count < cls.LG:
        default_profile = cls.MD
      case _ if cpu_count < 32:
        default_profile = cls.LG
      case _:
        default_profile = cls.MAX
    return default_profile

  def get_batch_size(self) -> int:
    match self:
      case ECpuProfile.MIN:
        return 50
      case ECpuProfile.SM:
        return 100
      case ECpuProfile.MD:
        return 250
      case ECpuProfile.LG:
        return 500
      case ECpuProfile.MAX:
        return 1_000

  def get_file_path(self) -> Path:
    return Path(f"./data/pypi_pi_{self.name}.csv")


@contextlib.asynccontextmanager
async def queue_lifespan() -> AsyncGenerator[tuple[janus.Queue[ResultDTO], janus.Queue[ProgressUpdateDTO]], None]:
  write_queue = janus.Queue[ResultDTO]()
  update_queue = janus.Queue[ProgressUpdateDTO]()
  yield write_queue, update_queue
  await write_queue.async_q.join()
  await update_queue.async_q.join()
  write_queue.close()
  await write_queue.wait_closed()
  update_queue.close()
  await update_queue.wait_closed()


async def process_packages(cpu_profile: ECpuProfile) -> None:
  console = Console()
  console.print("initializing...")
  BATCH_SIZE = cpu_profile.get_batch_size()
  CSV_PATH = cpu_profile.get_file_path()

  keep_queue_alive = Event()
  keep_queue_alive.set()

  async with TaskManager[None, None]() as queue_consumers, queue_lifespan() as (write_queue, update_queue):
    console.print("starting consumer threads...")
    queue_consumers.create_task(asyncio.to_thread(write_queue_consumer, CSV_PATH, write_queue.sync_q, update_queue.sync_q, keep_queue_alive))
    queue_consumers.create_task(asyncio.to_thread(update_queue_consumer, update_queue.sync_q, keep_queue_alive))
    console.print("creating process pool...")
    with ProcessPoolExecutor(max_workers=cpu_profile, mp_context=mp.get_context("spawn")) as pool:
      console.print("creating async managers...")
      async with TaskManager[None, Any]() as producers, TaskGroup() as process_result_callbacks:
        done_callback = ProducersCallBack(results_async_q=write_queue.async_q, update_async_q=update_queue.async_q, producers=producers)
        console.print("LAUNCH")
        async with httpx.AsyncClient() as client:
          resp = await client.get("https://pypi.org/simple/")
        for chunk in batched(yield_package_names(resp), BATCH_SIZE):
          task = process_result_callbacks.create_task(concurrent_future_to_coroutine(pool.submit(get_responses, chunk)))
          task.add_done_callback(done_callback)
    keep_queue_alive.clear()


app = typer.Typer()


@app.command()
def zoom(cpu_profile: Annotated[Optional[ECpuProfile], typer.Option("-p", envvar="CPU_PROFILE")] = None) -> None:  # noqa: UP007
  if cpu_profile and cpu_profile > ECpuProfile.MAX:
    raise IndexError()
  asyncio.run(process_packages(cpu_profile or ECpuProfile.get()))


if __name__ == "__main__":
  app()
