# type: ignore[B008]

import os
import time
import shutil
from typing import Any
from pathlib import Path

import typer

from ezpz_pluginz import mount_plugins, unmount_plugins
from ezpz_pluginz.logger import setup_logger
from ezpz_pluginz.registry import (
  REGISTRY_URL,
  LOCAL_REGISTRY_DIR,
  LOCAL_REGISTRY_FILE,
  PluginRegistryAPI,
  LocalPluginRegistry,
  find_plugin_in_path,
  is_package_installed,
  setup_local_registry,
)
from ezpz_pluginz.toml_schema import load_config
from ezpz_pluginz.registry.models import PluginUpdate

app = typer.Typer(name="ezpz", pretty_exceptions_show_locals=False, pretty_exceptions_short=True)
registry_app = typer.Typer(name="registry", help="Registry management commands")
app.add_typer(typer_instance=registry_app, name="registry")

logger = setup_logger("CLI")


def get_auth_secret() -> str:
  pat = os.getenv("AUTH_SECRET")
  if not pat:
    logger.error("Auth Secret required. Set AUTH_SECRET environment variable")
    raise typer.Exit(1)
  return pat


def return_bool(*, val: bool) -> bool:
  return val


# Core plugin management commands
@app.command(name="mount")
def mount() -> None:
  """
  Mount all configured plugins to make them available in your environment.

  Loads plugins specified in your ezpz.toml configuration.
  Makes plugin functions available for use.
  Run this after installing new plugins or changing configuration.
  """
  mount_plugins()


@app.command(name="unmount")
def unmount() -> None:
  """
  Unmount all plugins from your environment.

  Removes mounted plugins from your environment.
  Useful for troubleshooting or cleaning up.
  """
  unmount_plugins()


@app.command(name="list")
def list_plugins() -> None:
  """
  List all available plugins in the registry.

  Shows all plugins with installation status (✓ = installed, ○ = not installed).
  Displays plugin descriptions, authors, and versions.
  Sets up local registry if not present.
  """
  registry = LocalPluginRegistry()
  plugins = registry.list_plugins()

  if not plugins:
    logger.info("Local registry appears to be empty or not set up.")
    if not LOCAL_REGISTRY_FILE.exists():
      logger.info("Setting up local plugin registry for the first time...")
      setup_local_registry()
      registry = LocalPluginRegistry()
      plugins = registry.list_plugins()
    else:
      logger.info("Local registry exists but appears empty. Refreshing from remote...")
      if registry.fetch_and_update_registry():
        plugins = registry.list_plugins()
      else:
        logger.error("Failed to refresh local registry from remote.")

  if not plugins:
    logger.info("No plugins found in local registry after setup.")
    logger.info("This could indicate:")
    logger.info("  - Network connectivity issues")
    logger.info("  - Remote registry is empty")
    logger.info("  - Registry URL is incorrect")
    logger.info(f"  - Current registry URL: {REGISTRY_URL}")
    logger.info("Try running 'ezpz registry refresh' manually to update from remote registry.")
    return

  logger.info("Available EZPZ Plugins:")
  logger.info("-" * 50)
  for plugin in plugins:
    installed = "✓" if is_package_installed(plugin.package_name) else "○"
    logger.info(f"{installed} {plugin.name}")
    logger.info(f"   Package: {plugin.package_name}")
    logger.info(f"   Description: {plugin.description}")
    if plugin.aliases:
      logger.info(f"   Aliases: {', '.join(plugin.aliases)}")
    if plugin.author:
      logger.info(f"   Author: {plugin.author}")
    if plugin.version:
      logger.info(f"   Version: {plugin.version}")
    logger.info("")
  logger.info("")


