import logging
import importlib
import importlib.util
import importlib.metadata
from typing import Iterable
from pathlib import Path
from operator import attrgetter
from itertools import chain, groupby

from jinja2 import Template
from pydantic import BaseModel

from painlezz_basez.yamlable import BaseYamlModel
from ezpz_pluginz.modelz.toml_schema import EzpzPluginConfig
from ezpz_pluginz.macroz.register_plugin_macro import PolarsPluginMacroMetadataPD

logger = logging.getLogger(__name__)

EZPZ_TOML_FILENAME = "ezpz.toml"
EZPZ_LOCKFILE_FILENAME = "ezpz-lock.yaml"


def group_models_by_key[T: BaseModel](data: Iterable[T], key: str) -> dict[str, set[T]]:
  sorted_data = sorted(data, key=attrgetter(key))
  return {k: set(v) for k, v in groupby(sorted_data, key=attrgetter(key))}


class PolarsPluginLockfilePD(BaseYamlModel):
  project_plugins: dict[str, set[PolarsPluginMacroMetadataPD]]
  site_plugins: dict[str, set[PolarsPluginMacroMetadataPD]]

  @classmethod
  def generate(cls) -> "PolarsPluginLockfilePD":
    logger.debug(f"cwd: {Path.cwd()}")
    project_ezpz_toml_path = Path.cwd().joinpath(EZPZ_TOML_FILENAME)
    if not project_ezpz_toml_path.exists():
      return cls(project_plugins=dict[str, set[PolarsPluginMacroMetadataPD]](), site_plugins=dict[str, set[PolarsPluginMacroMetadataPD]]())
    project_entry = cls(project_plugins=EzpzPluginConfig.get_plugins(project_ezpz_toml_path), site_plugins={})
    for dist in importlib.metadata.distributions():
      if "ezpz-pluginz" in (dist.requires or []):
        spec = importlib.util.find_spec(dist.metadata["Name"])
        if spec and spec.origin:
          patch_file = Path(spec.origin).with_name(EZPZ_LOCKFILE_FILENAME)
          if patch_file.exists():
            project_entry.site_plugins.update(cls.from_yaml_file(patch_file).project_plugins)
    return project_entry

  def generate_registry(self) -> str:
    imports = list[str]()
    registry = list[str]()
    for plugin in chain(chain.from_iterable(self.project_plugins.values()), chain.from_iterable(self.site_plugins.values())):
      imports.append(plugin.import_)
      registry.append(plugin.registery_entry())
    return Template(Path(__file__).parent.parent.joinpath("templates", "sitecustomize.py.j2").read_text()).render(imports=imports, registry=registry)
