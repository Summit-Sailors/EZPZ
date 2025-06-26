import hashlib
from typing import TYPE_CHECKING
from datetime import datetime, timezone

from sqlmodel import asc, or_, desc, func, select

from ezpz_registry.db.models import Plugins

if TYPE_CHECKING:
  from sqlalchemy.ext.asyncio import AsyncSession

  from ezpz_registry.api.schema import PluginCreate, PluginUpdate


class PluginService:
  @staticmethod
  async def create_plugin(session: "AsyncSession", plugin_data: "PluginCreate", submitted_by: str | None = None) -> Plugins:
    plugin = Plugins(
      name=plugin_data.name,
      package_name=plugin_data.package_name,
      description=plugin_data.description,
      aliases=plugin_data.aliases,
      author=plugin_data.author,
      homepage=plugin_data.homepage,
      submitted_by=submitted_by,
      verification_token=PluginService._generate_verification_token(plugin_data.package_name),
    )

    session.add(plugin)
    await session.flush()
    return plugin

  @staticmethod
  async def get_plugin_by_id(session: "AsyncSession", plugin_id: int) -> Plugins | None:
    result = await session.execute(select(Plugins).where(Plugins.id == plugin_id))
    return result.scalar_one_or_none()

  @staticmethod
  async def get_plugin_by_package_name(session: "AsyncSession", package_name: str) -> Plugins | None:
    result = await session.execute(select(Plugins).where(Plugins.package_name == package_name))
    return result.scalar_one_or_none()

  @staticmethod
  async def get_plugin_by_name(session: "AsyncSession", name: str) -> Plugins | None:
    result = await session.execute(select(Plugins).where(Plugins.name == name))
    return result.scalar_one_or_none()

  @staticmethod
  async def update_plugin(session: "AsyncSession", plugin: Plugins, update_data: "PluginUpdate") -> Plugins:
    update_dict = update_data.model_dump(exclude_unset=True)

    for field, value in update_dict.items():
      if field == "homepage" and value:
        homepage_value = str(value)
        setattr(plugin, field, homepage_value)
      else:
        setattr(plugin, field, value)

    await session.flush()
    return plugin

  @staticmethod
  async def update_plugin_version(session: "AsyncSession", package_name: str, version: str) -> bool:
    result = await session.execute(select(Plugins).where(Plugins.package_name == package_name))
    plugin = result.scalar_one_or_none()

    if plugin:
      plugin.version = version
      await session.flush()
      return True
    return False

  @staticmethod
  async def verify_plugin(session: "AsyncSession", package_name: str) -> bool:
    result = await session.execute(select(Plugins).where(Plugins.package_name == package_name))
    plugin = result.scalar_one_or_none()

    if plugin:
      plugin.verified = True
      await session.flush()
      return True
    return False

  @staticmethod
  async def list_plugins(session: "AsyncSession", page: int = 1, page_size: int = 50, *, verified_only: bool = False) -> tuple[list[Plugins], int]:
    query = select(Plugins)

    if verified_only:
      query = query.where(Plugins.verified)

    # Get total count
    count_query = select(func.count()).select_from(query.subquery())
    total_result = await session.execute(count_query)
    total = total_result.scalar() or 0

    # Get paginated results
    query = query.order_by(desc(Plugins.verified), asc(Plugins.name))
    query = query.offset((page - 1) * page_size).limit(page_size)

    result = await session.execute(query)
    plugins = result.scalars().all()

    return list(plugins), total

  @staticmethod
  async def search_plugins(session: "AsyncSession", query: str, page: int = 1, page_size: int = 50) -> tuple[list[Plugins], int]:
    search_term = f"%{query}%"

    # Create search query
    search_query = select(Plugins).where(
      or_(
        func.lower(Plugins.name).like(search_term),
        func.lower(Plugins.description).like(search_term),
        func.lower(Plugins.author).like(search_term),
        func.lower(Plugins.package_name).like(search_term),
      )
    )

    # Get total count
    count_query = select(func.count()).select_from(search_query.subquery())
    total_result = await session.execute(count_query)
    total = total_result.scalar() or 0

    # Get paginated results
    search_query = search_query.order_by(desc(Plugins.verified), asc(Plugins.name))
    search_query = search_query.offset((page - 1) * page_size).limit(page_size)

    result = await session.execute(search_query)
    plugins = result.scalars().all()

    return list(plugins), total

  @staticmethod
  async def delete_plugin(session: "AsyncSession", plugin_id: int) -> bool:
    plugin = await PluginService.get_plugin_by_id(session, plugin_id)
    if plugin:
      await session.delete(plugin)
      await session.flush()
      return True
    return False

  @staticmethod
  def _generate_verification_token(package_name: str) -> str:
    data = f"{package_name}:{datetime.now(timezone.utc).isoformat()}"
    return hashlib.sha256(data.encode()).hexdigest()[:16]
