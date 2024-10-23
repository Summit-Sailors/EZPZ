import sys
import json
import asyncio
import logging
import contextlib
import multiprocessing as mp
from queue import Empty as SyncQueueEmpty
from typing import TYPE_CHECKING, NoReturn, Annotated, ParamSpec, AsyncIterator
from asyncio import TaskGroup
from pathlib import Path
from functools import partial
from itertools import batched
from threading import Event
from concurrent.futures import ProcessPoolExecutor

import httpx
import janus
import typer
import polars as pl
from tqdm import tqdm as synctqdm
from pydantic import BaseModel
from tenacity import retry, wait_exponential, stop_after_attempt
from tqdm.asyncio import tqdm as asynctdqm
from selectolax.parser import HTMLParser

from painlezz_spiderz.process_data import process_batch, write_database

if TYPE_CHECKING:
  import concurrent.futures
  from asyncio import Task

  from httpx import Response
  from tqdm.asyncio import tqdm_asyncio


P = ParamSpec("P")
SUCCESS_CODE = 200


async def concurrent_future_to_coroutine[T](future: "concurrent.futures.Future[T]") -> T:
  return await asyncio.wrap_future(future)


@retry(stop=stop_after_attempt(7), wait=wait_exponential(multiplier=1, min=4, max=10))
async def handle_task(package_name: str, client: httpx.AsyncClient) -> tuple[str, "Response"] | tuple[str, None]:
  with contextlib.suppress(httpx.TimeoutException):
    return package_name, await client.get(f"https://pypi.org/pypi/{package_name}/json")
  return package_name, None


async def aget_responses(chunk: tuple[str, ...]) -> list[tuple[str, "Response"] | tuple[str, None]]:
  async with httpx.AsyncClient(timeout=30) as client, TaskGroup() as tg:
    tasks = [tg.create_task(handle_task(package_name, client)) for package_name in chunk]
  return [task.result() for task in tasks]


def get_responses(chunk: tuple[str, ...]) -> list[tuple[str, "Response"] | tuple[str, None]]:
  return asyncio.run(aget_responses(chunk))


class ResultDTO(BaseModel):
  name: str
  code: int
  raw: str


CSV_PATH_SM = Path("./data/pypi_pi_sm.csv")
CSV_PATH_LG = Path("./data/pypi_pi_lg.csv")


async def yield_package_names() -> AsyncIterator[str]:
  async with httpx.AsyncClient() as client:
    resp = await client.get("https://pypi.org/simple/")
  for link in HTMLParser(resp.text).css("a"):
    if href := link.attributes.get("href"):
      yield href.strip("/").split("/")[-1]


async def push_to_queue(results: list[ResultDTO], async_q: janus.AsyncQueue[ResultDTO]) -> None:
  for result in results:
    await async_q.put(result)


def done_callback(
  task: "Task[list[tuple[str, Response] | tuple[str, None]]]",
  *,
  async_q: janus.AsyncQueue[ResultDTO],
  request_pbar: "tqdm_asyncio[NoReturn]",
  producers: asyncio.TaskGroup,
) -> "Task[list[tuple[str, Response] | tuple[str, None]]]":
  results = [
    ResultDTO(name=package_name, code=resp.status_code, raw=json.dumps(resp.json()) if resp.status_code == SUCCESS_CODE else resp.text)
    if resp is not None
    else ResultDTO(name=package_name, code=-1, raw="")
    for package_name, resp in task.result()
  ]
  producers.create_task(push_to_queue(results, async_q))
  request_pbar.update(len(results))
  return task


def consumer(file_path: Path, sync_q: janus.SyncQueue[ResultDTO], csv_pbar: "synctqdm[NoReturn]", join_event: Event, *, batch_size: int = 100) -> None:
  with file_path.open(mode="a") as f:
    while not sync_q.closed:
      batch = list[ResultDTO]()
      for _ in range(batch_size):
        while not join_event.is_set() or sync_q.qsize():
          with contextlib.suppress(SyncQueueEmpty):
            batch.append(sync_q.get(timeout=10))
            sync_q.task_done()
            break
      if not batch:
        continue

      schema = {"package_name": pl.String, "status_code": pl.Int64, "json_data": pl.String}
      batch_df = pl.DataFrame(batch, schema).filter(pl.col("status_code") == SUCCESS_CODE)
      try:
        processed_lf = process_batch(batch_df.lazy())
        if processed_lf:
          try:
            write_database(processed_lf)
          except Exception as db_error:
            logging.info(f"Error writing to database: {db_error}")
            sys.exit(1)

      except Exception as batch_error:
        logging.info(f"Error processing batch: {batch_error}")
        sys.exit(1)
      csv_pbar.update(len(batch))


async def process_packages(max_workers: int, packages_to_process: list[str] | None = None) -> None:
  write_queue = janus.Queue[ResultDTO]()
  csv_pbar = synctqdm(position=1)
  request_pbar = asynctdqm(position=0)
  join_event = Event()
  with ProcessPoolExecutor(max_workers=max_workers, mp_context=mp.get_context("spawn")) as pool:
    async with TaskGroup() as producers, TaskGroup() as process_result_callbacks:
      IS_SM_RUN = max_workers <= 4
      consumer_task = asyncio.create_task(asyncio.to_thread(consumer, CSV_PATH_SM if IS_SM_RUN else CSV_PATH_LG, write_queue.sync_q, csv_pbar, join_event))
      # using the provided list of packages if available, otherwise all packages
      package_iterator = packages_to_process or [i async for i in yield_package_names()]
      for idx, chunk in enumerate(batched(package_iterator, 1_000)):
        if IS_SM_RUN and idx >= 2:
          break
        task = process_result_callbacks.create_task(concurrent_future_to_coroutine(pool.submit(get_responses, chunk)))
        task.add_done_callback(partial(done_callback, async_q=write_queue.async_q, request_pbar=request_pbar, producers=producers))
  join_event.set()
  await write_queue.async_q.join()
  write_queue.close()
  await write_queue.wait_closed()
  await consumer_task


app = typer.Typer()
CPU_COUNT = mp.cpu_count()


@app.command()
def zoom(zoom: Annotated[bool, typer.Option("-z")] = False) -> None:
  asyncio.run(process_packages(min(4, mp.cpu_count()) if not zoom else mp.cpu_count()))


if __name__ == "__main__":
  app()
