import logging
from typing import TYPE_CHECKING, Any, ClassVar, Iterable, Optional, Generator
from pathlib import Path
from operator import attrgetter
from itertools import chain, groupby

import toml
import libcst as cst
from pydantic import Field, BaseModel

from ezpz_pluginz.register_plugin_macro import PolarsPluginCollector

if TYPE_CHECKING:
  from ezpz_pluginz.register_plugin_macro import PolarsPluginMacroMetadataPD

__all__ = ["EzpzPluginConfig"]

logger = logging.getLogger(__name__)

EZPZ_TOML_FILENAME = "ezpz.toml"
EZPZ_PROJECT_LOCKFILE_FILENAME = "ezpz-lock.yaml"


def group_models_by_key[T: BaseModel](data: Iterable[T], key: str) -> dict[str, set[T]]:
  sorted_data = sorted(data, key=attrgetter(key))
  return {k: set(v) for k, v in groupby(sorted_data, key=attrgetter(key))}


def _process_file(path: "Path") -> set["PolarsPluginMacroMetadataPD"]:
  plugin_visitor = PolarsPluginCollector()
  cst.parse_module(path.read_text()).visit(plugin_visitor)
  logger.debug(f"_process_file: {path}")
  logger.debug(f"_process_file:return: {plugin_visitor.macro_data}")
  return set(plugin_visitor.macro_data)


def process_includes(paths: Iterable["Path"]) -> "Generator[PolarsPluginMacroMetadataPD, Any, None]":
  for path in paths:
    if path.is_file():
      yield from _process_file(path)
    elif path.is_dir():
      sub_toml = path.joinpath(EZPZ_TOML_FILENAME)
      if sub_toml.exists():
        yield from process_includes(path.joinpath(subpath) for subpath in EzpzPluginConfig.from_toml_path(sub_toml).include)
      else:
        yield from process_includes(chain(path.rglob("*.py"), path.rglob("*.pyi")))


def get_plugins(project_toml_path: Path) -> dict[str, set["PolarsPluginMacroMetadataPD"]]:
  ezpz_pluginz = EzpzPluginConfig.from_toml_path(project_toml_path)
  return group_models_by_key(set(process_includes(ezpz_pluginz.include)), "polars_ns")


class EzpzPluginConfig(BaseModel):
  INVALID_SECTION: ClassVar[str] = "No valid [ezpz_pluginz] or [tool.ezpz_pluginz] section found in specified path"

  name: str
  include: list[Path]
  site_customize: bool | None = Field(default=None)

  @property
  def include_str_paths(self) -> list[str]:
    return [str(path) for path in self.include]

  @staticmethod
  def from_toml_path(path: Path) -> "EzpzPluginConfig":
    try:
      with path.open("r") as f:
        data = toml.load(f)
      toml_data = EzpzPluginToml(**data)

      if path.name == EZPZ_TOML_FILENAME and toml_data.ezpz_pluginz:
        return toml_data.ezpz_pluginz
      if path.name == "pyproject.toml" and toml_data.tool and "ezpz" in toml_data.tool:
        return EzpzPluginConfig(**toml_data.tool["ezpz"])
    except Exception:
      logger.exception(f"Error loading config from {path}")
      raise
    else:
      raise ValueError(EzpzPluginConfig.INVALID_SECTION)

  @staticmethod
  def get_plugins(project_toml_path: Path) -> dict[str, set["PolarsPluginMacroMetadataPD"]]:
    ezpz_pluginz = EzpzPluginConfig.from_toml_path(project_toml_path)
    return group_models_by_key(set(process_includes(ezpz_pluginz.include)), "polars_ns")


class EzpzPluginToml(BaseModel):
  ezpz_pluginz: Optional[EzpzPluginConfig] = None
  tool: Optional[dict[str, Any]] = None


def load_config(config_path: str | Path | None = None) -> Optional[EzpzPluginConfig]:
  if config_path is None:
    config_path = find_ezpz_toml()
    if config_path is None:
      logger.warning("Could not find ezpz.toml or pyproject.toml with [tool.ezpz_pluginz]")
      return None

  config_path = Path(config_path)
  if not config_path.exists():
    logger.error(f"Config file does not exist: {config_path}")
    return None

  try:
    return EzpzPluginConfig.from_toml_path(config_path)
  except Exception:
    logger.exception(f"Error loading config from {config_path}")
    return None


def find_ezpz_toml(start_path: Path | None = None) -> Optional[Path]:
  if start_path is None:
    start_path = Path.cwd()

  current_dir = Path(start_path).resolve()

  for parent in [current_dir, *list(current_dir.parents)]:
    config_file = parent / EZPZ_TOML_FILENAME
    if config_file.exists():
      logger.debug(f"Found ezpz.toml at: {config_file}")
      return config_file

    pyproject_file = parent / "pyproject.toml"
    if pyproject_file.exists():
      try:
        with pyproject_file.open("r") as f:
          data = toml.load(f)
        if data.get("tool", {}).get("ezpz"):
          logger.debug(f"Found [tool.ezpz_pluginz] in pyproject.toml at: {pyproject_file}")
          return pyproject_file
      except Exception as e:
        logger.debug(f"Error checking pyproject.toml at {pyproject_file}: {e}")
        continue

  return None
