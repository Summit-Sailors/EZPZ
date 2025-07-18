import importlib.util
import importlib.metadata
from typing import TYPE_CHECKING, Optional
from pathlib import Path

from ezpz_pluginz.logger import setup_logger
from ezpz_pluginz.registry.reg.local import LocalPluginRegistry

if TYPE_CHECKING:
  from ezpz_pluginz.registry.models import PluginMetadata

logger = setup_logger("Utils")


def is_package_installed(package_name: str) -> bool:
  try:
    importlib.metadata.distribution(package_name)
  except importlib.metadata.PackageNotFoundError:
    return False
  return True


def setup_local_registry() -> None:
  registry = LocalPluginRegistry()
  success = registry.fetch_and_update_registry()
  if success:
    logger.info("Local registry setup completed successfully")
  else:
    logger.warning("Failed to setup local registry from remote")


def find_plugin_in_path(plugin_path: str, include_paths: list[str]) -> Optional["PluginMetadata"]:
  plugin_path_obj = Path(plugin_path)
  logger.info(f"Searching for plugin in: {plugin_path_obj}")
  if plugin_path_obj.exists():
    plugin_info = _load_plugin_from_path(plugin_path_obj)
    if plugin_info:
      return plugin_info
  for include_path in include_paths:
    search_path = Path(include_path)
    full_path = search_path / plugin_path
    if full_path.exists():
      plugin_info = _load_plugin_from_path(full_path)
      if plugin_info:
        return plugin_info
    if search_path.exists():
      for subdir in search_path.iterdir():
        if subdir.is_dir() and subdir.name == plugin_path:
          plugin_info = _load_plugin_from_path(subdir)
          if plugin_info:
            return plugin_info
  return None


def _load_plugin_from_path(plugin_path: Path) -> Optional["PluginMetadata"]:
  try:
    entry_point_patterns = [
      plugin_path / "python" / _extract_package_name(plugin_path.name) / "__init__.py",
      plugin_path / "src" / _extract_package_name(plugin_path.name) / "__init__.py",
      plugin_path / _extract_package_name(plugin_path.name) / "__init__.py",
      plugin_path / "__init__.py",
    ]
    logger.debug(f"Checking entry point patterns: {[str(p) for p in entry_point_patterns]}")
    for entry_point_path in entry_point_patterns:
      if entry_point_path.exists():
        logger.debug(f"Found entry point: {entry_point_path}")
        plugin_info = _load_plugin_from_file(entry_point_path)
        if plugin_info:
          return plugin_info
    logger.debug(f"Searching recursively in {plugin_path}")
    for init_file in plugin_path.rglob("__init__.py"):
      logger.debug(f"Trying {init_file}")
      plugin_info = _load_plugin_from_file(init_file)
      if plugin_info:
        return plugin_info
  except Exception:
    logger.warning(f"Error loading plugin from {plugin_path}")
  return None


def _extract_package_name(plugin_dir_name: str) -> str:
  return plugin_dir_name.replace("-", "_")


def _load_plugin_from_file(file_path: Path) -> Optional["PluginMetadata"]:
  try:
    if not file_path.exists():
      logger.warning(f"Plugin file does not exist: {file_path}")
      return None
    spec = importlib.util.spec_from_file_location(f"plugin_{file_path.stem}", file_path)
    if spec is None or spec.loader is None:
      logger.warning(f"Could not create spec for {file_path}")
      return None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    if hasattr(module, "register_plugin"):
      register_func = module.register_plugin
      plugin_data: PluginMetadata = register_func()
      return plugin_data
    logger.warning(f"No register_plugin function in {file_path}")
  except Exception as e:
    logger.error(f"Failed to load plugin {file_path}: {e}", exc_info=True)
    return None
