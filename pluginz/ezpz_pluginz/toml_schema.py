import logging
from typing import TYPE_CHECKING, Any, Iterable, Generator
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
EZPZ_LOCKFILE_FILENAME = "ezpz-lock.yaml"


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
  name: str
  include: list[Path]
  site_customize: bool | None = Field(default=None)

  @staticmethod
  def from_toml_path(path: Path) -> "EzpzPluginConfig":
    return EzpzPluginToml(**toml.loads(path.read_text())).ezpz_pluginz

  @staticmethod
  def get_plugins(project_toml_path: Path) -> dict[str, set["PolarsPluginMacroMetadataPD"]]:
    ezpz_pluginz = EzpzPluginConfig.from_toml_path(project_toml_path)
    return group_models_by_key(set(process_includes(ezpz_pluginz.include)), "polars_ns")


class EzpzPluginToml(BaseModel):
  ezpz_pluginz: EzpzPluginConfig
