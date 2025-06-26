import os
import sys
import json
import time
import logging
import subprocess
from typing import Any
from pathlib import Path
from dataclasses import asdict, dataclass

import httpx
import typer

app = typer.Typer(name="ezplugins", pretty_exceptions_show_locals=False, pretty_exceptions_short=True)
logger = logging.getLogger(__name__)

DEFAULT_REGISTRY_URL = "https://registry.ezpz.dev"  # the registry
REGISTRY_URL = os.getenv("EZPZ_REGISTRY_URL", DEFAULT_REGISTRY_URL)
CACHE_DIR = Path.home() / ".ezpz" / "cache"
CACHE_EXPIRY_HOURS = 6


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


class PluginRegistry:
  """Registry for EZPZ ecosystem plugins."""

  def __init__(self) -> None:
    self._plugins: dict[str, PluginInfo] = {}
    self._api = PluginRegistryAPI()
    self._cache_dir = CACHE_DIR
    self._cache_dir.mkdir(parents=True, exist_ok=True)

    # Load in order of precedence
    self._load_cached_plugins()
    self._load_remote_plugins()
    self._load_site_plugins()

  def _get_cache_file(self) -> Path:
    """Get the cache file path."""
    return self._cache_dir / "registry_cache.json"

  def _is_cache_valid(self) -> bool:
    """Check if cache is still valid."""
    cache_file = self._get_cache_file()
    if not cache_file.exists():
      return False

    cache_age = time.time() - cache_file.stat().st_mtime
    return cache_age < (CACHE_EXPIRY_HOURS * 3600)

  def _save_cache(self, plugins: list[PluginInfo]) -> None:
    """Save plugins to cache."""
    try:
      cache_data = {"timestamp": time.time(), "plugins": [asdict(plugin) for plugin in plugins]}

      cache_file = self._get_cache_file()
      with Path.open(cache_file, "w") as f:
        json.dump(cache_data, f, indent=2)
    except Exception as e:
      logger.warning(f"Failed to save cache: {e}")

  def _load_cached_plugins(self) -> None:
    """Load plugins from cache if valid."""
    if not self._is_cache_valid():
      return

    try:
      cache_file = self._get_cache_file()
      with Path.open(cache_file, "r") as f:
        cache_data = json.load(f)

      for plugin_data in cache_data.get("plugins", []):
        plugin = PluginInfo(**plugin_data)
        self._register_plugin(plugin)

      logger.debug(f"Loaded {len(cache_data.get('plugins', []))} plugins from cache")
    except Exception as e:
      logger.warning(f"Failed to load cache: {e}")

  def _load_builtin_plugins(self) -> None:
    """Load builtin plugins that ship with ezpz_pluginz."""
    builtin_plugins = [
      PluginInfo(
        name="rust-ti",
        package_name="ezpz-rust-ti",
        description="Rust-powered technical analysis indicators for Polars",
        aliases=["ta", "technical-analysis"],
        author="Summit Sailors",
        homepage="https://github.com/Summit-Sailors/EZPZ",
      )
    ]
    for plugin in builtin_plugins:
      self._register_plugin(plugin)

  def _load_remote_plugins(self) -> None:
    if self._is_cache_valid():
      return

    logger.debug("Fetching plugins from remote registry...")
    remote_plugins = self._api.fetch_plugins()

    if remote_plugins:
      for plugin in remote_plugins:
        self._register_plugin(plugin)

      self._save_cache(remote_plugins)
      logger.debug(f"Loaded {len(remote_plugins)} plugins from remote registry")
    else:
      logger.warning("Failed to fetch from remote registry, using local data")

  def _load_site_plugins(self) -> None:
    """Load plugins from installed packages."""
    try:
      import importlib.metadata

      for dist in importlib.metadata.distributions():
        entry_points = dist.entry_points
        if hasattr(entry_points, "select"):
          ezpz_plugins = entry_points.select(group="ezpz.plugins")
        else:
          # Fallback for older versions
          ezpz_plugins = [ep for ep in entry_points if ep.group == "ezpz.plugins"]

        for entry_point in ezpz_plugins:
          try:
            plugin_info_func = entry_point.load()
            plugin_info_data = plugin_info_func()
            plugin_info = PluginInfo(**plugin_info_data) if isinstance(plugin_info_data, dict) else plugin_info_data
            self._register_plugin(plugin_info)
          except Exception as e:
            logger.warning(f"Failed to load plugin from {entry_point.name}: {e}")
    except ImportError:
      pass

  def _register_plugin(self, plugin: PluginInfo) -> None:
    self._plugins[plugin.name] = plugin
    # Also register aliases
    for alias in plugin.aliases:
      self._plugins[alias] = plugin

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
    # try local search
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

    if matching_plugins or self._is_cache_valid():
      return matching_plugins

    # try remote search otherwise
    remote_results = self._api.search_plugins(keyword)
    for plugin in remote_results:
      if plugin.name not in seen:
        matching_plugins.append(plugin)
        seen.add(plugin.name)

    return matching_plugins

  def refresh_cache(self) -> bool:
    try:
      cache_file = self._get_cache_file()
      if cache_file.exists():
        cache_file.unlink()

      self._plugins.clear()
      self._load_remote_plugins()
      self._load_site_plugins()

    except Exception as e:
      logger.exception(f"Failed to refresh cache: {e}")
      return False
    return True


