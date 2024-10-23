import json
from enum import StrEnum
from typing import TYPE_CHECKING, Any, ClassVar, Generator

import scrapy

if TYPE_CHECKING:
  from scrapy.http import TextResponse

SUCCESS_RESPONSE = 200


class Classifier(StrEnum):
  DEVELOPMENT_STATUS = "Development Status"
  INTENDED_AUDIENCE = "Intended Audience"
  LICENSE = "License"
  OPERATING_SYSTEM = "Operating System"
  PROGRAMMING_LANGUAGE = "Programming Language"
  TOPIC = "Topic"
  ENVIRONMENT = "Environment"


class PypiSpiderSpider(scrapy.Spider):
  name = "pypi_spider"
  allowed_domains: ClassVar = ["pypi.org", "pypistats.org"]
  start_urls: ClassVar = ["https://pypi.org/simple/"]

  def __init__(self, names: list[str] | None = None, *args: str, **kwargs: dict[str, Any]) -> None:
    super().__init__(*args, **kwargs)
    self.names = names

  def start_requests(self) -> Generator[scrapy.Request, Any, None]:
    if self.names:
      base_url = "https://pypi.org/pypi"
      for package_name in self.names:
        api_url = f"{base_url}/{package_name}/json"
        package_url = f"https://pypi.org/project/{package_name}/"
        pypistats_url = f"https://pypistats.org/api/packages/{package_name}/recent"

        yield scrapy.Request(
          url=api_url,
          callback=self.parse_package_data,
          meta={"package_name": package_name, "package_url": package_url, "pypistats_url": pypistats_url},
        )
    else:
      yield scrapy.Request(url=self.start_urls[0], callback=self.parse)

  def parse(self, response: "TextResponse") -> Generator[scrapy.Request, Any, None]:
    package_links = response.css("a::attr(href)").getall()

    if package_links:
      base_url = "https://pypi.org/pypi"

      for link in package_links:
        package_name = link.strip("/").split("/")[-1]
        api_url = f"{base_url}/{package_name}/json"
        package_url = f"https://pypi.org/project/{package_name}/"
        pypistats_url = f"https://pypistats.org/api/packages/{package_name}/recent"

        yield scrapy.Request(
          url=api_url,
          callback=self.parse_package_data,
          meta={"package_name": package_name, "package_url": package_url, "pypistats_url": pypistats_url},
        )

  def parse_package_data(self, response: "TextResponse") -> Generator[scrapy.Request, Any, None]:
    if response.status != SUCCESS_RESPONSE:
      self.logger.error(f"Failed to fetch data for package: {response.meta['package_name']}")
      return

    package_name = str(response.meta["package_name"])
    package_url = str(response.meta["package_url"])
    pypistats_url = str(response.meta["pypistats_url"])

    try:
      package_data: dict[str, Any] = json.loads(response.text)
    except json.JSONDecodeError:
      self.logger.exception(f"Failed to decode JSON for package: {package_name}")
      return

    yield scrapy.Request(
      url=package_url,
      callback=self.parse_additional_data,
      meta={"package_name": package_name, "package_data": package_data, "pypistats_url": pypistats_url},
    )

  def parse_additional_data(self, response: "TextResponse") -> Generator[scrapy.Request, Any, None]:
    package_name = str(response.meta["package_name"])
    package_data: dict[str, Any] = response.meta["package_data"]
    pypistats_url = str(response.meta["pypistats_url"])

    maintainers = list(set(response.css("span.sidebar-section__maintainer a span.sidebar-section__user-gravatar-text::text").getall()))

    yield scrapy.Request(
      pypistats_url, callback=self.parse_downloads, meta={"package_name": package_name, "package_data": package_data, "maintainers": maintainers}
    )

  # utility function to parse classifiers,
  def parse_classifiers(self, classifiers: list[str]) -> dict[str, str | list[str] | None]:
    parsed_classifiers: dict[str, str | list[str] | None] = {
      "development_status": None,
      "environment": [],
      "intended_audience": None,
      "license": None,
      "operating_system": None,
      "python_versions": [],
      "topics": [],
    }

    for classifier in classifiers:
      category = classifier.split("::")[0].strip()
      value = classifier.split("::")[-1].strip()

      match category:
        case Classifier.DEVELOPMENT_STATUS:
          parsed_classifiers["development_status"] = value
        case Classifier.INTENDED_AUDIENCE:
          parsed_classifiers["intended_audience"] = value
        case Classifier.LICENSE:
          parsed_classifiers["license"] = value
        case Classifier.OPERATING_SYSTEM:
          parsed_classifiers["operating_system"] = value
        case Classifier.PROGRAMMING_LANGUAGE if isinstance(parsed_classifiers["python_versions"], list):
          parsed_classifiers["python_versions"].append(value)
        case Classifier.TOPIC if isinstance(parsed_classifiers["topics"], list):
          parsed_classifiers["topics"].append(value)
        case Classifier.ENVIRONMENT if isinstance(parsed_classifiers["environment"], list):
          parsed_classifiers["environment"].append(value)
        case _:
          pass

    return parsed_classifiers

  def parse_downloads(self, response: "TextResponse") -> None | dict[str, Any]:
    package_name = str(response.meta["package_name"])
    package_data: dict[str, Any] = response.meta["package_data"]
    maintainers: list[str] = response.meta["maintainers"]

    if response.status != SUCCESS_RESPONSE:
      self.logger.error(f"Failed to fetch download statistics for package: {package_name}")
      return None

    try:
      stats_data = json.loads(response.text)
      downloads = stats_data.get("data", {})
    except json.JSONDecodeError:
      self.logger.exception(f"Failed to decode JSON for download statistics of package: {package_name}")
      downloads: dict[str, int] = {}

    package_data["info"]["maintainers"] = maintainers
    package_data["downloads"] = downloads

    classifiers: list[str] = package_data["info"].pop("classifiers", [])

    package_data["classifiers"] = self.parse_classifiers(classifiers)

    return package_data
