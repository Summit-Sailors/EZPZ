import os
import time
import logging

import typer

from ezpz_pluginz.registry import (
  REGISTRY_URL,
  LOCAL_REGISTRY_DIR,
  LOCAL_REGISTRY_FILE,
  PluginRegistryAPI,
  LocalPluginRegistry,
  install_package,
  check_ezpz_config,
  find_plugin_in_path,
  is_package_installed,
  setup_local_registry,
  create_default_ezpz_config,
)
from ezpz_pluginz.toml_schema import load_config

app = typer.Typer(name="ezplugins", pretty_exceptions_show_locals=False, pretty_exceptions_short=True)
logger = logging.getLogger(__name__)


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
  plugin_path: str = typer.Argument(..., help="Path to the plugin to register"),
  api_key: str | None = typer.Option(None, "--api-key", help="Registry API key"),
) -> None:
  config = load_config()
  if not config:
    logger.error("Could not load ezpz.toml configuration")
    raise typer.Exit(1)

  local_registry = LocalPluginRegistry()
  if not local_registry.fetch_and_update_registry():
    logger.warning("Failed to refresh local plugin registry, continuing with cached data")

  plugin_info = find_plugin_in_path(plugin_path, config.include_str_paths)
  if not plugin_info:
    logger.error(f"No plugin found at path: {plugin_path}")
    logger.info("Make sure the path contains a plugin with a register_plugin() function")
    logger.info(f"Searched in configured include paths: {config.include_str_paths}")
    raise typer.Exit(1)

  if local_registry.is_plugin_registered(plugin_info.name):
    logger.info(f"Plugin '{plugin_info.name}' is already registered")
    logger.info("Skipping registration to avoid duplicates")
    return

  api = PluginRegistryAPI()
  logger.info(f"Registering plugin: {plugin_info.name}")
  success = api.register_plugin(plugin_info, api_key)

  if success:
    logger.info(f"Successfully registered '{plugin_info.name}'")
    local_registry.fetch_and_update_registry()
  else:
    logger.error(f"Failed to register '{plugin_info.name}'")
    raise typer.Exit(1)


@app.command()
def unregister(
  plugin_name: str = typer.Argument(help="Name of the plugin to unregister"),
  api_key: str | None = typer.Option(None, "--api-key", help="Registry API key"),
) -> None:
  if not api_key:
    api_key = os.getenv("EZPZ_REGISTRY_API_KEY")
    if not api_key:
      logger.error("API key required. Set EZPZ_REGISTRY_API_KEY or use --api-key")
      raise typer.Exit(1)

  api = PluginRegistryAPI()
  success = api.delete_plugin(plugin_name, api_key)

  if success:
    logger.info(f"Successfully unregistered plugin '{plugin_name}' from EZPZ registry")

    # Refresh local cache to reflect changes
    registry = LocalPluginRegistry()
    registry.fetch_and_update_registry()
  else:
    logger.error(f"Failed to unregister plugin '{plugin_name}'")
    raise typer.Exit(1)


@app.command()
def refresh() -> None:
  logger.info("Refreshing local plugin registry...")
  registry = LocalPluginRegistry()
  if registry.fetch_and_update_registry():
    logger.info("Local plugin registry refreshed successfully")
  else:
    raise typer.Exit(1)


@app.command()
def status() -> None:
  registry = LocalPluginRegistry()

  logger.info("EZPZ Plugin Registry Status:")
  logger.info("-" * 40)
  logger.info(f"Registry URL: {REGISTRY_URL}")
  logger.info(f"Local registry directory: {LOCAL_REGISTRY_DIR}")

  if LOCAL_REGISTRY_FILE.exists():
    registry_age = time.time() - LOCAL_REGISTRY_FILE.stat().st_mtime
    hours_old = registry_age / 3600
    logger.info(f"Local registry file: {LOCAL_REGISTRY_FILE}")
    logger.info(f"Registry age: {hours_old:.1f} hours")
  else:
    logger.info("Local registry file: Not found")

  plugins = registry.list_plugins()
  logger.info(f"Total plugins available: {len(plugins)}")
  verified_count = sum(1 for p in plugins if p.verified)
  logger.info(f"Verified plugins: {verified_count}")


@app.command()
def add(
  plugin_name: str = typer.Argument(help="Name of the plugin to install"),
  auto_mount: bool = typer.Option(True, "--auto-mount/--no-auto-mount", help="Automatically mount plugins after installation"),
) -> None:
  registry = LocalPluginRegistry()
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
  registry = LocalPluginRegistry()
  plugins = registry.list_plugins()

  if not plugins:
    logger.info("Local registry appears to be empty or not set up.")

    if not LOCAL_REGISTRY_FILE.exists():
      logger.info("Setting up local plugin registry for the first time...")
      setup_local_registry()

      # reload plugins after setup
      registry = LocalPluginRegistry()
      plugins = registry.list_plugins()
    else:
      # registry file exists but is empty, try to refresh
      logger.info("Local registry exists but appears empty. Refreshing from remote...")
      if registry.fetch_and_update_registry():
        plugins = registry.list_plugins()
      else:
        logger.error("Failed to refresh local registry from remote.")

  # If still no plugins after setup attempts
  if not plugins:
    logger.info("No plugins found in local registry after setup.")
    logger.info("This could indicate:")
    logger.info("  - Network connectivity issues")
    logger.info("  - Remote registry is empty")
    logger.info("  - Registry URL is incorrect")
    logger.info(f"  - Current registry URL: {REGISTRY_URL}")
    logger.info("Try running 'ezplugins refresh' manually to update from remote registry.")
    return

  logger.info("Available EZPZ Plugins:")
  logger.info("-" * 50)

  for plugin in plugins:
    installed = "✓" if is_package_installed(plugin.package_name) else "○"
    verified = "🛡️" if plugin.verified else ""
    logger.info(f"{installed} {plugin.name} {verified}")
    logger.info(f"   Package: {plugin.package_name}")
    logger.info(f"   Description: {plugin.description}")
    if plugin.aliases:
      logger.info(f"   Aliases: {', '.join(plugin.aliases)}")
    if plugin.author:
      logger.info(f"   Author: {plugin.author}")
    if plugin.version:
      logger.info(f"   Version: {plugin.version}")
    logger.info("")


@app.command()
def find(
  keyword: str = typer.Argument(help="Keyword to search for in plugins"),
) -> None:
  registry = LocalPluginRegistry()
  matching_plugins = registry.search_plugins(keyword)

  if not matching_plugins:
    logger.info(f"No plugins found matching '{keyword}'")
    return

  logger.info(f"Plugins matching '{keyword}':")
  logger.info("-" * 50)

  for plugin in matching_plugins:
    installed = "✓" if is_package_installed(plugin.package_name) else "○"
    logger.info(f"{installed} {plugin.name}")
    logger.info(f"   Package: {plugin.package_name}")
    logger.info(f"   Description: {plugin.description}")
    logger.info("")


def post_install_setup() -> None:
  logger.info("Setting up EZPZ Plugin Registry...")
  setup_local_registry()


if __name__ == "__main__":
  app()