@app.command(name="find")
def find(
  keyword: str = typer.Argument(help="Keyword to search for in plugins"),
  *,
  field: str = typer.Option(None, "--field", "-f", help="Search in specific field: name, description, author, package, category, aliases, all"),
  remote: bool = typer.Option(return_bool(val=False), "--remote", "-r", help="Search in remote registry instead of local"),
  both: bool = typer.Option(return_bool(val=False), "--both", "-b", help="Search in both local and remote registries"),
  case_sensitive: bool = typer.Option(return_bool(val=False), "--case-sensitive", "-c", help="Perform case-sensitive search"),
  exact: bool = typer.Option(return_bool(val=False), "--exact", "-e", help="Exact match instead of partial match"),
  limit: int = typer.Option(50, "--limit", "-l", help="Maximum number of results to show"),
  show_details: bool = typer.Option(return_bool(val=False), "--details", "-d", help="Show detailed plugin information"),
) -> None:
  """
  Advanced search for plugins with flexible filtering.

  Search in specific fields: --field name|description|author|package|category|aliases|all.
  Search remote registry: --remote.
  Search both local and remote: --both.
  Case-sensitive search: --case-sensitive.
  Exact match: --exact.
  Limit results: --limit N.
  Show detailed info: --details.

  Examples:
    ezpz find rust --field category
    ezpz find 'technical analysis' --remote --details
    ezpz find polars --both --exact
  """
  valid_fields = {"name", "description", "author", "package", "category", "aliases", "all", None}
  if field and field not in valid_fields:
    logger.error(f"Invalid field '{field}'. Valid options: {', '.join(f for f in valid_fields if f)}")
    raise typer.Exit(1)

  search_field = field or "all"
  search_local = not remote or both
  search_remote = remote or both

  if not keyword.strip():
    logger.error("Search keyword cannot be empty")
    raise typer.Exit(1)

  local_results = []
  remote_results = []

  if search_local:
    try:
      registry = LocalPluginRegistry()
      local_results = advanced_search_local(registry, keyword, search_field, case_sensitive=case_sensitive, exact=exact)
    except Exception as e:
      logger.warning(f"Local search failed: {e}")

  if search_remote:
    try:
      api = PluginRegistryAPI()
      remote_results = api.search_plugins(keyword)
      if search_field != "all":
        remote_results = filter_remote_results(remote_results, keyword, search_field, case_sensitive=case_sensitive, exact=exact)
    except Exception as e:
      logger.warning(f"Remote search failed: {e}")

  all_results = combine_results(local_results, remote_results)
  if limit > 0:
    all_results = all_results[:limit]
  display_search_results(all_results, keyword, search_field, searched_local=search_local, searched_remote=search_remote, show_details=show_details)


# Registry subcommands
@registry_app.command(name="health")
def health() -> None:
  """
  Check the health of the remote plugin registry.

  Verifies connectivity and status of the central plugin registry server.
  """
  remote_reg = PluginRegistryAPI()
  try:
    response = remote_reg.check_health()
    logger.info(response)
  except Exception as e:
    logger.exception("Health check failed")
    raise typer.Exit(1) from e


@registry_app.command(name="register")
def register(
  plugin_path: str = typer.Argument(..., help="Path to the plugin to register"),
) -> None:
  """
  Register a new plugin to the remote registry.

  Requires AUTH_SECRET environment variable.
  Plugin must have a register_plugin() function.
  Path should point to your plugin directory or file.
  Plugin will be made available to other users.
  """
  config = load_config()
  if not config:
    logger.error("Could not load ezpz.toml configuration")
    raise typer.Exit(1)

  local_registry = LocalPluginRegistry()
  if not local_registry.fetch_and_update_registry():
    logger.warning("Failed to refresh local plugin registry, continuing with cached data")

  plugin_info = find_plugin_in_path(plugin_path, config.include_str_paths)
  if plugin_info is None:
    logger.error(f"No plugin found at path: {plugin_path}")
    logger.info("Make sure the path contains a plugin with a register_plugin() function in the module entry i.e '__init__.py'")
    logger.info(f"Searched in configured include paths: {config.include_str_paths}")
    raise typer.Exit(1)

  if local_registry.is_plugin_registered(plugin_info.name):
    logger.info(f"Plugin '{plugin_info.name}' is already registered")
    logger.info("Skipping registration")
    return

  auth_secret = get_auth_secret()
  api = PluginRegistryAPI()
  success = api.register_plugin(plugin_info, auth_secret)

  if success:
    logger.info(f"Successfully registered '{plugin_info.name}'")
    local_registry.fetch_and_update_registry()
  else:
    logger.error(f"Failed to register '{plugin_info.name}'")
    raise typer.Exit(1)


@registry_app.command(name="push")
def update_plugin(
  plugin_name: str = typer.Argument(help="Name of the plugin to update"),
  plugin_path: str = typer.Argument(default=..., help="Path to the updated plugin"),
) -> None:
  """
  Update an existing plugin in the registry.

  Requires AUTH_SECRET environment variable.
  Updates the plugin version in the remote registry.
  Plugin must already exist in the registry.
  """
  auth_secret = get_auth_secret()
  refresh()
  config = load_config()
  if not config:
    logger.error("Could not load ezpz.toml configuration")
    raise typer.Exit(1)

  plugin_info = find_plugin_in_path(plugin_path, config.include_str_paths)
  if not plugin_info:
    logger.error(f"No plugin found at path: {plugin_path}")
    raise typer.Exit(1)

  local_registry = LocalPluginRegistry()
  existing_plugin = local_registry.get_plugin(plugin_name)
  if not existing_plugin:
    logger.error(f"Plugin '{plugin_name}' not found in local registry")
    logger.info("Try running 'ezpz registry refresh' to update the local registry")
    raise typer.Exit(1)

  api = PluginRegistryAPI()
  logger.info(f"Updating plugin: {plugin_info.name}")
  plugin_update = PluginUpdate(**plugin_info.model_dump())
  success = api.update_plugin(existing_plugin.id, plugin_update, auth_secret)

  if success:
    logger.info(f"Successfully updated '{plugin_info.name}'")
    local_registry.fetch_and_update_registry()
  else:
    logger.error(f"Failed to update '{plugin_info.name}'")
    raise typer.Exit(1)


