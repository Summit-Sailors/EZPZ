import asyncio
import logging
import contextlib
from typing import TYPE_CHECKING, ClassVar

import httpx

from ezpz_registry.config import settings
from ezpz_registry.db.connection import db_manager
from ezpz_registry.services.plugins import PluginService

if TYPE_CHECKING:
  from sqlalchemy.ext.asyncio import AsyncSession

logger = logging.getLogger(__name__)


class PyPIService:
  PYPI_NOT_INITIALIZED: ClassVar[str] = "PyPIService not initialized as context manager"
  SUCCESS_CODE: ClassVar[int] = 200

  def __init__(self) -> None:
    self.client: httpx.AsyncClient | None = None

  async def __aenter__(self) -> "PyPIService":
    self.client = httpx.AsyncClient(timeout=httpx.Timeout(10.0), headers={"User-Agent": "ezpz-plugin-registry/1.0.0"})
    return self

  async def __aexit__(self, exc_type: type[BaseException] | None, exc_val: BaseException | None, exc_tb: object | None) -> None:
    if self.client:
      await self.client.aclose()

  async def get_package_info(self, package_name: str) -> dict[str, str] | None:
    if not self.client:
      raise RuntimeError(self.PYPI_NOT_INITIALIZED)

    try:
      response = await self.client.get(f"https://pypi.org/pypi/{package_name}/json")

      if response.status_code == self.SUCCESS_CODE:
        data = response.json()
        info = data.get("info", {})

        return {
          "version": info.get("version", ""),
          "author": info.get("author", ""),
          "summary": info.get("summary", ""),
          "home_page": info.get("home_page", ""),
          "project_urls": info.get("project_urls", {}),
        }

    except Exception:
      logger.exception("Error fetching PyPI info for")
      return None
    return None

  async def verify_package_exists(self, package_name: str) -> bool:
    package_info = await self.get_package_info(package_name)
    return package_info is not None

  async def verify_single_plugin(self, session: "AsyncSession", package_name: str) -> bool:
    try:
      package_info = await self.get_package_info(package_name)

      if package_info:
        # Mark as verified
        await PluginService.verify_plugin(session, package_name)

        # Update version if available
        if package_info.get("version"):
          await PluginService.update_plugin_version(session, package_name, package_info["version"])

        logger.info(f"Verified plugin: {package_name} v{package_info.get('version', 'unknown')}")
        return True

    except Exception:
      logger.exception("Error verifying plugin")
      return False
    return False


class PyPIVerificationService:
  def __init__(self) -> None:
    self.running = False
    self.task: asyncio.Task[None] | None = None

  async def start(self) -> None:
    if self.running:
      return

    self.running = True
    self.task = asyncio.create_task(self._verification_loop())
    logger.info("PyPI verification service started")

  async def stop(self) -> None:
    if not self.running:
      return

    self.running = False
    if self.task:
      self.task.cancel()
      with contextlib.suppress(asyncio.CancelledError):
        await self.task

    logger.info("PyPI verification service stopped")

  async def _verification_loop(self) -> None:
    while self.running:
      try:
        await self._verify_unverified_plugins()
        await asyncio.sleep(settings.pypi_check_interval)
      except Exception:
        logger.exception("Error in PyPI verification loop")
        await asyncio.sleep(60)

  async def _verify_unverified_plugins(self) -> None:
    async with db_manager.aget_sa_session() as session:
      # Get unverified plugins
      plugins, _ = await PluginService.list_plugins(session, page=1, page_size=1000, verified_only=False)

      unverified_plugins = [p for p in plugins if not p.verified]

      if not unverified_plugins:
        logger.debug("No unverified plugins to check")
        return

      logger.info(f"Checking {len(unverified_plugins)} unverified plugins")

      async with PyPIService() as pypi_service:
        for plugin in unverified_plugins:
          try:
            success = await pypi_service.verify_single_plugin(session, plugin.package_name)

            if success:
              await session.commit()

            await asyncio.sleep(1)

          except Exception:
            logger.exception("Error verifying plugin")
            await session.rollback()


verification_service = PyPIVerificationService()
