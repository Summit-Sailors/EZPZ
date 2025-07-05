#!/usr/bin/env python3
# ruff: noqa: T201

import os
import sys
import json
import importlib.util
from typing import Any, Callable
from pathlib import Path

import toml


def load_ezpz_config() -> dict[str, Any]:
  try:
    with Path.open(Path("ezpz.toml"), "r") as f:
      config = toml.load(f)
    return config.get("ezpz_pluginz", {})
  except FileNotFoundError:
    print("❌ ezpz.toml not found")
    sys.exit(1)


def load_local_registry() -> dict[str, Any]:
  possible_paths = [Path.home() / ".ezpz" / "plugins.json", Path(".ezpz") / "plugins.json", Path("plugins.json"), Path("registry.json")]

  for registry_path in possible_paths:
    if registry_path.exists():
      print(f"📁 Found registry at: {registry_path}")
      with Path.open(registry_path, "r") as f:
        return json.load(f)

  print("❌ Local registry not found. Did 'rye run ezplugins refresh' run successfully?")
  print("🔍 Searched in:", [str(p) for p in possible_paths])
  return {"plugins": []}


def extract_project_plugins(config: dict[str, Any]) -> list[dict[str, str]]:
  include_paths: list[str] = config.get("include", [])
  project_plugins = list[dict[str, str]]()

  for path in include_paths:
    if Path.exists(Path(path)):
      package_name = Path(path).name
      project_plugins.append({"package_name": package_name, "path": path})
    else:
      print(f"⚠️ Path not found: {path}")

  return project_plugins


def _load_plugin_from_file(file_path: Path) -> dict[str, Any] | None:
  try:
    if not file_path.exists():
      return None

    spec = importlib.util.spec_from_file_location(f"plugin_{file_path.stem}", file_path)

    if spec is None or spec.loader is None:
      return None

    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)

    if hasattr(module, "register_plugin"):
      register_func: Callable[[], dict[str, Any]] = module.register_plugin
      return register_func()
  except Exception as e:
    print(f"⚠️ Error loading plugin from {file_path}: {e}")
    return None
  return None


def _extract_package_name(plugin_dir_name: str) -> str:
  return plugin_dir_name.replace("-", "_")


def _load_plugin_from_path(plugin_path: Path) -> dict[str, Any] | None:
  try:
    entry_point_patterns = [
      plugin_path / "python" / _extract_package_name(plugin_path.name) / "__init__.py",
      plugin_path / "src" / _extract_package_name(plugin_path.name) / "__init__.py",
      plugin_path / _extract_package_name(plugin_path.name) / "__init__.py",
      plugin_path / "__init__.py",
    ]

    for entry_point_path in entry_point_patterns:
      if entry_point_path.exists():
        print(f"🔍 Trying entry point: {entry_point_path}")
        plugin_info = _load_plugin_from_file(entry_point_path)
        if plugin_info:
          print(f"✅ Successfully loaded plugin from {entry_point_path}")
          return plugin_info

    # If no standard patterns work, search recursively
    print(f"🔍 Searching recursively in {plugin_path} for register_plugin function...")
    for init_file in plugin_path.rglob("__init__.py"):
      try:
        with Path.open(init_file, "r") as f:
          content = f.read()
          if "def register_plugin" in content:
            print(f"🔍 Found register_plugin in {init_file}")
            plugin_info = _load_plugin_from_file(init_file)
            if plugin_info:
              print(f"✅ Successfully loaded plugin from {init_file}")
              return plugin_info
      except Exception as e:
        print(f"⚠️ Error reading {init_file}: {e}")
        continue

  except Exception as e:
    print(f"❌ Error loading plugin from {plugin_path}: {e}")

  return None


def get_plugin_registration_info(plugin_path: str) -> dict[str, Any] | None:
  """Get registration info by calling register_plugin() function"""
  plugin_path_obj = Path(plugin_path)
  print(f"🔍 Searching for plugin in: {plugin_path_obj}")

  if plugin_path_obj.exists():
    plugin_info = _load_plugin_from_path(plugin_path_obj)
    if plugin_info:
      return plugin_info

  return None


def compare_plugins(project_plugin_info: dict[str, Any], registry_plugin: dict[str, Any]) -> bool:
  fields_to_compare = ["version", "description", "author", "category", "homepage", "aliases", "metadata_"]

  for field in fields_to_compare:
    project_value = project_plugin_info.get(field)
    registry_value = registry_plugin.get(field)

    if project_value != registry_value:
      print(f"🔄 Difference found in {field}: {project_value} vs {registry_value}")
      return True

  return False


def main() -> None:
  print("🔍 Starting plugin analysis...")

  config = load_ezpz_config()
  local_registry = load_local_registry()

  # project plugins
  project_plugins = extract_project_plugins(config)
  print(f"📦 Found {len(project_plugins)} plugins in project")

  # lookup for registry plugins
  registry_plugins = {p["package_name"]: p for p in local_registry.get("plugins", [])}

  plugins_to_register = list[dict[str, Any]]()
  plugins_to_update = list[dict[str, Any]]()

  for project_plugin in project_plugins:
    package_name = project_plugin["package_name"]
    plugin_path = project_plugin["path"]

    print(f"\n📋 Analyzing plugin: {package_name}")

    # registration info from the plugin
    registration_info = get_plugin_registration_info(plugin_path)
    if not registration_info:
      print(f"⚠️ Skipping {package_name} - no registration info found")
      continue

    # if plugin exists in registry
    if package_name not in registry_plugins:
      print(f"🆕 New plugin detected: {package_name}")
      plugins_to_register.append({"package_name": package_name, "path": plugin_path, "registration_info": registration_info})
    else:
      # compare with registry version
      registry_plugin = registry_plugins[package_name]
      if compare_plugins(registration_info, registry_plugin):
        print(f"🔄 Update needed for: {package_name}")
        plugins_to_update.append({"package_name": package_name, "path": plugin_path, "registration_info": registration_info, "registry_info": registry_plugin})
      else:
        print(f"✅ No changes detected for: {package_name}")

  print("\n📊 Analysis Summary:")
  print(f"   - Plugins to register: {len(plugins_to_register)}")
  print(f"   - Plugins to update: {len(plugins_to_update)}")

  # GitHub outputs
  has_changes = len(plugins_to_register) > 0 or len(plugins_to_update) > 0

  github_output_path = Path(os.environ["GITHUB_OUTPUT"])
  with github_output_path.open("a") as f:
    f.write(f"project-plugins={json.dumps(project_plugins)}\n")
    f.write(f"plugins-to-register={json.dumps(plugins_to_register)}\n")
    f.write(f"plugins-to-update={json.dumps(plugins_to_update)}\n")
    f.write(f"has-changes={str(has_changes).lower()}\n")

  print("\n✅ Plugin analysis completed")


if __name__ == "__main__":
  main()