@registry_app.command(name="refresh")
def refresh() -> None:
  """
  Refresh the local plugin registry from remote.

  Downloads latest plugin information from registry.
  Run this to see newly published plugins.
  Automatically done when installing plugins.
  """
  logger.info("Refreshing local plugin registry...")
  registry = LocalPluginRegistry()
  if registry.fetch_and_update_registry():
    logger.info("Local plugin registry refreshed successfully")
  else:
    raise typer.Exit(1)


@registry_app.command(name="status")
def status() -> None:
  """
  Show current status of the plugin system.

  Shows registry URL and local cache information.
  Displays number of available and verified plugins.
  Useful for troubleshooting registry issues.
  """
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


@registry_app.command(name="delete")
def delete_registry() -> None:
  """
  Delete the local plugin registry cache.

  Removes the local registry (~/.ezpz).
  Useful for troubleshooting registry corruption or clearing cache.
  Registry can be automatically recreated using refresh.
  """
  LOCAL_REGISTRY = Path.home() / ".ezpz"
  if not LOCAL_REGISTRY.exists():
    logger.info("Local registry file does not exist - nothing to delete")
    return
  try:
    shutil.rmtree(LOCAL_REGISTRY)
    logger.info(f"Successfully deleted local registry: {LOCAL_REGISTRY}")
  except Exception as e:
    logger.exception("Failed to delete local registry")
    raise typer.Exit(1) from e


@registry_app.command(name="delete-plugin")
def delete_plugin(_id: str = typer.Argument(help="The ID of the plugin to be deleted")) -> None:
  """
  Mark a plugin as deleted in the remote registry.

  Requires AUTH_SECRET environment variable.
  Removes the plugin from the local cache after successful remote deletion.
  """
  local_registry = LocalPluginRegistry()
  remote_registry = PluginRegistryAPI()
  pat = get_auth_secret()
  try:
    try:
      plugin = remote_registry.get_plugin(_id)
      if plugin.is_deleted:
        logger.warning(f"Plugin {_id} is already deleted")
        local_registry.remove_plugin_from_local_registry(plugin=plugin)
        return
    except Exception as e:
      logger.warning(f"Failed to check plugin status: {e}")
    remote_registry.delete_plugin(_id, pat)
    logger.info(f"Successfully deleted plugin: {_id}")
    if "plugin" in locals():
      local_registry.remove_plugin_from_local_registry(plugin=plugin)
    else:
      try:
        plugin = remote_registry.get_plugin(_id)
        local_registry.remove_plugin_from_local_registry(plugin=plugin)
      except Exception:
        local_registry.fetch_and_update_registry()
  except Exception as e:
    logger.exception("Failed to delete plugin")
    raise typer.Exit(1) from e


# Helper functions
def advanced_search_local(registry: LocalPluginRegistry, keyword: str, field: str, *, case_sensitive: bool, exact: bool) -> list:
  plugins = registry.list_plugins()
  search_keyword = keyword if case_sensitive else keyword.lower()

  return [("local", plugin) for plugin in plugins if should_include_plugin(plugin, search_keyword, field, case_sensitive=case_sensitive, exact=exact)]


def filter_remote_results(plugins: list[dict[str, Any]], keyword: str, field: str, *, case_sensitive: bool, exact: bool) -> list:
  if field == "all":
    return [("remote", plugin) for plugin in plugins]

  search_keyword = keyword if case_sensitive else keyword.lower()

  return [("remote", plugin) for plugin in plugins if should_include_plugin(plugin, search_keyword, field, case_sensitive=case_sensitive, exact=exact)]


