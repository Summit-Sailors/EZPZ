from typing import Any
from dataclasses import dataclass

from ezpz_pluginz.logger import setup_logger
from ezpz_pluginz.registry.config import DEFAULT_VERSION
from ezpz_pluginz.registry.exceptions import PluginValidationError

logger = setup_logger("Models")


@dataclass
class PluginCreate:
  name: str
  package_name: str
  description: str
  aliases: list[str]
  category: str
  author: str
  metadata_: dict[str, Any] | None
  version: str
  homepage: str

  def __post_init__(self) -> None:
    self._validate()

  def _validate(self) -> None:
    if not self.name or not self.name.strip():
      raise PluginValidationError("plugin_name")
    if not self.package_name or not self.package_name.strip():
      raise PluginValidationError("package_name")
    if not self.description or not self.description.strip():
      raise PluginValidationError("description")
    if not self.author or not self.author.strip():
      raise PluginValidationError("author")


@dataclass(frozen=True)
class PluginResponse:
  id: str
  name: str
  package_name: str
  description: str
  aliases: list[str]
  version: str
  author: str
  category: str
  homepage: str
  created_at: str
  updated_at: str
  metadata_: dict[str, Any]
  downloads: int = 0
  verified: bool = False
  is_deleted: bool = False


def safe_deserialize_plugin(plugin_data: dict[str, Any]) -> PluginResponse | None:
  try:
    return PluginResponse(
      id=plugin_data.get("id", ""),
      name=plugin_data.get("name", ""),
      package_name=plugin_data.get("package_name", ""),
      description=plugin_data.get("description", ""),
      aliases=plugin_data.get("aliases", []),
      category=plugin_data.get("category", ""),
      author=plugin_data.get("author", ""),
      version=plugin_data.get("version", DEFAULT_VERSION),
      homepage=plugin_data.get("homepage", ""),
      metadata_=plugin_data.get("metadata_", {}),
      created_at=plugin_data.get("created_at", ""),
      updated_at=plugin_data.get("updated_at", ""),
      verified=plugin_data.get("verified", False),
      is_deleted=plugin_data.get("is_deleted", False),
    )
  except Exception:
    logger.exception("Failed to deserialize plugin data")
    return None
