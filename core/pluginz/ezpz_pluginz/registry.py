import os
import sys
import json
import time
import logging
import tomllib
import subprocess
import importlib.metadata
from typing import Any
from pathlib import Path
from datetime import datetime
from dataclasses import asdict, dataclass
from urllib.parse import quote
from importlib.util import module_from_spec, spec_from_file_location

import httpx

logger = logging.getLogger(__name__)

DEFAULT_REGISTRY_URL = "http://localhost:8000"
REGISTRY_URL = os.getenv("EZPZ_REGISTRY_URL", DEFAULT_REGISTRY_URL)


LOCAL_REGISTRY_DIR = Path.home() / ".ezpz" / "registry"
LOCAL_REGISTRY_FILE = LOCAL_REGISTRY_DIR / "plugins.json"


@dataclass
class PluginInfo:
  name: str
  package_name: str
  description: str
  aliases: list[str]
  version: str | None = None
  author: str | None = None
  homepage: str | None = None
  verified: bool = False
  created_at: str | None = None
  updated_at: str | None = None


class PluginRegistryAPI:
  def __init__(self, base_url: str = REGISTRY_URL) -> None:
    self.base_url = base_url.rstrip("/")
    self.timeout = 30.0

  def _make_request(self, endpoint: str, method: str = "GET", data: dict[str, Any] | None = None) -> dict[str, Any]:
    url = f"{self.base_url}{endpoint}"
    try:
      with httpx.Client(timeout=self.timeout) as client:
        if method == "GET":
          response = client.get(url)
        elif method == "POST":
          response = client.post(url, json=data)
        elif method == "DELETE":
          response = client.delete(url)
        elif method == "PUT":
          response = client.put(url, json=data)
        else:
          raise ValueError(f"Unsupported HTTP method: {method}")

        response.raise_for_status()

        # empty responses
        if not response.content:
          logger.warning(f"Empty response from {url}")
          return {}

        try:
          return response.json()
        except json.JSONDecodeError as e:
          logger.warning(f"Invalid JSON response from {url}: {e}")
          return {}

    except (httpx.RequestError, httpx.HTTPStatusError, ValueError) as e:
      logger.warning(f"Registry API request failed for {url}: {e}")
      return {}

  def fetch_plugins(self) -> list[PluginInfo]:
    try:
      response = self._make_request("/api/v1/plugins")

      if not response:
        logger.warning("Invalid or empty response from plugins API")
        return []

      plugins_data: list[PluginInfo] = response.get("plugins", [])

      plugins = list[PluginInfo]()
      for plugin_data in plugins_data:
        if not isinstance(plugin_data, dict):
          logger.warning(f"Invalid plugin data format: {plugin_data}")
          continue

        plugin = safe_deserialize_plugin(plugin_data)
        if plugin:
          plugins.append(plugin)

      logger.debug(f"Successfully fetched {len(plugins)} plugins from registry")

    except Exception as e:
      logger.warning(f"Failed to fetch plugins from registry: {e}")
      return []
    return plugins

  def search_plugins(self, keyword: str) -> list[PluginInfo]:
    if not keyword:
      logger.warning("Invalid search keyword provided")
      return []

    try:
      encoded_keyword = quote(keyword)
      response = self._make_request(f"/api/v1/plugins/search?q={encoded_keyword}")

      if not response:
        logger.warning("Invalid or empty response from search API")
        return []

      plugins_data: list[PluginInfo] = response.get("plugins", [])

      plugins = list[PluginInfo]()
      for plugin_data in plugins_data:
        if not isinstance(plugin_data, dict):
          continue

        plugin = safe_deserialize_plugin(plugin_data)
        if plugin:
          plugins.append(plugin)

      logger.debug(f"Search returned {len(plugins)} plugins for keyword '{keyword}'")

    except Exception as e:
      logger.warning(f"Failed to search plugins: {e}")
      return []
    return plugins

  def register_plugin(self, plugin_info: PluginInfo, api_key: str | None) -> bool:
    # if not plugin_info or not api_key:
    #   logger.error("Plugin info and API key are required for registration")
    #   return False

    try:
      data = {"plugin": asdict(plugin_info)}
      # headers = {"Authorization": f"Bearer {api_key}"}

      logger.info(f"data: {data}")

      with httpx.Client(timeout=self.timeout) as client:
        response = client.post(f"{self.base_url}/api/v1/plugins/register", json=data)
        response.raise_for_status()

        if not response.content:
          return False

        result = response.json()
        return result.get("success", False)

    except Exception as e:
      logger.exception(f"Failed to register plugin: {e}")
      return False

  def update_plugin(self, plugin_info: PluginInfo, api_key: str) -> bool:
    if not plugin_info or not api_key:
      logger.error("Plugin info and API key are required for update")
      return False

    try:
      data = {"plugin": safe_serialize_plugin(plugin_info)}
      headers = {"Authorization": f"Bearer {api_key}"}

      with httpx.Client(timeout=self.timeout) as client:
        response = client.put(f"{self.base_url}/api/v1/plugins/{plugin_info.name}", json=data, headers=headers)
        response.raise_for_status()

        if not response.content:
          return False

        result = response.json()
        return result.get("success", False)

    except Exception as e:
      logger.exception(f"Failed to update plugin: {e}")
      return False

  def delete_plugin(self, plugin_name: str, api_key: str) -> bool:
    if not plugin_name or not api_key:
      logger.error("Plugin name and API key are required for deletion")
      return False

    try:
      headers = {"Authorization": f"Bearer {api_key}"}

      with httpx.Client(timeout=self.timeout) as client:
        response = client.delete(f"{self.base_url}/api/v1/plugins/{plugin_name}", headers=headers)
        response.raise_for_status()

        if not response.content:
          return False

        result = response.json()
        return result.get("success", False)

    except Exception as e:
      logger.exception(f"Failed to delete plugin: {e}")
      return False


