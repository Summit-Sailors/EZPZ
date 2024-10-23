import scrapy


class PypiScraperItem(scrapy.Item):
  # package info
  name = scrapy.Field()
  description = scrapy.Field()
  summary = scrapy.Field()
  version = scrapy.Field()
  author = scrapy.Field()
  license = scrapy.Field()
  maintainers = scrapy.Field()
  programming_languages = scrapy.Field()

  # URLs
  home_page = scrapy.Field()
  bugtrack_url = scrapy.Field()
  download_url = scrapy.Field()
  project_url = scrapy.Field()
  project_urls = scrapy.Field()
  package_url = scrapy.Field()
  release_url = scrapy.Field()

  # metadata
  keywords = scrapy.Field()
  requires_dist = scrapy.Field()
  requires_python = scrapy.Field()
  platform = scrapy.Field()
  provides_extra = scrapy.Field()
  yanked = scrapy.Field()
  yanked_reason = scrapy.Field()
  downloads = scrapy.Field()

  # releases
  releases = scrapy.Field()

  # urls
  urls = scrapy.Field()

  # vulnerabilities
  vulnerabilities = scrapy.Field()
