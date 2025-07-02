from uuid import UUID, uuid4
from typing import Any, ClassVar
from datetime import datetime, timezone
from functools import cached_property

from pydantic import (
  HttpUrl,
  field_validator,
)
from sqlmodel import Field, Column, MetaData, SQLModel, Relationship, UniqueConstraint, func, inspect
from sqlalchemy import Text, String, Boolean, Integer, DateTime, ForeignKey
from sqlalchemy.sql import expression
from sqlalchemy.dialects.postgresql import ARRAY, JSONB

from ezpz_registry.db.types.http_url import HttpUrlType

metadata_obj = MetaData()


class BaseDBModel(SQLModel):
  __abstract__ = True
  metadata = metadata_obj

  @cached_property
  def pk_names(self) -> tuple[str, ...]:
    return tuple(col.name for col in inspect(type(self)).primary_key)


# Main tables
class Plugins(BaseDBModel, table=True):
  __tablename__: str = "plugins"

  INVALID_URL_ERROR: ClassVar[str] = "Invalid homepage URL format"
  ALIASES_TYPE_ERROR: ClassVar[str] = "Aliases must be a list"

  id: UUID = Field(primary_key=True, default_factory=uuid4, nullable=False, unique=True)
  name: str = Field(max_length=100, sa_column=Column(String(100), unique=True, nullable=False, index=True))
  package_name: str = Field(max_length=100, sa_column=Column(String(100), unique=True, nullable=False, index=True))
  description: str = Field(sa_column=Column(Text, nullable=False))
  aliases: list[str] = Field(default_factory=list, sa_column=Column(ARRAY(String), default=list, nullable=False))
  version: str | None = Field(default=None, max_length=50, sa_column=Column(String(50), nullable=True))
  author: str | None = Field(default=None, max_length=100, sa_column=Column(String(100), nullable=True))
  category: str = Field(max_length=50, sa_column=Column(String(50), nullable=False, index=True))
  homepage: HttpUrl | None = Field(default=None, sa_column=Column(HttpUrlType(500), nullable=True))
  verified: bool = Field(default=False, sa_column=Column(Boolean, default=False, nullable=False, index=True))

  # Timestamps
  created_at: datetime = Field(
    default_factory=lambda: datetime.now(timezone.utc),
    sa_column=Column(DateTime(timezone=True), nullable=False, server_default=func.now()),
  )
  updated_at: datetime = Field(
    default_factory=lambda: datetime.now(timezone.utc),
    sa_column=Column(DateTime(timezone=True), nullable=False, server_default=func.now(), onupdate=func.now()),
  )

  # Soft delete
  deleted_at: datetime | None = Field(default=None, sa_column=Column(DateTime(timezone=True), nullable=True))
  is_deleted: bool = Field(default=False, sa_column=Column(Boolean, server_default=expression.false(), nullable=False))

  # Metadata
  metadata_: dict[str, Any] = Field(default_factory=dict, sa_column=Column("metadata", JSONB, default=dict, nullable=False))

  # Relationships
  downloads: list["PluginDownloads"] = Relationship(back_populates="plugin")

  @field_validator("homepage")
  def validate_homepage_url(cls, v: object) -> HttpUrl | None | object:
    if v is not None and isinstance(v, str):
      try:
        return HttpUrl(v)
      except ValueError:
        raise ValueError(cls.INVALID_URL_ERROR) from None
    return v

  @field_validator("aliases")
  def validate_aliases(cls, v: object) -> list[str]:
    if v is None:
      return list[str]()
    if not isinstance(v, list):
      raise TypeError(cls.ALIASES_TYPE_ERROR) from None
    return [alias.strip() for alias in v if alias.strip()]

  def __repr__(self) -> str:
    return f"<Plugin(name='{self.name}', package_name='{self.package_name}')>"

  @property
  def is_active(self) -> bool:
    """not soft deleted."""
    return not self.is_deleted

  def soft_delete(self) -> None:
    self.is_deleted = True
    self.deleted_at = datetime.now(timezone.utc)

  def restore(self) -> None:
    """Restore soft deleted plugin."""
    self.is_deleted = False
    self.deleted_at = None


class PluginDownloads(BaseDBModel, table=True):
  __tablename__: str = "plugin_downloads"
  __table_args__ = (UniqueConstraint("plugin_id", "date", name="unique_plugin_date"),)

  NEGATIVE_DOWNLOADS_ERROR: ClassVar[str] = "Downloads count must be non-negative"

  id: UUID = Field(primary_key=True, default_factory=uuid4, nullable=False, unique=True)
  plugin_id: UUID = Field(sa_column=Column(ForeignKey("plugins.id"), nullable=False, index=True))
  date: datetime = Field(sa_column=Column(DateTime(timezone=True), nullable=False, index=True))
  downloads: int = Field(default=0, sa_column=Column(Integer, default=0, nullable=False))

  # Timestamps
  created_at: datetime = Field(default_factory=lambda: datetime.now(timezone.utc))
  updated_at: datetime | None = Field(default=None, sa_column=Column(DateTime, onupdate=datetime.now(timezone.utc)))

  # Relationships
  plugin: Plugins = Relationship(back_populates="downloads")

  @field_validator("downloads")
  def validate_downloads(cls, v: int) -> int:
    if v < 0:
      raise ValueError(cls.NEGATIVE_DOWNLOADS_ERROR)
    return v

  def __repr__(self) -> str:
    return f"<PluginDownload(plugin_id={self.plugin_id}, date={self.date}, downloads={self.downloads})>"

  @classmethod
  def create_daily_stat(cls, plugin_id: UUID, date: datetime, downloads: int = 0) -> "PluginDownloads":
    return cls(plugin_id=plugin_id, date=date.replace(hour=0, minute=0, second=0, microsecond=0), downloads=downloads)


# Response models
class PluginResponse(SQLModel):
  id: UUID
  name: str
  package_name: str
  description: str
  aliases: list[str]
  version: str
  author: str
  category: str
  homepage: HttpUrl
  downloads: int = 0
  verified: bool = False
  created_at: datetime
  updated_at: datetime | None = None
  is_deleted: bool = False
  metadata_: dict[str, Any]

  class Config:
    from_attributes = True


class PluginDownloadResponse(SQLModel):
  id: UUID
  plugin_id: UUID
  date: datetime
  downloads: int
  created_at: datetime
  updated_at: datetime | None = None

  class Config:
    from_attributes = True


# Create/Update models
class PluginCreate(SQLModel):
  name: str = Field(max_length=100)
  package_name: str = Field(max_length=100)
  description: str
  aliases: list[str] | None = Field(default_factory=list)
  version: str | None = Field(default=None, max_length=50)
  author: str | None = Field(default=None, max_length=100)
  homepage: HttpUrl | None = None
  category: str = Field(max_length=50)
  metadata_: dict[str, Any] | None = Field(default_factory=dict)


class PluginUpdate(SQLModel):
  name: str | None = Field(default=None, max_length=100)
  package_name: str | None = Field(default=None, max_length=100)
  description: str | None = None
  aliases: list[str] | None = None
  version: str | None = Field(default=None, max_length=50)
  author: str | None = Field(default=None, max_length=100)
  homepage: HttpUrl | None = None
  verified: bool | None = None
  metadata_: dict[str, Any] | None = None
  category: str | None = Field(default=None, max_length=50)