class LocalPluginRegistry:
  """Local registry for EZPZ ecosystem plugins."""

  def __init__(self) -> None:
    self._plugins: dict[str, PluginInfo] = {}
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
          plugin = PluginInfo(**plugin_data)
          self._register_plugin(plugin)
      logger.debug(f"Loaded {len(data.get('plugins', []))} plugins from local registry")
    except Exception as e:
      logger.warning(f"Failed to load local registry: {e}")

  def _save_local_registry(self, plugins: list[PluginInfo]) -> None:
    try:
      registry_data = {"timestamp": time.time(), "plugins": [asdict(plugin) for plugin in plugins]}
      with LOCAL_REGISTRY_FILE.open("w") as f:
        json.dump(registry_data, f, indent=2)
      logger.debug(f"Saved {len(plugins)} plugins to local registry")
    except Exception as e:
      logger.warning(f"Failed to save local registry: {e}")

  def _register_plugin(self, plugin: PluginInfo) -> None:
    self._plugins[plugin.name.lower()] = plugin
    # Also register aliases
    for alias in plugin.aliases:
      self._plugins[alias.lower()] = plugin

  def fetch_and_update_registry(self) -> bool:
    logger.debug("Fetching plugins from remote registry...")
    try:
      remote_plugins = self._api.fetch_plugins()

      # Clear existing plugins and update with remote data
      self._plugins.clear()
      for plugin in remote_plugins:
        self._register_plugin(plugin)

      self._save_local_registry(remote_plugins)

      if len(remote_plugins) == 0:
        logger.info("Updated local registry - remote registry is empty")
      else:
        logger.info(f"Updated local registry with {len(remote_plugins)} plugins")

    except Exception as e:
      logger.warning(f"Failed to update registry: {e}")
      return False
    return True

  def get_plugin(self, name: str) -> PluginInfo | None:
    return self._plugins.get(name.lower())

  def list_plugins(self) -> list[PluginInfo]:
    seen: set[str] = set()
    unique_plugins: list[PluginInfo] = []

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

    except Exception as e:
      logger.warning(f"Error checking plugin registration for '{plugin_name}': {e}")
      return False
    return False

  def search_plugins(self, keyword: str) -> list[PluginInfo]:
    keyword_lower = keyword.lower()
    matching_plugins = list[PluginInfo]()
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


