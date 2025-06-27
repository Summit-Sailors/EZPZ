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
from dataclasses import asdict, dataclass
from importlib.util import module_from_spec, spec_from_file_location

import httpx
import typer

app = typer.Typer(name="ezplugins", pretty_exceptions_show_locals=False, pretty_exceptions_short=True)
logger = logging.getLogger(__name__)

DEFAULT_REGISTRY_URL = "http://localhost:8080"
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
        return response.json()
    except (httpx.RequestError, httpx.HTTPStatusError, ValueError) as e:
      logger.warning(f"Registry API request failed: {e}")
      return {}

  def fetch_plugins(self) -> list[PluginInfo]:
    try:
      response = self._make_request("/api/v1/plugins")
      plugins = list[PluginInfo]()
      for plugin_data in response.get("plugins", []):
        plugins.append(PluginInfo(**plugin_data))
    except Exception as e:
      logger.warning(f"Failed to fetch plugins from registry: {e}")
      return []
    return plugins

  def search_plugins(self, keyword: str) -> list[PluginInfo]:
    try:
      response = self._make_request(f"/api/v1/plugins/search?q={keyword}")
      plugins = list[PluginInfo]()
      for plugin_data in response.get("plugins", []):
        plugins.append(PluginInfo(**plugin_data))
    except Exception as e:
      logger.warning(f"Failed to search plugins: {e}")
      return []
    return plugins

  def register_plugin(self, plugin_info: PluginInfo, api_key: str) -> bool:
    try:
      data = {"plugin": asdict(plugin_info)}
      with httpx.Client(timeout=self.timeout) as client:
        response = client.post(f"{self.base_url}/api/v1/plugins/register", json=data, headers={"Authorization": f"Bearer {api_key}"})
        response.raise_for_status()
        result = response.json()
        return result.get("success", False)
    except Exception as e:
      logger.exception(f"Failed to register plugin: {e}")
      return False

  def update_plugin(self, plugin_info: PluginInfo, api_key: str) -> bool:
    try:
      data = {"plugin": asdict(plugin_info)}
      with httpx.Client(timeout=self.timeout) as client:
        response = client.put(f"{self.base_url}/api/v1/plugins/{plugin_info.name}", json=data, headers={"Authorization": f"Bearer {api_key}"})
        response.raise_for_status()
        result = response.json()
        return result.get("success", False)
    except Exception as e:
      logger.exception(f"Failed to update plugin: {e}")
      return False

  def delete_plugin(self, plugin_name: str, api_key: str) -> bool:
    try:
      with httpx.Client(timeout=self.timeout) as client:
        response = client.delete(f"{self.base_url}/api/v1/plugins/{plugin_name}", headers={"Authorization": f"Bearer {api_key}"})
        response.raise_for_status()
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
    remote_plugins = self._api.fetch_plugins()

    if remote_plugins:
      self._plugins.clear()
      for plugin in remote_plugins:
        self._register_plugin(plugin)

      self._save_local_registry(remote_plugins)
      logger.info(f"Updated local registry with {len(remote_plugins)} plugins")
      return True
    logger.warning("Failed to fetch from remote registry")
    return False

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


def find_plugins_in_directory(directory: Path) -> list[PluginInfo]:
  plugins = list[PluginInfo]()

  if not directory.exists():
    return plugins

  for python_file in directory.rglob("*.py"):
    if python_file.name.startswith("__") and python_file.name.endswith("__.py"):
      continue

    try:
      spec = spec_from_file_location(python_file.stem, python_file)
      if spec and spec.loader:
        module = module_from_spec(spec)
        spec.loader.exec_module(module)

        if hasattr(module, "register_plugin"):
          plugin_data = module.register_plugin()
          if isinstance(plugin_data, dict):
            plugin = PluginInfo(**plugin_data)
            plugins.append(plugin)
          elif isinstance(plugin_data, PluginInfo):
            plugins.append(plugin_data)
    except Exception as e:
      logger.warning(f"Error loading plugin from {python_file}: {e}")

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