def is_package_installed(package_name: str) -> bool:
  import importlib.metadata

  try:
    importlib.metadata.distribution(package_name)
  except importlib.metadata.PackageNotFoundError:
    return False
  return True


def detect_package_manager() -> tuple[list[str], str]:
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
      logger.exception("Exception occurred while checking for rye project files")

  # for rye-specific files
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

  # Check for available package managers
  for cmd, name in package_managers:
    if name in ("rye", "poetry", "pipenv", "conda", "mamba"):
      continue  # Already checked above

    if name == "uv" and _command_available("uv"):
      return (cmd, name)
    if name == "pip":
      return (cmd, name)  # pip is always available with Python

  # Fallback to pip
  return ([sys.executable, "-m", "pip", "install"], "pip")


def _command_available(command: str) -> bool:
  try:
    result = subprocess.run([command, "--version"], capture_output=True, text=True, timeout=5, check=False)
  except (subprocess.CalledProcessError, subprocess.TimeoutExpired, FileNotFoundError):
    return False
  return result.returncode == 0


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
  """Create a default ezpz.toml configuration file."""
  config_content = f"""[ezpz_pluginz]
name = "{project_name}"
include = [
    "src/",
    "*.py"
]
site_customize = true
"""
  Path("ezpz.toml").write_text(config_content)


@app.command(name="mount")
def mount() -> None:
  """Mount your plugins type hints"""
  from ezpz_pluginz import mount_plugins

  mount_plugins()


@app.command()
def unmount() -> None:
  """Unmount your plugins type hints"""
  from ezpz_pluginz import unmount_plugins

  unmount_plugins()


@app.command()
def register(
  plugin_name: str = typer.Argument(help="Name of the plugin to register"),
  package_name: str = typer.Option(..., "--package", help="PyPI package name"),
  description: str = typer.Option(..., "--description", help="Plugin description"),
  aliases: str = typer.Option("", "--aliases", help="Comma-separated aliases"),
  author: str = typer.Option("", "--author", help="Plugin author"),
  homepage: str = typer.Option("", "--homepage", help="Plugin homepage URL"),
  api_key: str | None = typer.Option(None, "--api-key", help="Registry API key"),
) -> None:
  """Register a plugin with the EZPZ registry."""

  if not api_key:
    api_key = os.getenv("EZPZ_REGISTRY_API_KEY")
    if not api_key:
      logger.error("API key required. Set EZPZ_REGISTRY_API_KEY or use --api-key")
      raise typer.Exit(1)

  plugin_info = PluginInfo(
    name=plugin_name,
    package_name=package_name,
    description=description,
    aliases=[a.strip() for a in aliases.split(",") if a.strip()],
    author=author or None,
    homepage=homepage or None,
  )

  api = PluginRegistryAPI()
  success = api.register_plugin(plugin_info, api_key)

  if success:
    logger.info(f"Successfully registered plugin '{plugin_name}' with EZPZ registry")
    logger.info("Plugin will be available to users within a few minutes")
  else:
    logger.error(f"Failed to register plugin '{plugin_name}'")
    raise typer.Exit(1)


@app.command()
def refresh() -> None:
  """Refresh the plugin registry cache."""
  logger.info("Refreshing plugin registry cache...")

  registry = PluginRegistry()
  if registry.refresh_cache():
    logger.info("Plugin registry cache refreshed successfully")
  else:
    logger.error("Failed to refresh plugin registry cache")
    raise typer.Exit(1)


