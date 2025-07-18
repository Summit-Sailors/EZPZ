import re
from typing import Any, ClassVar, Optional

from pydantic import Field, HttpUrl, EmailStr, BaseModel, field_validator

from ezpz_pluginz.logger import setup_logger

logger = setup_logger("Models")

PACKAGE_NAME_REGEX = re.compile(r"^ezpz[_-][a-zA-Z0-9]([a-zA-Z0-9._-]*[a-zA-Z0-9])?$")


class PluginMetadataInner(BaseModel):
  PY_VERSION_ERROR: ClassVar[str] = "python_version must be in the format '>=3.X' (e.g., '>=3.13')"

  tags: list[str] = Field(default_factory=list, description="Tags describing the plugin")
  license: str = Field(..., description="License type (e.g., MIT, Apache-2.0)")
  python_version: str = Field(..., description="Minimum Python version (e.g., >=3.13)")
  dependencies: list[str] = Field(default_factory=list, description="List of required packages")
  documentation: HttpUrl = Field(..., description="URL to plugin documentation")
  support_email: EmailStr = Field(..., description="Contact email for support")

  @field_validator("python_version")
  def validate_python_version(cls, v: str) -> str:
    if not re.match(r"^>=3\.\d{1,2}$", v):
      raise ValueError(cls.PY_VERSION_ERROR)
    return v


class PluginMetadata(BaseModel):
  VERSION_ERROR: ClassVar[str] = "Version must follow semantic versioning (e.g., '0.1.0')"
  FIELD_ERROR: ClassVar[str] = "Field must not be empty"

  name: str = Field(..., description="Short name of the plugin")
  package_name: str = Field(..., description="Package name for installation")
  description: str = Field(..., description="Brief description of the plugin")
  aliases: list[str] = Field(default_factory=list, description="Alternative names for the plugin")
  version: str = Field(..., description="Plugin version (semantic versioning)")
  author: str = Field(..., description="Author or maintainer of the plugin")
  category: str = Field(..., description="Category of the plugin (e.g., Technical analysis)")
  homepage: HttpUrl = Field(..., description="URL to plugin homepage")
  metadata_: PluginMetadataInner = Field(..., description="Additional metadata")

  @field_validator("version")
  def validate_version(cls, v: str) -> str:
    if not re.match(r"^\d+\.\d+\.\d+$", v):
      raise ValueError(cls.VERSION_ERROR)
    return v

  @field_validator("name", "package_name", "description", "author", "category")
  def validate_non_empty(cls, v: str) -> str:
    if not v.strip():
      raise ValueError(cls.FIELD_ERROR)
    return v.strip()


class PluginCreate(PluginMetadata):
  pass  # Inherits all fields and validation from PluginMetadata


class PluginResponse(PluginMetadata):
  id: str = Field(..., description="Unique identifier for the plugin")
  created_at: str = Field(..., description="Creation timestamp")
  updated_at: str = Field(..., description="Last update timestamp")
  verified: bool = Field(default=False, description="Whether the plugin is verified")
  is_deleted: bool = Field(default=False, description="Whether the plugin is marked as deleted")


class PluginUpdate(BaseModel):
  name: Optional[str] = Field(None, description="Short name of the plugin")
  package_name: Optional[str] = Field(None, description="Package name for installation")
  description: Optional[str] = Field(None, description="Brief description of the plugin")
  aliases: Optional[list[str]] = Field(None, description="Alternative names for the plugin")
  version: Optional[str] = Field(None, description="Plugin version (semantic versioning)")
  author: Optional[str] = Field(None, description="Author or maintainer of the plugin")
  category: Optional[str] = Field(None, description="Category of the plugin")
  homepage: Optional[HttpUrl] = Field(None, description="URL to plugin homepage")
  metadata_: Optional[PluginMetadataInner] = Field(None, description="Additional metadata")

  @field_validator("version")
  def validate_version(cls, v: Optional[str]) -> Optional[str]:
    if v and not re.match(r"^\d+\.\d+\.\d+$", v):
      raise ValueError(PluginMetadata.VERSION_ERROR)
    return v

  @field_validator("name", "package_name", "description", "author", "category")
  def validate_non_empty(cls, v: Optional[str]) -> Optional[str]:
    if v is not None and not v.strip():
      raise ValueError(PluginMetadata.FIELD_ERROR)
    return v.strip() if v else v


def safe_deserialize_plugin(plugin_data: dict[str, Any]) -> Optional[PluginResponse]:
  try:
    return PluginResponse.model_validate(plugin_data)
  except Exception:
    logger.exception("Failed to deserialize plugin data")
    return None