def should_include_plugin(plugin: dict[str, Any], search_keyword: str, field: str, *, case_sensitive: bool, exact: bool) -> bool:  # noqa: PLR0911
  def get_field_value(plugin: dict[str, Any], field_name: str) -> str:
    if field_name == "package":
      field_name = "package_name"

    value = getattr(plugin, field_name, "") or ""
    if not case_sensitive:
      value = value.lower()
    return value

  def get_aliases_text(plugin: dict[str, Any]) -> str:
    aliases = getattr(plugin, "aliases", []) or []
    text = " ".join(aliases)
    if not case_sensitive:
      text = text.lower()
    return text

  def matches_text(text: str, keyword: str, *, exact: bool) -> bool:
    if exact:
      return text == keyword
    return keyword in text

  # Field-specific search
  if field == "name":
    return matches_text(get_field_value(plugin, "name"), search_keyword, exact=exact)
  if field == "description":
    return matches_text(get_field_value(plugin, "description"), search_keyword, exact=exact)
  if field == "author":
    return matches_text(get_field_value(plugin, "author"), search_keyword, exact=exact)
  if field == "package":
    return matches_text(get_field_value(plugin, "package_name"), search_keyword, exact=exact)
  if field == "category":
    return matches_text(get_field_value(plugin, "category"), search_keyword, exact=exact)
  if field == "aliases":
    return matches_text(get_aliases_text(plugin), search_keyword, exact=exact)
  # field == "all"
  search_fields = [
    get_field_value(plugin, "name"),
    get_field_value(plugin, "description"),
    get_field_value(plugin, "author"),
    get_field_value(plugin, "package_name"),
    get_field_value(plugin, "category"),
    get_aliases_text(plugin),
  ]
  return any(matches_text(field_text, search_keyword, exact=exact) for field_text in search_fields)


def combine_results(local_results: list, remote_results: list) -> list:
  """Combine and deduplicate local and remote results."""
  seen_plugins = set()
  combined = []

  # local results first (they take precedence)
  for source, plugin in local_results:
    plugin_key = (plugin.name, plugin.package_name)
    if plugin_key not in seen_plugins:
      combined.append((source, plugin))
      seen_plugins.add(plugin_key)

  # remote results that aren't already in local
  for source, plugin in remote_results:
    plugin_key = (plugin.name, plugin.package_name)
    if plugin_key not in seen_plugins:
      combined.append((source, plugin))
      seen_plugins.add(plugin_key)

  return combined


def display_search_results(results: list, keyword: str, field: str, *, searched_local: bool, searched_remote: bool, show_details: bool) -> None:
  if not results:
    search_scope = []
    if searched_local:
      search_scope.append("local")
    if searched_remote:
      search_scope.append("remote")
    scope_text = " and ".join(search_scope)

    logger.info(f"No plugins found matching '{keyword}' in {scope_text} registry")
    if field != "all":
      logger.info(f"Searched in field: {field}")
    return

  # header
  search_info = f"Found {len(results)} plugin(s) matching '{keyword}'"
  if field != "all":
    search_info += f" in field '{field}'"

  logger.info(search_info)
  logger.info("-" * 60)

  local_results = [plugin for source, plugin in results if source == "local"]
  remote_results = [plugin for source, plugin in results if source == "remote"]

  if local_results:
    logger.info(f"LOCAL REGISTRY ({len(local_results)} results):")
    logger.info("")
    for plugin in local_results:
      display_plugin_result(plugin, show_details=show_details, is_local=True)

  if remote_results:
    if local_results:  # separator if we have both
      logger.info("")
      logger.info("=" * 60)
      logger.info("")

    logger.info(f"REMOTE REGISTRY ({len(remote_results)} results):")
    logger.info("")
    for plugin in remote_results:
      display_plugin_result(plugin, show_details=show_details, is_local=False)


def display_plugin_result(plugin: dict[str, Any], *, show_details: bool, is_local: bool) -> None:
  if is_local:
    installed = "✓" if is_package_installed(plugin.package_name) else "○"
    status_prefix = f"{installed} "
  else:
    installed = "✓" if is_package_installed(plugin.package_name) else "◯"
    status_prefix = f"{installed} "

  logger.info(f"{status_prefix}{plugin.name}")

  if show_details:
    logger.info(f"   Package: {plugin.package_name}")
    logger.info(f"   Description: {plugin.description}")

    if hasattr(plugin, "aliases") and plugin.aliases:
      logger.info(f"   Aliases: {', '.join(plugin.aliases)}")

    if hasattr(plugin, "author") and plugin.author:
      logger.info(f"   Author: {plugin.author}")

    if hasattr(plugin, "version") and plugin.version:
      logger.info(f"   Version: {plugin.version}")

    if hasattr(plugin, "category") and plugin.category:
      logger.info(f"   Category: {plugin.category}")

    if hasattr(plugin, "verified") and plugin.verified:
      logger.info("   Status: ✅ Verified")

    if not is_local:
      logger.info("   Source: Remote Registry")
  else:
    logger.info(f"   {plugin.description}")

  logger.info("")


if __name__ == "__main__":
  app()
