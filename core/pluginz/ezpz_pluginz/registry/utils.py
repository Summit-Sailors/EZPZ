import os
import sys
import subprocess
import importlib.util
import importlib.metadata
from typing import Any
from pathlib import Path

from ezpz_pluginz.logger import setup_logger
from ezpz_pluginz.registry.config import get_package_manager_from_config
from ezpz_pluginz.registry.models import PluginCreate
from ezpz_pluginz.registry.reg.local import LocalPluginRegistry

logger = setup_logger("Utils")


def is_package_installed(package_name: str) -> bool:
  try:
    importlib.metadata.distribution(package_name)
  except importlib.metadata.PackageNotFoundError:
    return False
  return True


def _command_available(command: str) -> bool:
  try:
    result = subprocess.run([command, "--version"], capture_output=True, text=True, timeout=5, check=False)  # noqa: S603
  except (subprocess.CalledProcessError, subprocess.TimeoutExpired, FileNotFoundError):
    return False
  return result.returncode == 0


def detect_package_manager() -> tuple[list[str], str]:  # noqa: PLR0911
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
    subprocess.run(cmd, capture_output=True, text=True, check=True)  # noqa: S603
    logger.info(f"Installation completed successfully with {manager_name}")
  except subprocess.CalledProcessError as e:
    logger.exception(f"Failed to install {package_name} using {manager_name}")
    logger.exception(f"Error output: {e.stderr}")

    if manager_name != "pip":
      logger.info("Falling back to pip...")
      try:
        pip_cmd = [sys.executable, "-m", "pip", "install", package_name]
        subprocess.run(pip_cmd, capture_output=True, text=True, check=True)  # noqa: S603
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


def setup_local_registry() -> None:
  registry = LocalPluginRegistry()
  success = registry.fetch_and_update_registry()
  if success:
    logger.info("Local registry setup completed successfully")
  else:
    logger.warning("Failed to setup local registry from remote")


def find_plugin_in_path(plugin_path: str, include_paths: list[str]) -> PluginCreate | None:
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


def _load_plugin_from_path(plugin_path: Path) -> PluginCreate | None:
  try:
    # Common patterns for plugin entry points
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

    # If no standard patterns work, search recursively for __init__.py files
    # that contain register_plugin function
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


def _load_plugin_from_file(file_path: Path) -> "PluginCreate | None":
  try:
    if not file_path.exists():
      logger.warning(f"Plugin file does not exist: {file_path}")
      return None

    # spec directly from file path
    spec = importlib.util.spec_from_file_location(f"plugin_{file_path.stem}", file_path)

    if spec is None or spec.loader is None:
      logger.warning(f"Could not create spec for {file_path}")
      return None

    module = importlib.util.module_from_spec(spec)

    spec.loader.exec_module(module)

    if hasattr(module, "register_plugin"):
      register_func = module.register_plugin
      plugin_data = register_func()

      return PluginCreate(**plugin_data)
    logger.warning(f"No register_plugin function in {file_path}")
  except Exception as e:
    logger.error(f"Failed to load plugin {file_path}: {e}", exc_info=True)
    return None


def register_plugin() -> dict[str, Any]:
  """
  Plugin developers should implement this function in their package
  and register it as an entry point under 'ezpz.plugins' group.

  This is a template function that plugin developers should copy
  and modify for their specific plugin.

  # Returns:
      dict containing plugin information that will be converted to PluginCreate

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
    "name": "rust-ti",
    "package_name": "ezpz-rust-ti",
    "description": "Rust-powered technical analysis indicators for Polars DataFrames",
    "aliases": ["ta", "technical-analysis", "indicators"],
    "version": "0.1.0",
    "author": "Summit Sailors",
    "category": "Technical analysis",
    "homepage": "https://github.com/Summit-Sailors/EZPZ/tree/main/ezpz-rust-ti",
    "metadata_": {
      "tags": ["testing", "development", "api"],
      "license": "MIT",
      "python_version": ">=3.8",
      "dependencies": ["requests", "pydantic"],
      "documentation": "https://docs.example.com/plugin",
      "support_email": "support@example.com",
    },
  }
