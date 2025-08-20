import logging
import importlib
import contextlib
import importlib.util
import importlib.metadata
from typing import Self, Iterable
from pathlib import Path
from operator import attrgetter
from itertools import chain, groupby

import yaml
from jinja2 import Template
from pydantic import BaseModel

from ezpz_pluginz.toml_schema import EzpzPluginConfig
from ezpz_pluginz.register_plugin_macro import PolarsPluginMacroMetadataPD

logger = logging.getLogger(__name__)

EZPZ_TOML_FILENAME = "ezpz.toml"
EZPZ_PROJECT_LOCKFILE_FILENAME = "ezpz-lock.yaml"
EZPZ_PLUGIN_LOCKFILE_FILENAME = "ezpz-lock.yml"


def group_models_by_key[T: BaseModel](data: Iterable[T], key: str) -> dict[str, set[T]]:
  sorted_data = sorted(data, key=attrgetter(key))
  return {k: set(v) for k, v in groupby(sorted_data, key=attrgetter(key))}


class PolarsPluginLockfilePD(BaseModel):
  project_plugins: dict[str, set[PolarsPluginMacroMetadataPD]]
  site_plugins: dict[str, set[PolarsPluginMacroMetadataPD]]

  @classmethod
  def generate(cls) -> "PolarsPluginLockfilePD":
    logger.debug(f"cwd: {Path.cwd()}")

    # Initialize empty project and site plugins
    project_plugins = dict[str, set[PolarsPluginMacroMetadataPD]]()
    site_plugins = dict[str, set[PolarsPluginMacroMetadataPD]]()
    project_entry = cls(project_plugins=project_plugins, site_plugins=site_plugins)

    # Try to load project plugins from ezpz.toml or pyproject.toml
    project_ezpz_toml_path = Path.cwd().joinpath(EZPZ_TOML_FILENAME)
    pyproject_toml_path = Path.cwd().joinpath("pyproject.toml")

    try:
      if project_ezpz_toml_path.exists():
        project_entry.project_plugins = EzpzPluginConfig.get_plugins(project_ezpz_toml_path)
        logger.debug(f"Loaded plugins from {EZPZ_TOML_FILENAME}")
      elif pyproject_toml_path.exists():
        project_entry.project_plugins = EzpzPluginConfig.get_plugins(pyproject_toml_path)
        logger.debug("Loaded plugins from pyproject.toml")
      else:
        logger.info("No local config found. Checking for remote plugins only.")
    except ValueError as e:
      logger.warning(f"Failed to load plugins: {e}. Continuing with empty project plugins.")

    logger.info("Proceeding to check for site packages")

    # track processed plugin lockfiles to avoid duplicates
    processed_lockfiles: set[Path] = set()
    has_ezpz_pluginz_dep = False

    for dist in importlib.metadata.distributions():
      if "ezpz-pluginz" in (dist.requires or []):
        has_ezpz_pluginz_dep = True
        spec = importlib.util.find_spec(dist.metadata["Name"].replace("-", "_"))
        if spec and spec.origin:
          patch_file = Path(spec.origin).with_name(EZPZ_PLUGIN_LOCKFILE_FILENAME)

          if patch_file.exists() and patch_file not in processed_lockfiles:
            try:
              site_plugin_data = cls.from_yaml_file(patch_file)

              for ns, plugins in site_plugin_data.project_plugins.items():
                if ns not in project_entry.project_plugins:
                  if ns not in project_entry.site_plugins:
                    project_entry.site_plugins[ns] = set()
                  project_entry.site_plugins[ns].update(plugins)
                else:
                  logger.debug(f"Skipping site plugins for {ns} - already loaded as project plugins")

              processed_lockfiles.add(patch_file)
              logger.debug(f"Loaded site plugins from {patch_file}")
            except Exception as e:
              logger.warning(f"Failed to load site plugins from {patch_file}: {e}")

    if not project_entry.project_plugins and not project_entry.site_plugins:
      if not has_ezpz_pluginz_dep:
        logger.error("No plugins found and no distributions depend on ezpz-pluginz.")
        msg = "No plugins or ezpz-pluginz dependencies found."
        raise ValueError(msg)
      logger.warning("Found ezpz-pluginz dependencies but no plugin lockfiles were loaded.")

    return project_entry

  def generate_registry(self) -> str:
    imports = list[str]()
    registry = list[str]()
    for plugin in chain(chain.from_iterable(self.project_plugins.values()), chain.from_iterable(self.site_plugins.values())):
      imports.append(plugin.import_)
      registry.append(plugin.registery_entry())
    return Template(Path(__file__).parent.joinpath("templates", "sitecustomize.py.j2").read_text()).render(imports=imports, registry=registry)

  def to_yaml(self) -> str:
    return yaml.safe_dump(self.model_dump(mode="json"), sort_keys=False)

  @classmethod
  def from_yaml(cls, content: str) -> Self:
    return cls.model_validate(yaml.safe_load(content))

  @classmethod
  def from_yaml_file(cls, lockfile_path: "Path") -> Self:
    return cls.from_yaml(lockfile_path.read_text())

  def to_yaml_file(self, lockfile_path: "Path") -> None:
    lockfile_path.write_text(self.to_yaml())

  def generate_and_save_plugin_lockfiles(self) -> None:
    if not self.project_plugins:
      logger.debug("No project plugins found, skipping plugin-level lock file generation")
      return

    for dist in importlib.metadata.distributions():
      if "ezpz-pluginz" in (dist.requires or []):
        spec = importlib.util.find_spec(dist.metadata["Name"].replace("-", "_"))
        if spec and spec.origin:
          plugin_module_path = Path(spec.origin)
          plugin_lockfile_path = plugin_module_path.with_name(EZPZ_PLUGIN_LOCKFILE_FILENAME)

          try:
            # plugins specific to this distribution/package
            plugin_specific_plugins = self._get_plugins_for_package(plugin_module_path.parent)

            if plugin_specific_plugins:
              plugin_lockfile_data = PolarsPluginLockfilePD(
                project_plugins=plugin_specific_plugins,
                site_plugins={},
              )

              plugin_lockfile_data.to_yaml_file(plugin_lockfile_path)
              logger.info(f"Generated plugin-level lock file: {plugin_lockfile_path}")
            else:
              logger.debug(f"No plugins found for package at {plugin_module_path.parent}")
          except Exception as e:
            logger.warning(f"Failed to generate plugin-level lock file at {plugin_lockfile_path}: {e}")

  def _get_plugins_for_package(self, package_path: Path) -> dict[str, set[PolarsPluginMacroMetadataPD]]:
    package_plugins: dict[str, set[PolarsPluginMacroMetadataPD]] = {}
    package_name = package_path.name

    for polars_ns, plugins in self.project_plugins.items():
      matching_plugins: set[PolarsPluginMacroMetadataPD] = set()

      for plugin in plugins:
        if self._plugin_belongs_to_package(plugin, package_name, package_path):
          matching_plugins.add(plugin)

      if matching_plugins:
        package_plugins[polars_ns] = matching_plugins

    return package_plugins

  def _plugin_belongs_to_package(self, plugin: PolarsPluginMacroMetadataPD, package_name: str, package_path: Path) -> bool:
    import_statement = plugin.import_

    # The module name from import statement
    if import_statement.startswith("from "):
      try:
        module_part = import_statement.split(" import ")[0].replace("from ", "").strip()

        if module_part.startswith(package_name):
          return True

        # Try to resolve the actual module path to be more accurate
        try:
          spec = importlib.util.find_spec(module_part)
          if spec and spec.origin:
            module_file_path = Path(spec.origin)
            # module file is within the package directory ?
            try:
              module_file_path.relative_to(package_path)
            except ValueError:
              contextlib.suppress(ValueError)
        except (ImportError, ModuleNotFoundError, ValueError):
          # If we can't resolve the module, fall back to string matching
          pass
        else:
          return True

      except (IndexError, ValueError):
        logger.warning(f"Could not parse import statement: {import_statement}")

    return False
