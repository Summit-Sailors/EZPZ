from __future__ import annotations

import asyncio
import inspect
import contextlib
import multiprocessing
import multiprocessing as mp
from typing import TYPE_CHECKING, Any, Callable, Iterable, Coroutine, ParamSpec, AsyncIterator, cast
from dataclasses import field, dataclass
from concurrent.futures import ProcessPoolExecutor

import httpx
import janus
import structlog
from pydantic_core import Url
from selectolax.parser import HTMLParser

from painlezz_async.task_manager.task_manager import TaskManager

if TYPE_CHECKING:
  import concurrent.futures

logger = structlog.stdlib.get_logger(__name__)


async def concurrent_future_to_coroutine[T](future: concurrent.futures.Future[T]) -> T:
  return await asyncio.wrap_future(future)


def default_handler(url: str, html_content: str) -> tuple[str, set[str]]:
  _url = Url(url)
  new_urls = set[str]()
  for link in HTMLParser(html_content).css("a"):
    if href := link.attributes.get("href"):
      try:
        full_url = Url(href)
      except ValueError:
        full_url = Url(f"https://{_url.host}/{href}")
      if _url.host == full_url.host:
        new_urls.add(str(full_url))
  return url, new_urls


P = ParamSpec("P")

type TMaybeAsync[**P, R] = Callable[P, Coroutine[None, None, R]] | Callable[P, R]


@dataclass
class Spider:
  start_urls: list[str]
  base_url_str: str | None = None
  url_sieve: str | Callable[[str], str] | None = None
  handler_chain: tuple[Iterable[TMaybeAsync[[str, str], tuple[str, str]]], TMaybeAsync[[str, str], Any]] | None = None
  batchable_handlers: Iterable[TMaybeAsync[[str, str], Any]] | None = None
  max_workers: int = field(default_factory=lambda: min(4, multiprocessing.cpu_count()))

  async def run(self) -> None:
    self.visited_urls = set[str]()
    try:
      with ProcessPoolExecutor(max_workers=self.max_workers, mp_context=mp.get_context("spawn")) as self.pool:
        self.url_queue = janus.Queue[str]()
        for url in self.start_urls:
          self.url_queue.async_q.put_nowait(url)
        async with asyncio.TaskGroup() as self.worker_group:
          fetcher: asyncio.Task[None] = asyncio.create_task(asyncio.sleep(0))  # TaskGroup vs TaskManager says fetcher is potentially unbounded
          async with TaskManager[Any, Any]() as self.nonresult_group, TaskManager[Any, Any]() as self.result_group:
            fetcher = self.worker_group.create_task(self._url_fetcher())
            await self.url_queue.async_q.join()
          await self.url_queue.async_q.join()
          if not fetcher.done() and self.awaiting_queue:
            fetcher.cancel()
    finally:
      self.url_queue.close()
      await self.url_queue.wait_closed()

  async def _handle_chain(self, url: str, html_str: str) -> None:
    if self.handler_chain is None:
      return
    result = (url, html_str)
    inner_handlers, terminal_handler = self.handler_chain
    for handler in inner_handlers:
      if inspect.iscoroutinefunction(handler):
        result = await handler(*result)
      else:
        result = await asyncio.wrap_future(self.pool.submit(cast(Callable[[str, str], tuple[str, str]], handler), *result))
    if inspect.iscoroutinefunction(terminal_handler):
      await terminal_handler(*result)
    else:
      await asyncio.wrap_future(self.pool.submit(cast(Callable[[str, str], tuple[str, str]], terminal_handler), *result))

  async def _handle_batchables(self, url: str, html_str: str) -> None:
    for handler in self.batchable_handlers or []:
      coro = handler(url, html_str) if inspect.iscoroutinefunction(handler) else concurrent_future_to_coroutine(self.pool.submit(handler, url, html_str))
      self.result_group.create_task(coro)

  @contextlib.asynccontextmanager
  async def set_awaiting_queue(self) -> AsyncIterator[None]:
    self.awaiting_queue = True
    yield
    self.awaiting_queue = False

  async def _url_fetcher(self) -> None:
    async with httpx.AsyncClient() as client:
      while not self.url_queue.async_q.closed:
        async with self.set_awaiting_queue():
          url = await self.url_queue.async_q.get()
        if url in self.visited_urls:
          self.url_queue.async_q.task_done()
          continue
        self.visited_urls.add(url)
        try:
          response = await client.get(url)
        except Exception as e:
          await logger.aerror(f"Error fetching {url}: {e}")
        else:

          def push_to_queue(result: concurrent.futures.Future[tuple[str, set[str]]]) -> None:
            async def apush_to_queue(result: concurrent.futures.Future[tuple[str, set[str]]]) -> None:
              if e := result.exception():
                await logger.aerror(str(e))
              else:
                url, links = result.result()
                self.visited_urls.add(url)
                for link in links:
                  if link not in self.visited_urls:
                    match self.url_sieve:
                      case str() if not link.startswith(self.url_sieve):
                        continue
                      case Callable() if not self.url_sieve(link):
                        continue
                      case _:
                        self.url_queue.async_q.put_nowait(link)

            self.nonresult_group.create_task(apush_to_queue(result))

          html_str = response.text
          concurrent_future = self.pool.submit(default_handler, url, html_str)
          concurrent_future.add_done_callback(push_to_queue)

          self.nonresult_group.create_task(concurrent_future_to_coroutine(concurrent_future))
          self.nonresult_group.create_task(self._handle_batchables(url, html_str))
          self.result_group.create_task(self._handle_chain(url, html_str))

        finally:
          self.url_queue.async_q.task_done()
      return


async def main() -> None:
  await Spider(["https://example.com/about", "https://example.com/contact"], "https://example.com").run()


if __name__ == "__main__":
  asyncio.run(main())
