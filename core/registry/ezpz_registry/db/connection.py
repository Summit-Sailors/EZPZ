from typing import TYPE_CHECKING, Any, ClassVar, AsyncGenerator
from contextlib import asynccontextmanager

from sqlalchemy.pool import NullPool
from sqlalchemy.engine.url import URL
from sqlalchemy.ext.asyncio import (
  AsyncEngine,
  AsyncSession,
  async_sessionmaker,
  create_async_engine,
)

from ezpz_registry.config import settings
from ezpz_registry.context.asession import set_session, reset_session

if TYPE_CHECKING:
  from sqlalchemy.ext.asyncio import AsyncEngine


class DatabaseManager:
  DB_INIT_ERROR: ClassVar[str] = "Database not initialized. Call initialize() first"

  def __init__(self) -> None:
    self._engine: AsyncEngine | None = None
    self._session_factory: async_sessionmaker[AsyncSession] | None = None

  def initialize(self) -> None:
    self._engine = create_async_engine(
      settings.database_url or self.get_db_url(),
      echo=settings.debug,
      poolclass=NullPool if settings.debug else None,
      pool_pre_ping=True,
      pool_recycle=3600,
    )
    self._session_factory = async_sessionmaker(
      bind=self._engine,
      class_=AsyncSession,
      expire_on_commit=False,
    )

  async def close(self) -> None:
    if self._engine:
      await self._engine.dispose()

  @property
  def engine(self) -> "AsyncEngine":
    if not self._engine:
      raise RuntimeError(self.DB_INIT_ERROR)
    return self._engine

  @property
  def session_factory(self) -> async_sessionmaker[AsyncSession]:
    if not self._session_factory:
      raise RuntimeError(self.DB_INIT_ERROR)
    return self._session_factory

  @asynccontextmanager
  async def aget_sa_session(self) -> AsyncGenerator[AsyncSession, Any]:
    session = self.session_factory()
    try:
      yield session
    except Exception:
      await session.rollback()
      raise
    finally:
      await session.close()

  async def aget_session(self) -> AsyncGenerator[AsyncSession, Any]:
    session = self.session_factory()
    token = set_session(session)
    try:
      yield session
    except Exception:
      await session.rollback()
      raise
    finally:
      await session.close()
      reset_session(token)

  def get_db_url(self, protocol: str = "postgresql+psycopg", *, sync: bool = False) -> str:
    driver = "postgresql+psycopg2" if sync else protocol
    return URL.create(
      drivername=driver,
      username=settings.db_user,
      password=settings.db_password,
      host=settings.db_host,
      port=settings.db_port,
      database=settings.db_name,
    ).render_as_string(hide_password=False)


db_manager = DatabaseManager()