def discover_local_plugins() -> list[PluginInfo]:
  plugins = list[PluginInfo]()

  try:
    for dist in importlib.metadata.distributions():
      entry_points = dist.entry_points
      ezpz_plugins = entry_points.select(group="ezpz.plugins") if hasattr(entry_points, "select") else [ep for ep in entry_points if ep.group == "ezpz.plugins"]

      for entry_point in ezpz_plugins:
        try:
          plugin_info_func = entry_point.load()
          plugin_info_data = plugin_info_func()
          plugin_info = PluginInfo(**plugin_info_data) if isinstance(plugin_info_data, dict) else plugin_info_data
          plugins.append(plugin_info)
        except Exception as e:
          logger.warning(f"Failed to load plugin from {entry_point.name}: {e}")
  except ImportError:
    logger.debug("importlib.metadata not available")

  return plugins


def find_plugins_in_directory(group: str = "ezpz.plugins") -> list[PluginInfo]:
  plugins = list[PluginInfo]()

  try:
    eps = importlib.metadata.entry_points(group=group)
    for ep in eps:
      try:
        register_func = ep.load()
        plugin_data = register_func()

        if isinstance(plugin_data, dict):
          plugin = PluginInfo(**plugin_data)
          plugins.append(plugin)
        elif isinstance(plugin_data, PluginInfo):
          plugins.append(plugin_data)

      except Exception as e:
        logger.warning(f"Error loading plugin {ep.name}: {e}")

  except Exception as e:
    logger.warning(f"Error discovering entry points: {e}")

  return plugins


def load_ezpz_config() -> dict[str, Any]:
  config_file = Path("ezpz.toml")
  if not config_file.exists():
    return {}

  try:
    with config_file.open("rb") as f:
      return tomllib.load(f)
  except Exception as e:
    logger.warning(f"Failed to load ezpz.toml: {e}")
    return {}


def get_package_manager_from_config() -> str | None:
  config = load_ezpz_config()
  return config.get("ezpz_pluginz", {}).get("package_manager")


def is_package_installed(package_name: str) -> bool:
  try:
    importlib.metadata.distribution(package_name)
  except importlib.metadata.PackageNotFoundError:
    return False
  return True


def _command_available(command: str) -> bool:
  try:
    result = subprocess.run([command, "--version"], capture_output=True, text=True, timeout=5, check=False)
  except (subprocess.CalledProcessError, subprocess.TimeoutExpired, FileNotFoundError):
    return False
  return result.returncode == 0


def detect_package_manager() -> tuple[list[str], str]:
  config_manager = get_package_manager_from_config()
  if config_manager:
    if config_manager == "pip":
      return ([sys.executable, "-m", "pip", "install"], "pip")
    if config_manager == "uv" and _command_available("uv"):
      return (["uv", "pip", "install"], "uv")
    if config_manager == "rye" and _command_available("rye"):
      return (["rye", "add"], "rye")
    if config_manager == "poetry" and _command_available("poetry"):
      return (["poetry", "add"], "poetry")
    if config_manager == "pipenv" and _command_available("pipenv"):
      return (["pipenv", "install"], "pipenv")
    if config_manager == "conda" and _command_available("conda"):
      return (["conda", "install", "-c", "conda-forge"], "conda")
    if config_manager == "mamba" and _command_available("mamba"):
      return (["mamba", "install", "-c", "conda-forge"], "mamba")

  # auto-detect
  package_managers = [
    # uv
    (["uv", "pip", "install"], "uv"),
    # rye
    (["rye", "add"], "rye"),
    # poetry (if pyproject.toml with poetry config exists)
    (["poetry", "add"], "poetry"),
    # pipenv (if Pipfile exists)
    (["pipenv", "install"], "pipenv"),
    # conda/mamba (if in conda environment)
    (["conda", "install", "-c", "conda-forge"], "conda"),
    (["mamba", "install", "-c", "conda-forge"], "mamba"),
    # pip (fallback)
    ([sys.executable, "-m", "pip", "install"], "pip"),
  ]

  # project-specific indicators
  if Path("pyproject.toml").exists():
    try:
      content = Path("pyproject.toml").read_text()
      # rye project
      if "[tool.rye" in content or ("[project]" in content and "rye" in content):
        if _command_available("rye"):
          return (["rye", "add"], "rye")
      # poetry project
      elif "[tool.poetry" in content and _command_available("poetry"):
        return (["poetry", "add"], "poetry")
    except Exception:
      logger.exception("Exception occurred while checking pyproject.toml")

  # rye-specific files
  if Path(".python-version").exists() and _command_available("rye"):
    try:
      if Path("requirements.lock").exists() or Path("requirements-dev.lock").exists():
        return (["rye", "add"], "rye")
    except Exception:
      logger.exception("Exception occurred while checking for rye project files")

  if Path("Pipfile").exists() and _command_available("pipenv"):
    return (["pipenv", "install"], "pipenv")

  # conda environment
  if "CONDA_DEFAULT_ENV" in os.environ or "CONDA_PREFIX" in os.environ:
    if _command_available("mamba"):
      return (["mamba", "install", "-c", "conda-forge"], "mamba")
    if _command_available("conda"):
      return (["conda", "install", "-c", "conda-forge"], "conda")

  for cmd, name in package_managers:
    if name in ("rye", "poetry", "pipenv", "conda", "mamba"):
      continue  # already checked above
    if name == "uv" and _command_available("uv"):
      return (cmd, name)
    if name == "pip":
      return (cmd, name)

  # pip as a fallback
  return ([sys.executable, "-m", "pip", "install"], "pip")


