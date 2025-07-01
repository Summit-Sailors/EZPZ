# type: ignore[B008]

import os
import time
import logging

import typer

from ezpz_pluginz import mount_plugins, unmount_plugins
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


def get_github_pat() -> str:
  pat = os.getenv("GITHUB_PAT")
  if not pat:
    logger.error("GitHub PAT required. Set GITHUB_PAT or GITHUB_TOKEN environment variable")
    raise typer.Exit(1)
  return pat


@app.command(name="mount")
def mount() -> None:
  mount_plugins()


@app.command(name="unmount")
def unmount() -> None:
  unmount_plugins()


@app.command(name="register")
def register(
  plugin_path: str = typer.Argument(..., help="Path to the plugin to register"),
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

  github_pat = get_github_pat()
  api = PluginRegistryAPI()
  success = api.register_plugin(plugin_info, github_pat)

  if success:
    logger.info(f"Successfully registered '{plugin_info.name}'")
    local_registry.fetch_and_update_registry()
  else:
    logger.error(f"Failed to register '{plugin_info.name}'")
    raise typer.Exit(1)


@app.command(name="update")
def update_plugin(
  plugin_name: str = typer.Argument(help="Name of the plugin to update"),
  plugin_path: str = typer.Argument(..., help="Path to the updated plugin"),
) -> None:
  github_pat = get_github_pat()

  config = load_config()
  if not config:
    logger.error("Could not load ezpz.toml configuration")
    raise typer.Exit(1)

  # Find the updated plugin info
  plugin_info = find_plugin_in_path(plugin_path, config.include_str_paths)
  if not plugin_info:
    logger.error(f"No plugin found at path: {plugin_path}")
    raise typer.Exit(1)

  # Get the plugin ID from the registry
  local_registry = LocalPluginRegistry()
  existing_plugin = local_registry.get_plugin(plugin_name)

  if not existing_plugin:
    logger.error(f"Plugin '{plugin_name}' not found in local registry")
    logger.info("Try running 'ezplugins refresh' to update the local registry")
    raise typer.Exit(1)

  # Search for plugin ID via API
  api = PluginRegistryAPI()
  plugins = api.search_plugins(plugin_name)
  matching_plugin = None

  for p in plugins:
    if p.name == plugin_name:
      matching_plugin = p
      break

  if not matching_plugin:
    logger.error(f"Plugin '{plugin_name}' not found in remote registry")
    raise typer.Exit(1)

  plugin_id = getattr(matching_plugin, "id", None)
  if not plugin_id:
    logger.error("Could not determine plugin ID for update")
    raise typer.Exit(1)

  logger.info(f"Updating plugin: {plugin_info.name}")
  success = api.update_plugin(plugin_id, plugin_info, github_pat)

  if success:
    logger.info(f"Successfully updated '{plugin_info.name}'")
    local_registry.fetch_and_update_registry()
  else:
    logger.error(f"Failed to update '{plugin_info.name}'")
    raise typer.Exit(1)


@app.command(name="refresh")
def refresh() -> None:
  logger.info("Refreshing local plugin registry...")
  registry = LocalPluginRegistry()
  if registry.fetch_and_update_registry():
    logger.info("Local plugin registry refreshed successfully")
  else:
    raise typer.Exit(1)


@app.command(name="status")
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


@app.command(name="add")
def add(
  plugin_name: str = typer.Argument(help="Name of the plugin to install"),
  *,
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

  if is_package_installed(plugin.package_name):
    logger.info(f"Package {plugin.package_name} is already installed")
  else:
    if not install_package(plugin.package_name):
      logger.info(f"Failed to install {plugin.package_name}")
      raise typer.Exit(1)
    logger.info(f"Successfully installed {plugin.package_name}")

  if not check_ezpz_config():
    if typer.confirm("No ezpz.toml found. Create default configuration?"):
      project_name = typer.prompt("Project name", default="my-ezpz-project")
      create_default_ezpz_config(project_name)
      logger.info("Created ezpz.toml configuration")
    elif auto_mount:
      logger.info("Cannot auto-mount without ezpz.toml")
      auto_mount = False

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


@app.command(name="find")
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


if __name__ == "__main__":
  app()
