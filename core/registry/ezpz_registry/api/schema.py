from uuid import UUID
from typing import Any, ClassVar
from datetime import UTC, datetime

from pydantic import Field, HttpUrl, BaseModel, ConfigDict, field_validator

from ezpz_registry.db.models import PermissionType


class PluginBase(BaseModel):
  INVALID_PACKAGE_NAME: ClassVar[str] = "Invalid package name format"
  UNIQUE_ALIAS_ERROR: ClassVar[str] = "Aliases must be unique"

  name: str = Field(..., min_length=1, max_length=100, description="Plugin display name")
  package_name: str = Field(..., min_length=1, max_length=100, description="PyPI package name")
  description: str = Field(..., min_length=1, description="Plugin description")
  aliases: list[str] = Field(default_factory=list, description="Alternative names")
  author: str | None = Field(None, max_length=100, description="Plugin author")
  homepage: HttpUrl | None = Field(None, description="Plugin homepage URL")

  @field_validator("package_name")
  @classmethod
  def validate_package_name(cls, v: str) -> str:
    import re

    if not re.match(r"^[a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$", v):
      raise ValueError(cls.INVALID_PACKAGE_NAME)
    return v.lower()

  @field_validator("aliases")
  @classmethod
  def validate_aliases(cls, v: list[str]) -> list[str]:
    if len(v) != len(set(v)):
      raise ValueError(cls.UNIQUE_ALIAS_ERROR)
    return [alias.strip() for alias in v if alias.strip()]


class PluginCreate(PluginBase):
  metadata_: dict[str, Any] | None = Field(default_factory=dict, description="Plugin metadata")
  verified: bool = Field(default=False, description="Whether plugin is verified on PyPI")
  created_at: datetime | None = Field(None, description="Creation timestamp")
  updated_at: datetime | None = Field(None, description="Update timestamp")


class PluginUpdate(BaseModel):
  name: str | None = Field(None, min_length=1, max_length=100)
  description: str | None = Field(None, min_length=1)
  aliases: list[str] | None = Field(None)
  author: str | None = Field(None, max_length=100)
  homepage: HttpUrl | None = Field(None)
  metadata_: dict[str, Any] | None = Field(None, description="Plugin metadata")


class PluginResponse(PluginBase):
  model_config = ConfigDict(from_attributes=True)

  id: UUID
  version: str | None = Field(None, description="Latest version from PyPI")
  verified: bool = Field(description="Whether plugin is verified on PyPI")
  created_at: datetime
  updated_at: datetime
  submitted_by: str | None = Field(None, description="Who submitted the plugin")
  is_deleted: bool = Field(default=False, description="Soft delete flag")


class PluginRegistrationRequest(BaseModel):
  plugin: PluginCreate
  verification_token: str | None = Field(None, description="Optional verification token")


class PluginListResponse(BaseModel):
  plugins: list[PluginResponse]
  total: int
  page: int
  page_size: int
  total_pages: int


class PluginSearchResponse(BaseModel):
  plugins: list[PluginResponse]
  query: str
  total: int


class ApiKeyCreate(BaseModel):
  name: str = Field(..., min_length=1, max_length=100, description="Key name")
  permissions: list[PermissionType] = Field(default_factory=list, description="Key permissions")
  expires_at: datetime | None = Field(None, description="Expiration date")


class ApiKeyResponse(BaseModel):
  model_config = ConfigDict(from_attributes=True)

  id: UUID
  name: str
  permissions: list[PermissionType]
  active: bool
  created_at: datetime
  expires_at: datetime | None
  last_used_at: datetime | None
  is_expired: bool = Field(description="Whether the key is expired")


class HealthResponse(BaseModel):
  status: str
  timestamp: datetime
  version: str
  database: str


class WebhookResponse(BaseModel):
  status: str
  message: str | None = None


class ErrorResponse(BaseModel):
  error: str
  detail: str | None = None
  timestamp: datetime = Field(default_factory=lambda: datetime.now(UTC))

  def model_dump(self, **kwargs: Any) -> dict[str, Any]:
    data = super().model_dump(**kwargs)
    if isinstance(data.get("timestamp"), datetime):
      data["timestamp"] = data["timestamp"].isoformat()
    return data