def install_package(package_name: str) -> bool:
  cmd_base, manager_name = detect_package_manager()
  cmd = [*cmd_base, package_name]

  logger.info(f"Installing {package_name} using {manager_name}...")
  logger.info(f"Command: {' '.join(cmd)}")

  try:
    subprocess.run(cmd, capture_output=True, text=True, check=True)
    logger.info(f"Installation completed successfully with {manager_name}")
  except subprocess.CalledProcessError as e:
    logger.exception(f"Failed to install {package_name} using {manager_name}")
    logger.exception(f"Error output: {e.stderr}")

    if manager_name != "pip":
      logger.info("Falling back to pip...")
      try:
        pip_cmd = [sys.executable, "-m", "pip", "install", package_name]
        subprocess.run(pip_cmd, capture_output=True, text=True, check=True)
        logger.info("Installation completed successfully with pip (fallback)")
      except subprocess.CalledProcessError as fallback_e:
        logger.exception(f"Pip fallback also failed: {fallback_e.stderr}")
        return False
    else:
      return False
  except FileNotFoundError:
    logger.exception(f"Package manager '{manager_name}' not found")
    return False

  return True


def check_ezpz_config() -> bool:
  return Path("ezpz.toml").exists()


def create_default_ezpz_config(project_name: str = "my-ezpz-project") -> None:
  config_content = f"""[ezpz_pluginz]
name = "{project_name}"
include = [
    "src/",
    "*.py"
]
site_customize = true
package_manager = "pip"  # Options: pip, uv, rye, poetry, pipenv, conda, mamba
"""
  Path("ezpz.toml").write_text(config_content)


def setup_local_registry() -> None:
  registry = LocalPluginRegistry()
  success = registry.fetch_and_update_registry()
  if success:
    logger.info("Local registry setup completed successfully")
  else:
    logger.warning("Failed to setup local registry from remote")


def find_plugin_in_path(plugin_path: str, include_paths: list[str]) -> PluginInfo | None:
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


