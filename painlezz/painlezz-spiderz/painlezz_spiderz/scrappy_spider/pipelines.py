import logging
from typing import TYPE_CHECKING, Any, Type

from sqlmodel import select
from sqlalchemy.exc import IntegrityError

from pysilo_env.db_env import DB_ENV
from pysilo_database.models.db_models import Url, Info, Release, Downloads, UrlDigest, Classifier, PypiPackage, ReleaseDigest, Vulnerability

if TYPE_CHECKING:
  from uuid import UUID

  import scrapy

logger = logging.getLogger(__name__)


class PypiScraperPipeline:
  def __init__(self) -> None:
    self.deleted_packages_dict: dict[str, bool] = {}
    self.deleted_packages_fetched = False

  async def fetch_deleted_packages(self) -> None:
    async with DB_ENV.async_sessionmaker() as session, session.begin():
      package_deleted_query = select(PypiPackage.name).where(PypiPackage.deleted_at is not None)
      package_del_result = await session.exec(package_deleted_query)
      deleted_packages = package_del_result.fetchall()
      self.deleted_packages_dict = {package_name: True for package_name in deleted_packages}
      self.deleted_packages_fetched = True

  async def process_item(self, item: dict[str, Any], spider: "scrapy.Spider") -> None:
    if not self.deleted_packages_fetched:
      await self.fetch_deleted_packages()

    async with DB_ENV.async_sessionmaker() as session, session.begin():
      try:

        def construct_upsert_statement(model: Type[Any], data: list[dict[str, Any]]):
          return model().upsert(data)

        package_name = str(item["info"]["name"])

        info_data: dict[str, str | None | list[str] | dict[str, str | None] | bool] = {**item["info"]}
        info_data.pop("name", None)

        package_data_list: list[dict[str, str | None]] = [
          {"name": package_name, **({"deleted_at": None} if package_name in self.deleted_packages_dict else {})}
        ]

        upsert_package_sql = construct_upsert_statement(PypiPackage, package_data_list).returning(PypiPackage)
        package_result = await session.exec(upsert_package_sql)
        package_id = package_result.scalar().id

        info_data_list: list[dict[str, str | None | list[str] | dict[str, str | None] | bool | UUID]] = [{**info_data, "package_id": package_id}]
        upsert_info_sql = construct_upsert_statement(Info, info_data_list).returning(Info)
        info_result = await session.exec(upsert_info_sql)
        info_id = info_result.scalar().id

        # downloads data
        downloads_data_list: list[dict[str, int | None | UUID]] = [{**item["downloads"], "info_id": info_id}]
        downloads_sql = construct_upsert_statement(Downloads, downloads_data_list)
        await session.exec(downloads_sql)

        # classifiers
        classifiers_data_list: list[dict[str, str | list[str] | None | UUID]] = [{**item["classifiers"], "package_id": package_id}]
        classifiers_sql = construct_upsert_statement(Classifier, classifiers_data_list)
        await session.exec(classifiers_sql)

        # process releases
        release_list: list[dict[str, str | None | bool | dict[str, str | None] | int | UUID]] = []
        release_digests_map: dict[str, dict[str, str | UUID]] = {}

        for version, release_data_list in item.get("releases", {}).items():
          for release_data in release_data_list:
            release_update_data: dict[str, str | None | bool | dict[str, str | None] | UUID] = {
              **release_data,
              "version": version,
              "package_id": package_id,
            }
            release_update_data.pop("digests", None)
            release_list.append({**release_update_data})
            release_digests_map[release_data["md5_digest"]] = release_data["digests"]

        if release_list:
          release_stmt = construct_upsert_statement(Release, release_list).returning(Release)
          release_result = await session.exec(release_stmt)
          release_ids = release_result.fetchall()

          # Use md5_digest from result to map the digests
          release_digest_list: list[dict[str, str | UUID]] = []
          for release in release_ids:
            release_id = release[0].id
            md5_digest = release[0].md5_digest
            digest_data = release_digests_map[md5_digest]
            release_digest_list.append({**digest_data, "release_id": release_id})

          if release_digest_list:
            release_digest_stmt = construct_upsert_statement(ReleaseDigest, release_digest_list)
            await session.exec(release_digest_stmt)

        # process URLs
        url_list: list[dict[str, str | dict[str, str | None] | int | bool | None | UUID]] = []
        url_digests_map: dict[str, dict[str, str | UUID]] = {}

        for url_data in item.get("urls", []):
          url_update_data: dict[str, str | dict[str, str | None] | int | bool | None | UUID] = {
            **url_data,
            "package_id": package_id,
          }
          url_update_data.pop("digests", None)
          url_list.append({**url_update_data})
          url_digests_map[url_data["md5_digest"]] = url_data["digests"]

        if url_list:
          url_stmt = construct_upsert_statement(Url, url_list).returning(Url)
          url_result = await session.exec(url_stmt)
          url_ids = url_result.fetchall()

          url_digest_list: list[dict[str, str | UUID]] = []
          for url in url_ids:
            url_id = url[0].id
            md5_digest = url[0].md5_digest
            digest_data = url_digests_map[md5_digest]
            url_digest_list.append({**digest_data, "url_id": url_id})

          if url_digest_list:
            url_digest_stmt = construct_upsert_statement(UrlDigest, url_digest_list)
            await session.exec(url_digest_stmt)

          # process vulnerabilities
          vuln_list: list[dict[str, str | list[str] | None | UUID]] = [{**vuln_data, "package_id": package_id} for vuln_data in item.get("vulnerabilities", [])]
          if vuln_list:
            vuln_stmt = construct_upsert_statement(Vulnerability, vuln_list)
            await session.exec(vuln_stmt)

      except IntegrityError as e:
        logger.exception(f"Database integrity error: {e}")
        await session.rollback()
      except Exception as e:
        logger.exception(f"Error processing item: {e}")
        await session.rollback()
      else:
        await session.commit()
        logger.info(f"Successfully processed item: {item['info']["name"]}")
