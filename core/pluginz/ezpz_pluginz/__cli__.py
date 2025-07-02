# type: ignore[B008]

import os
import time
import logging
from typing import Any

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


@app.command(name="help")
def show_help(command: str = typer.Argument(None, help="Show help for a specific command")) -> None:
  if command:
    command_help = {
      "mount": {
        "description": "Mount all configured plugins to make them available in your environment",
        "usage": "ezplugins mount",
        "details": [
          "• Loads plugins specified in your ezpz.toml configuration",
          "• Makes plugin functions available for use",
          "• Run this after installing new plugins or changing configuration",
        ],
      },
      "unmount": {
        "description": "Unmount all plugins from your environment",
        "usage": "ezplugins unmount",
        "details": ["• Removes mounted plugins from your environment", "• Useful for troubleshooting or cleaning up"],
      },
      "register": {
        "description": "Register a new plugin to the remote registry",
        "usage": "ezplugins register <plugin_path>",
        "details": [
          "• Requires GITHUB_PAT environment variable",
          "• Plugin must have a register_plugin() function",
          "• Path should point to your plugin directory or file",
          "• Plugin will be made available to other users",
        ],
      },
      "update": {
        "description": "Update an existing plugin in the registry",
        "usage": "ezplugins update <plugin_name> <plugin_path>",
        "details": [
          "• Requires GITHUB_PAT environment variable",
          "• Updates the plugin version in the remote registry",
          "• Plugin must already exist in the registry",
        ],
      },
      "refresh": {
        "description": "Refresh the local plugin registry from remote",
        "usage": "ezplugins refresh",
        "details": [
          "• Downloads latest plugin information from registry",
          "• Run this to see newly published plugins",
          "• Automatically done when installing plugins",
        ],
      },
      "status": {
        "description": "Show current status of the plugin system",
        "usage": "ezplugins status",
        "details": [
          "• Shows registry URL and local cache information",
          "• Displays number of available and verified plugins",
          "• Useful for troubleshooting registry issues",
        ],
      },
      "add": {
        "description": "Install and optionally mount a plugin",
        "usage": "ezplugins add <plugin_name> [--no-auto-mount]",
        "details": [
          "• Downloads and installs the plugin package",
          "• Creates ezpz.toml if not present",
          "• Automatically mounts plugins unless --no-auto-mount is used",
          "• Use 'ezplugins list' to see available plugins",
        ],
      },
      "list": {
        "description": "List all available plugins in the registry",
        "usage": "ezplugins list",
        "details": [
          "• Shows all plugins with installation status (✓ = installed, ○ = not installed)",
          "• Displays plugin descriptions, authors, and versions",
          "• Sets up local registry if not present",
        ],
      },
      "find": {
        "description": "Advanced search for plugins with flexible filtering",
        "usage": "ezplugins find <keyword> [options]",
        "details": [
          "• Search in specific fields: --field name|description|author|package|category|aliases|all",
          "• Search remote registry: --remote",
          "• Search both local and remote: --both",
          "• Case-sensitive search: --case-sensitive",
          "• Exact match: --exact",
          "• Limit results: --limit N",
          "• Show detailed info: --details",
          "• Examples:",
          "  ezplugins find rust --field category",
          "  ezplugins find 'technical analysis' --remote --details",
          "  ezplugins find polars --both --exact",
        ],
      },
    }

    if command in command_help:
      help_info = command_help[command]
      logger.info(f"Command: {command}")
      logger.info("-" * 50)
      logger.info(f"Description: {help_info['description']}")
      logger.info(f"Usage: {help_info['usage']}")
      logger.info("")
      logger.info("Details:")
      for detail in help_info["details"]:
        logger.info(f"  {detail}")
    else:
      logger.error(f"Unknown command: {command}")
      logger.info("Available commands: mount, unmount, register, update, refresh, status, add, list, find")
      raise typer.Exit(1)
    return

  # Show general help
  logger.info("EZPZ Plugins - Plugin Management System")
  logger.info("=" * 50)
  logger.info("")
  logger.info("EZPZ Plugins allows you to discover, install, and manage plugins for your projects.")
  logger.info("")

  logger.info("QUICK START:")
  logger.info("  1. List available plugins:     ezplugins list")
  logger.info("  2. Install a plugin:           ezplugins add <plugin_name>")
  logger.info("  3. Mount plugins:              ezplugins mount")
  logger.info("")

  logger.info("AVAILABLE COMMANDS:")
  logger.info("")

  commands = [
    ("list", "List all available plugins"),
    ("find", "Advanced search for plugins with flexible filtering"),
    ("add", "Install and mount a plugin"),
    ("mount", "Mount configured plugins"),
    ("unmount", "Unmount all plugins"),
    ("status", "Show plugin system status"),
    ("refresh", "Refresh local plugin registry"),
    ("register", "Register a new plugin (requires GitHub PAT)"),
    ("update", "Update an existing plugin (requires GitHub PAT)"),
    ("help", "Show this help or help for specific commands"),
  ]

  for cmd, desc in commands:
    logger.info(f"  {cmd:<12} {desc}")

  logger.info("")
  logger.info("EXAMPLES:")
  logger.info("  ezplugins list                    # Show all available plugins")
  logger.info("  ezplugins find database           # Search for database-related plugins")
  logger.info("  ezplugins add my-plugin           # Install and mount 'my-plugin'")
  logger.info("  ezplugins add my-plugin --no-auto-mount  # Install without mounting")
  logger.info("  ezplugins help add                # Show detailed help for 'add' command")
  logger.info("")

  logger.info("CONFIGURATION:")
  logger.info("  • Configuration file: ezpz.toml (created automatically)")
  logger.info("  • Registry cache: ~/.ezpz/plugins/registry.json")
  logger.info("  • Environment variables:")
  logger.info("    - GITHUB_PAT or GITHUB_TOKEN (for registering/updating plugins)")
  logger.info("")

  logger.info("For detailed help on any command, use: ezplugins help <command>")


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
  if plugin_info is None:
    logger.error(f"No plugin found at path: {plugin_path}")
    logger.info("Make sure the path contains a plugin with a register_plugin() function in the module entry i.e '__init__.py'")
    logger.info(f"Searched in configured include paths: {config.include_str_paths}")
    raise typer.Exit(1)

  if local_registry.is_plugin_registered(plugin_info.name):
    logger.info(f"Plugin '{plugin_info.name}' is already registered")
    logger.info("Skipping registration")
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
  plugin_path: str = typer.Argument(default=..., help="Path to the updated plugin"),
) -> None:
  github_pat = get_github_pat()

  refresh()

  config = load_config()
  if not config:
    logger.error("Could not load ezpz.toml configuration")
    raise typer.Exit(1)

  # the updated plugin info
  plugin_info = find_plugin_in_path(plugin_path, config.include_str_paths)
  if not plugin_info:
    logger.error(f"No plugin found at path: {plugin_path}")
    raise typer.Exit(1)

  # plugin ID from the registry
  local_registry = LocalPluginRegistry()
  existing_plugin = local_registry.get_plugin(plugin_name)

  if not existing_plugin:
    logger.error(f"Plugin '{plugin_name}' not found in local registry")
    logger.info("Try running 'ezplugins refresh' to update the local registry")
    raise typer.Exit(1)

  api = PluginRegistryAPI()
  logger.info(f"Updating plugin: {plugin_info.name}")
  success = api.update_plugin(existing_plugin.id, plugin_info, github_pat)

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


@app.command(name="find")
def find(
  keyword: str = typer.Argument(help="Keyword to search for in plugins"),
  *,
  field: str = typer.Option(None, "--field", "-f", help="Search in specific field: name, description, author, package, category, aliases, all"),
  remote: bool = typer.Option(False, "--remote", "-r", help="Search in remote registry instead of local"),
  both: bool = typer.Option(False, "--both", "-b", help="Search in both local and remote registries"),
  case_sensitive: bool = typer.Option(False, "--case-sensitive", "-c", help="Perform case-sensitive search"),
  exact: bool = typer.Option(False, "--exact", "-e", help="Exact match instead of partial match"),
  limit: int = typer.Option(50, "--limit", "-l", help="Maximum number of results to show"),
  show_details: bool = typer.Option(False, "--details", "-d", help="Show detailed plugin information"),
) -> None:
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