def _load_plugin_from_path(plugin_path: Path) -> PluginInfo | None:
  try:
    # Common patterns for plugin entry points
    entry_point_patterns = [
      # plugins/plugin-name/python/package_name/__init__.py
      plugin_path / "python" / _extract_package_name(plugin_path.name) / "__init__.py",
      # plugins/plugin-name/src/package_name/__init__.py
      plugin_path / "src" / _extract_package_name(plugin_path.name) / "__init__.py",
      # plugins/plugin-name/package_name/__init__.py
      plugin_path / _extract_package_name(plugin_path.name) / "__init__.py",
      # plugins/plugin-name/__init__.py
      plugin_path / "__init__.py",
    ]

    logger.debug(f"Checking entry point patterns: {[str(p) for p in entry_point_patterns]}")

    for entry_point_path in entry_point_patterns:
      if entry_point_path.exists():
        logger.debug(f"Found entry point: {entry_point_path}")
        plugin_info = _load_plugin_from_file(entry_point_path)
        if plugin_info:
          return plugin_info

    # If no standard patterns work, search recursively for __init__.py files
    # that contain register_plugin function
    logger.debug(f"Searching recursively in {plugin_path}")
    for init_file in plugin_path.rglob("__init__.py"):
      logger.debug(f"Trying {init_file}")
      plugin_info = _load_plugin_from_file(init_file)
      if plugin_info:
        return plugin_info

  except Exception as e:
    logger.warning(f"Error loading plugin from {plugin_path}: {e}")

  return None


def _extract_package_name(plugin_dir_name: str) -> str:
  return plugin_dir_name.replace("-", "_")


def _load_plugin_from_file(file_path: Path) -> PluginInfo | None:
  try:
    parent_dir = str(file_path.parent)
    if parent_dir not in sys.path:
      sys.path.insert(0, parent_dir)
      path_added = True
    else:
      path_added = False

    try:
      spec = spec_from_file_location("temp_plugin_module", file_path)
      if spec is None or spec.loader is None:
        return None

      module = module_from_spec(spec)
      spec.loader.exec_module(module)

      if hasattr(module, "register_plugin"):
        register_func = module.register_plugin
        plugin_data = register_func()

        if isinstance(plugin_data, dict):
          return PluginInfo(**plugin_data)
        if isinstance(plugin_data, PluginInfo):
          return plugin_data
    finally:
      # Clean up sys.path
      if path_added:
        sys.path.remove(parent_dir)

  except Exception as e:
    logger.debug(f"Could not load plugin from {file_path}: {e}")

  return None


def safe_deserialize_plugin(plugin_data: dict[str, Any]) -> PluginInfo | None:
  try:
    plugin_data.setdefault("name", "")
    plugin_data.setdefault("package_name", "")
    plugin_data.setdefault("description", "")
    plugin_data.setdefault("aliases", [])

    if not isinstance(plugin_data.get("aliases"), list):
      plugin_data["aliases"] = []

    if not plugin_data.get("name") or not plugin_data.get("package_name"):
      logger.warning(f"Plugin data missing required fields: {plugin_data}")
      return None

    return PluginInfo(**plugin_data)
  except (TypeError, ValueError) as e:
    logger.warning(f"Failed to deserialize plugin data {plugin_data}: {e}")
    return None


def safe_serialize_plugin(plugin: PluginInfo) -> dict[str, Any]:
  plugin_dict = asdict(plugin)

  for field in ["created_at", "updated_at"]:
    value = plugin_dict.get(field)
    if value is not None:
      if isinstance(value, datetime):
        plugin_dict[field] = value.isoformat()
      elif not isinstance(value, str):
        plugin_dict[field] = str(value)

  if not isinstance(plugin_dict.get("aliases"), list):
    plugin_dict["aliases"] = []

  return plugin_dict


def register_plugin() -> dict[str, Any]:
  """
  Plugin developers should implement this function in their package
  and register it as an entry point under 'ezpz.plugins' group.

  This is a template function that plugin developers should copy
  and modify for their specific plugin.

  # Returns:
      dict containing plugin information that will be converted to PluginInfo

  **Example usage in plugin developer's setup.py or pyproject.toml:**

  # setup.py
  ```python
      setup(
          name="my-ezpz-plugin",
          entry_points={
              "ezpz.plugins": [
                  "my-plugin = my_plugin:register_plugin",
              ],
          },
      )
  ```

  # pyproject.toml
  ```toml
      [project.entry-points."ezpz.plugins"]
      my-plugin = "my_plugin:register_plugin"
  ```
  """
  return {
    "name": "example-plugin",
    "package_name": "ezpz-example-plugin",
    "description": "An example EZPZ plugin",
    "aliases": ["example", "demo"],
    "version": "1.0.0",
    "author": "Plugin Developer",
    "homepage": "https://github.com/developer/ezpz-example-plugin",
  }
