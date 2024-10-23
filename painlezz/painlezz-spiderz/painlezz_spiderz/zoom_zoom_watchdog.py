import asyncio
import logging
from itertools import batched

import polars as pl
from httpx import HTTPError, AsyncClient
from selectolax.parser import HTMLParser

from painlezz_spiderz.zoom_zoom import CPU_COUNT, CSV_PATH_SM, process_packages


class PackageMonitor:
  def __init__(self, initial_packages_df: pl.DataFrame, update_interval: int = 3600, batch_size: int = 100) -> None:
    self.packages_df = initial_packages_df
    self.update_interval = update_interval
    self.batch_size = batch_size
    self.rss_url = "https://pypi.org/rss/updates.xml"
    self.packages_to_update: set[str] = set()

  async def monitor_packages(self):
    while True:
      await self.check_for_updates()
      if self.packages_to_update:
        yield list(self.packages_to_update)
        self.packages_to_update.clear()
      await asyncio.sleep(self.update_interval)

  async def check_for_updates(self) -> None:
    try:
      packages = await self.fetch_rss_feed()
      await asyncio.gather(*(self.process_batch(batch) for batch in batched(packages, self.batch_size)))
    except HTTPError as e:
      logging.exception(f"Failed to fetch RSS feed: {e}")
    except Exception as e:
      logging.exception(f"Unexpected error during package update check: {e}")

  async def fetch_rss_feed(
    self,
  ) -> list[
    tuple[
      str,
      str,
    ]
  ]:
    async with AsyncClient() as client:
      resp = await client.get(self.rss_url)
      resp.raise_for_status()

    parser = HTMLParser(resp.text)
    packages: list[tuple[str, str]] = []

    for item in parser.css("item"):
      title = item.css_first("title").text()
      package_name, version = title.split()
      packages.append((package_name, version))

    return packages

  async def process_batch(self, batch: tuple[tuple[str, str], ...]) -> None:
    for package_name, version in batch:
      if self.is_package_new(package_name) or self.is_package_updated(package_name, version):
        self.packages_to_update.add(package_name)

  def is_package_new(self, package_name: str) -> bool:
    return self.packages_df.filter(pl.col("title") == package_name).is_empty()

  def is_package_updated(self, package_name: str, version: str) -> bool:
    package_row = self.packages_df.filter(pl.col("title") == package_name)
    if package_row.is_empty():
      return True
    stored_version = package_row.select("version").item()
    return version != stored_version


async def create_package_monitor(initial_packages_df: pl.DataFrame, update_interval: int = 3600) -> PackageMonitor:
  return PackageMonitor(initial_packages_df, update_interval)


async def monitor_and_process(monitor: PackageMonitor, max_workers: int) -> None:
  async for packages_to_update in monitor.monitor_packages():
    await process_packages(max_workers, packages_to_update)


async def main() -> None:
  initial_packages_df = pl.read_csv(CSV_PATH_SM)  # here we use the info dataframe generated from data/generate_dfs.py file (read from database)
  monitor = await create_package_monitor(initial_packages_df)

  await monitor_and_process(monitor, CPU_COUNT)
