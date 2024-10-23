import logging
import xml.etree.ElementTree as ET
from typing import Any, ClassVar, Generator

import scrapy
import scrapy.http
from sqlmodel import select
from scrapy.crawler import CrawlerRunner
from py_spider.spiders.pypi_spider import PypiSpiderSpider

from pysilo_env import ENV
from pysilo_database.models.db_models import PypiPackage

logger = logging.getLogger(__name__)


class MonitorSpider(scrapy.Spider):
  name = "monitor_spider"
  allowed_domains: ClassVar = ["pypi.org"]
  start_urls: ClassVar = ["https://pypi.org"]

  def __init__(self, *args: str, **kwargs: dict[str, Any]) -> None:
    super().__init__(*args, **kwargs)
    self.updated_packages: list[str] = []
    self.fetcher_triggered = False

  def start_requests(self) -> Generator[scrapy.Request, Any, None]:
    try:
      with ENV.get_sa_session() as session, session.begin():
        statement = select(PypiPackage)
        packages = session.exec(statement).all()
        logger.info(packages)
        if not packages:
          self.logger.info("No packages found in the database. Triggering PypiSpiderSpider.")
          self.trigger_fetcher()
          return
        for package in packages:
          rss_url = f"https://pypi.org/rss/project/{package.name}/releases.xml"
          yield scrapy.Request(
            url=rss_url,
            callback=self.parse_rss,
            errback=self.handle_request_error,
            meta={"package_name": package.name, "current_version": package.info.version},
          )
        session.commit()
    except Exception as e:
      self.logger.warning(f"An error occurred: {e}")
      raise

  def parse_rss(self, response: scrapy.http.Response) -> None:
    package_name: str = response.meta["package_name"]
    current_version: str = response.meta["current_version"]
    try:
      root = ET.fromstring(response.text)
      items = root.find("channel").findall("item")
      if not items:
        self.logger.warning(f"No releases found in RSS feed for {package_name}")
        return
      latest_version = items[0].find("title").text
      if latest_version and latest_version != current_version:
        self.logger.info(f"New version detected for {package_name}: {latest_version}.")
        self.updated_packages.append(package_name)
    except ET.ParseError:
      self.logger.exception(f"Failed to parse RSS feed for {package_name}")
    except Exception as e:
      self.logger.exception(f"Unexpected error processing RSS feed for {package_name}: {e}")
    # trigger the fetcher spider if any new packages are updated
    if self.updated_packages and not self.fetcher_triggered:
      self.trigger_fetcher()

  def trigger_fetcher(self) -> None:
    if self.updated_packages:
      try:
        runner = CrawlerRunner()
        runner.crawl(PypiSpiderSpider, names=self.updated_packages)
        self.updated_packages.clear()
        self.fetcher_triggered = True
      except Exception as e:
        self.logger.exception(f"Failed to trigger fetcher spider: {e}")

  def handle_request_error(self, failure: Any) -> None:
    request = failure.request
    package_name = request.meta.get("package_name", "Unknown")
    self.logger.error(f"Failed to fetch RSS feed for {package_name}: {failure.getErrorMessage}")
