import json
import time
import importlib.metadata
from typing import Any
from dataclasses import asdict

from ezpz_pluginz.logger import setup_logger
from ezpz_pluginz.registry.config import LOCAL_REGISTRY_DIR, LOCAL_REGISTRY_FILE
from ezpz_pluginz.registry.models import PluginResponse
from ezpz_pluginz.registry.reg.remote import PluginRegistryAPI

logger = setup_logger("Registry")


class LocalPluginRegistry:
  def __init__(self) -> None:
    self._plugins: dict[str, PluginResponse] = {}
    self._api = PluginRegistryAPI()
    self._ensure_registry_dir()
    self._load_local_registry()

  def _ensure_registry_dir(self) -> None:
    LOCAL_REGISTRY_DIR.mkdir(parents=True, exist_ok=True)

  def _load_local_registry(self) -> None:
    if not LOCAL_REGISTRY_FILE.exists():
      return

    try:
      with LOCAL_REGISTRY_FILE.open("r") as f:
        data = json.load(f)
        for plugin_data in data.get("plugins", []):
          plugin = PluginResponse(**plugin_data)
          self._register_plugin(plugin)
      logger.debug(f"Loaded {len(data.get('plugins', []))} plugins from local registry")
    except Exception:
      logger.warning("Failed to load local registry")

  def _save_local_registry(self, plugins: list[PluginResponse]) -> None:
    try:
      registry_data = {"timestamp": time.time(), "plugins": [asdict(plugin) for plugin in plugins]}
      with LOCAL_REGISTRY_FILE.open("w") as f:
        json.dump(registry_data, f, indent=2)
      logger.debug(f"Saved {len(plugins)} plugins to local registry")
    except Exception:
      logger.warning("Failed to save local registry")

  def _register_plugin(self, plugin: PluginResponse) -> None:
    self._plugins[plugin.name.lower()] = plugin
    # Also register aliases
    for alias in plugin.aliases:
      self._plugins[alias.lower()] = plugin

  def fetch_and_update_registry(self) -> bool:
    logger.debug("Fetching plugins from remote registry...")
    try:
      remote_plugins = self._api.fetch_plugins()
      if remote_plugins:
        self._plugins.clear()
        for plugin in remote_plugins:
          self._register_plugin(plugin)

        self._save_local_registry(remote_plugins)

        logger.info(f"Updated local registry with {len(remote_plugins)} plugins")

    except Exception:
      logger.warning("Failed to update registry")
      return False
    return True

  def get_plugin(self, name: str) -> PluginResponse | None:
    return self._plugins.get(name.lower())

  def list_plugins(self) -> list[PluginResponse]:
    seen: set[str] = set()
    unique_plugins: list[PluginResponse] = []

    for plugin in self._plugins.values():
      if plugin.name not in seen:
        unique_plugins.append(plugin)
        seen.add(plugin.name)

    return unique_plugins

  def is_plugin_registered(self, plugin_name: str) -> bool:
    try:
      plugin_name_lower = plugin_name.lower()

      if plugin_name_lower in self._plugins:
        return True

      # check if it exists as an alias or package name
      for plugin in self.list_plugins():  # list_plugins to get unique plugins
        if (
          plugin.name.lower() == plugin_name_lower
          or plugin.package_name.lower() == plugin_name_lower
          or plugin_name_lower in [alias.lower() for alias in plugin.aliases]
        ):
          return True

    except Exception:
      logger.warning(f"Error checking plugin registration for '{plugin_name}'")
      return False
    return False

  def search_plugins(self, keyword: str) -> list[PluginResponse]:
    keyword_lower = keyword.lower()
    matching_plugins = list[PluginResponse]()
    seen: set[str] = set()

    for plugin in self._plugins.values():
      if plugin.name in seen:
        continue

      search_fields = [
        plugin.name.lower(),
        plugin.description.lower(),
        plugin.author.lower() if plugin.author else "",
        *[alias.lower() for alias in plugin.aliases],
      ]

      if any(keyword_lower in field for field in search_fields):
        matching_plugins.append(plugin)
        seen.add(plugin.name)

    return matching_plugins


def discover_local_plugins() -> list[PluginResponse]:
  plugins = list[PluginResponse]()

  try:
    for dist in importlib.metadata.distributions():
      entry_points = dist.entry_points
      ezpz_plugins = entry_points.select(group="ezpz.plugins") if hasattr(entry_points, "select") else [ep for ep in entry_points if ep.group == "ezpz.plugins"]

      for entry_point in ezpz_plugins:
        try:
          plugin_info_func = entry_point.load()
          plugin_info_data: dict[str, Any] = plugin_info_func()
          plugin_info = PluginResponse(**plugin_info_data)
          plugins.append(plugin_info)
        except Exception:
          logger.warning(f"Failed to load plugin from {entry_point.name}")
  except ImportError:
    logger.debug("importlib.metadata not available")

  return plugins
