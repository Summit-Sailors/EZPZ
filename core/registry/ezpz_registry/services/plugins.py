from typing import TYPE_CHECKING
from datetime import datetime, timezone

from sqlalchemy import asc, or_, desc, func, select

if TYPE_CHECKING:
  from uuid import UUID

  from sqlalchemy.ext.asyncio import AsyncSession

  from ezpz_registry.api.schema import PluginCreate, PluginUpdate

from ezpz_registry.db.models import Plugins


class PluginService:
  @staticmethod
  async def create_plugin(session: "AsyncSession", plugin_data: "PluginCreate") -> Plugins:
    plugin = Plugins(
      name=plugin_data.name,
      package_name=plugin_data.package_name,
      description=plugin_data.description,
      aliases=plugin_data.aliases or [],
      author=plugin_data.author,
      category=plugin_data.category,
      homepage=plugin_data.homepage,
      version=plugin_data.version,
      verified=False,
      metadata_=plugin_data.metadata_ or {},
    )
    session.add(plugin)

    await session.commit()
    return plugin

  @staticmethod
  async def get_plugin_by_id(session: "AsyncSession", plugin_id: "UUID") -> Plugins | None:
    result = await session.execute(select(Plugins).where(Plugins.id == plugin_id, ~Plugins.is_deleted))
    return result.scalar_one_or_none()

  @staticmethod
  async def get_plugin_by_package_name(session: "AsyncSession", package_name: str) -> Plugins | None:
    result = await session.execute(select(Plugins).where(Plugins.package_name == package_name, ~Plugins.is_deleted))
    return result.scalar_one_or_none()

  @staticmethod
  async def get_plugin_by_name(session: "AsyncSession", name: str) -> Plugins | None:
    result = await session.execute(select(Plugins).where(Plugins.name == name, ~Plugins.is_deleted))
    return result.scalar_one_or_none()

  @staticmethod
  async def update_plugin(session: "AsyncSession", plugin: Plugins, update_data: "PluginUpdate") -> Plugins:
    update_dict = update_data.model_dump(exclude_unset=True)
    for field, value in update_dict.items():
      if field == "homepage" and value:
        # Handle HttpUrl conversion properly
        homepage_value = str(value) if value else None
        setattr(plugin, field, homepage_value)
      else:
        setattr(plugin, field, value)

    # Update the updated_at timestamp
    plugin.updated_at = datetime.now(timezone.utc)

    await session.commit()
    return plugin

  @staticmethod
  async def update_plugin_version(session: "AsyncSession", package_name: str, version: str) -> bool:
    result = await session.execute(select(Plugins).where(Plugins.package_name == package_name, ~Plugins.is_deleted))
    plugin = result.scalar_one_or_none()
    if plugin:
      plugin.version = version
      plugin.updated_at = datetime.now(timezone.utc)

      await session.commit()
      return True
    return False

  @staticmethod
  async def verify_plugin(session: "AsyncSession", package_name: str) -> bool:
    result = await session.execute(select(Plugins).where(Plugins.package_name == package_name, ~Plugins.is_deleted))
    plugin = result.scalar_one_or_none()
    if plugin:
      plugin.verified = True
      plugin.updated_at = datetime.now(timezone.utc)

      await session.commit()
      return True
    return False

  @staticmethod
  async def delete_plugin(session: "AsyncSession", plugin_id: "UUID") -> bool:
    result = await session.execute(select(Plugins).where(Plugins.id == plugin_id, ~Plugins.is_deleted))
    plugin = result.scalar_one_or_none()
    if plugin:
      plugin.is_deleted = True
      plugin.updated_at = datetime.now(timezone.utc)

      await session.commit()
      return True
    return False

  @staticmethod
  async def list_plugins(session: "AsyncSession", page: int = 1, page_size: int = 50, *, verified_only: bool = False) -> tuple[list[Plugins], int]:
    session.expire_all()
    query = select(Plugins).where(~Plugins.is_deleted)  # Add soft delete check

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
  async def search_plugins(session: "AsyncSession", query_text: str, page: int = 1, page_size: int = 50) -> tuple[list[Plugins], int]:
    session.expire_all()
    search_term = f"%{query_text.lower()}%"

    # search query with soft delete check
    search_query = select(Plugins).where(
      ~Plugins.is_deleted,
      or_(
        func.lower(Plugins.name).like(search_term),
        func.lower(Plugins.description).like(search_term),
        func.lower(Plugins.author).like(search_term),
        func.lower(Plugins.package_name).like(search_term),
      ),
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