@app.command()
def status() -> None:
  registry = PluginRegistry()
  cache_file = registry._get_cache_file()

  logger.info("EZPZ Plugin Registry Status:")
  logger.info("-" * 40)
  logger.info(f"Registry URL: {REGISTRY_URL}")
  logger.info(f"Cache directory: {registry._cache_dir}")

  if cache_file.exists():
    cache_age = time.time() - cache_file.stat().st_mtime
    hours_old = cache_age / 3600
    is_valid = registry._is_cache_valid()

    logger.info(f"Cache file: {cache_file}")
    logger.info(f"Cache age: {hours_old:.1f} hours")
    logger.info(f"Cache status: {'Valid' if is_valid else 'Expired'}")
  else:
    logger.info("Cache file: Not found")

  plugins = registry.list_plugins()
  logger.info(f"Total plugins available: {len(plugins)}")

  verified_count = sum(1 for p in plugins if p.verified)
  logger.info(f"Verified plugins: {verified_count}")


@app.command()
def add(
  plugin_name: str = typer.Argument(help="Name of the plugin to install"),
  auto_mount: bool = typer.Option(True, "--auto-mount/--no-auto-mount", help="Automatically mount plugins after installation"),
) -> None:
  registry = PluginRegistry()
  plugin = registry.get_plugin(plugin_name)

  if not plugin:
    logger.info(f"Plugin '{plugin_name}' not found in registry.")
    logger.info("Use 'ezplugins list' to see available plugins.")
    raise typer.Exit(1)

  logger.info(f"Installing {plugin.name} ({plugin.package_name})...")
  logger.info(f"Description: {plugin.description}")

  # Check if already installed
  if is_package_installed(plugin.package_name):
    logger.info(f"Package {plugin.package_name} is already installed")
  else:
    if not install_package(plugin.package_name):
      logger.info(f"Failed to install {plugin.package_name}")
      raise typer.Exit(1)
    logger.info(f"Successfully installed {plugin.package_name}")

  # Check for ezpz.toml and create if needed
  if not check_ezpz_config():
    if typer.confirm("No ezpz.toml found. Create default configuration?"):
      project_name = typer.prompt("Project name", default="my-ezpz-project")
      create_default_ezpz_config(project_name)
      logger.info("Created ezpz.toml configuration")
    elif auto_mount:
      logger.info("Cannot auto-mount without ezpz.toml")
      auto_mount = False

  # Auto-mount if requested
  if auto_mount:
    logger.info("Mounting plugins...")
    mount()

  logger.info(f"Plugin '{plugin.name}' is ready to use!")


@app.command(name="list")
def list_plugins() -> None:
  registry = PluginRegistry()
  plugins = registry.list_plugins()

  if not plugins:
    logger.info("No plugins found in registry.")
    logger.info("Try running 'ezplugins refresh' to update the cache.")
    return

  logger.info("Available EZPZ Plugins:")
  logger.info("-" * 50)

  for plugin in plugins:
    installed = "✓" if is_package_installed(plugin.package_name) else "○"
    verified = "🛡️" if plugin.verified else ""

    logger.info(f"{installed} {plugin.name} {verified}")
    logger.info(f"  Package: {plugin.package_name}")
    logger.info(f"  Description: {plugin.description}")
    if plugin.aliases:
      logger.info(f"  Aliases: {', '.join(plugin.aliases)}")
    if plugin.author:
      logger.info(f"  Author: {plugin.author}")
    if plugin.version:
      logger.info(f"  Version: {plugin.version}")


@app.command()
def find(
  keyword: str = typer.Argument(help="Keyword to search for in plugins"),
) -> None:
  registry = PluginRegistry()
  matching_plugins = registry.search_plugins(keyword)

  if not matching_plugins:
    logger.info(f"No plugins found matching '{keyword}'")
    return

  logger.info(f"Plugins matching '{keyword}':")
  logger.info("-" * 50)

  for plugin in matching_plugins:
    installed = "✓" if is_package_installed(plugin.package_name) else "○"
    logger.info(f"{installed} {plugin.name}")
    logger.info(f"  Package: {plugin.package_name}")
    logger.info(f"  Description: {plugin.description}")


if __name__ == "__main__":
  app()
