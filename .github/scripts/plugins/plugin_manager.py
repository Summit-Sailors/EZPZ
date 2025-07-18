#!/usr/bin/env python3
# ruff: noqa: T201

import os
import sys
import json
import shutil
import argparse
import subprocess
import importlib.util
from typing import Any, TypedDict
from pathlib import Path

import toml


class PluginInfo(TypedDict):
  package_name: str
  path: str
  registration_info: dict[str, Any]


class PluginManager:
  def __init__(self) -> None:
    self.config = self.load_config()
    self.registry = self.load_registry()

  def load_config(self) -> dict[str, Any]:
    """Load configuration from ezpz.toml or pyproject.toml."""
    for config_path in [Path("ezpz.toml"), Path("pyproject.toml")]:
      if config_path.exists():
        try:
          with config_path.open("r") as f:
            config = toml.load(f)
          return config.get("ezpz_pluginz", config.get("tool", {}).get("ezpz", {}))
        except Exception as e:
          print(f"❌ Error loading {config_path}: {e}")
          sys.exit(1)
    print("⚠️ No valid configuration found, using empty config")
    return {}

  def load_registry(self) -> dict[str, Any]:
    """Load local plugin registry."""
    registry_path = Path.home() / ".ezpz" / "registry" / "plugins.json"
    if registry_path.exists():
      with registry_path.open("r") as f:
        return json.load(f)
    print("⚠️ Local registry not found, assuming empty")
    return {"plugins": []}

  def extract_project_plugins(self) -> list[PluginInfo]:
    """Extract plugins from configuration."""
    include_paths = self.config.get("include", [])
    project_plugins: list[PluginInfo] = []
    for path in include_paths:
      path_obj = Path(path)
      if path_obj.exists():
        project_plugins.append({"package_name": path_obj.name, "path": str(path_obj), "registration_info": {}})
      else:
        print(f"⚠️ Path not found: {path}")
    return project_plugins

  def get_plugin_registration_info(self, plugin_path: str) -> dict[str, Any] | None:
    """Get registration info from plugin's register_plugin function."""
    plugin_path_obj = Path(plugin_path)
    entry_points = [
      plugin_path_obj / "python" / plugin_path_obj.name.replace("-", "_") / "__init__.py",
      plugin_path_obj / "src" / plugin_path_obj.name.replace("-", "_") / "__init__.py",
      plugin_path_obj / plugin_path_obj.name.replace("-", "_") / "__init__.py",
      plugin_path_obj / "__init__.py",
    ]

    for entry_point in entry_points:
      if entry_point.exists():
        try:
          spec = importlib.util.spec_from_file_location(f"plugin_{entry_point.stem}", entry_point)
          if spec and spec.loader:
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            if hasattr(module, "register_plugin"):
              return module.register_plugin()
        except Exception as e:
          print(f"⚠️ Error loading plugin from {entry_point}: {e}")

    for init_file in plugin_path_obj.rglob("__init__.py"):
      try:
        with init_file.open("r") as f:
          if "def register_plugin" in f.read():
            spec = importlib.util.spec_from_file_location(f"plugin_{init_file.stem}", init_file)
            if spec and spec.loader:
              module = importlib.util.module_from_spec(spec)
              spec.loader.exec_module(module)
              if hasattr(module, "register_plugin"):
                return module.register_plugin()
      except Exception as e:
        print(f"⚠️ Error reading {init_file}: {e}")
    return None

  def compare_plugins(self, project_plugin: dict[str, Any], registry_plugin: dict[str, Any]) -> bool:
    """Compare plugin metadata to detect changes."""
    fields = ["version", "description", "author", "category", "homepage", "aliases", "metadata_"]
    return any(project_plugin.get(f) != registry_plugin.get(f) for f in fields)

  def write_outputs(self, outputs: dict[str, Any]) -> None:
    """Write outputs to GITHUB_OUTPUT with unique keys."""
    with Path(os.environ["GITHUB_OUTPUT"]).open("a") as f:
      f.writelines(f"{key}={json.dumps(value)}\n" for key, value in outputs.items())

  def analyze(self) -> None:
    """Analyze plugins and generate lists for registration/updates."""
    project_plugins = self.extract_project_plugins()
    registry_plugins = {p["package_name"]: p for p in self.registry.get("plugins", [])}
    plugins_to_register: list[PluginInfo] = []
    plugins_to_update: list[PluginInfo] = []

    for plugin in project_plugins:
      package_name = plugin["package_name"]
      plugin_path = plugin["path"]
      registration_info = self.get_plugin_registration_info(plugin_path)
      if not registration_info:
        print(f"⚠️ Skipping {package_name} - no registration info")
        continue
      plugin["registration_info"] = registration_info

      if package_name not in registry_plugins:
        plugins_to_register.append(plugin)
      elif self.compare_plugins(registration_info, registry_plugins[package_name]):
        plugins_to_update.append(plugin)

    self.write_outputs(
      {
        "project-plugins": project_plugins,
        "plugins-to-register": plugins_to_register,
        "plugins-to-update": plugins_to_update,
        "has-changes": len(plugins_to_register) > 0 or len(plugins_to_update) > 0,
      }
    )

  def resolve_executable(self, cmd: str) -> str:
    """Resolve the full path to an executable."""
    full_path = shutil.which(cmd)
    if not full_path:
      print(f"❌ Executable '{cmd}' not found in PATH")
      sys.exit(1)
    return full_path

  def safe_subprocess_run(self, args: list[str], **kwargs: Any) -> subprocess.CompletedProcess[Any]:
    """Run a subprocess with validated executable path."""
    validated_args = [self.resolve_executable(args[0]), *args[1:]]
    return subprocess.run(validated_args, **kwargs, check=True)  # type: ignore # noqa: S603

  def register(self, plugins_json: str, *, dry_run: bool) -> None:
    """Register new plugins."""
    plugins: list[PluginInfo] = json.loads(plugins_json)
    failed_plugins = list[str]()
    for plugin in plugins:
      package_name = plugin["package_name"]
      plugin_path = plugin["path"]
      try:
        if dry_run:
          print(f"🏃 DRY RUN: Would register {package_name}")
        else:
          self.safe_subprocess_run(["rye", "run", "ezpz", "register", plugin_path], check=True, text=True)
          print(f"✅ Registered {package_name}")
      except subprocess.CalledProcessError as e:
        print(f"❌ Failed to register {package_name}: {e}")
        failed_plugins.append(package_name)
    if failed_plugins:
      print(f"❌ Failed to register {len(failed_plugins)} plugins: {', '.join(failed_plugins)}")
      sys.exit(1)

  def update(self, plugins_json: str, *, dry_run: bool) -> None:
    """Update existing plugins."""
    plugins: list[PluginInfo] = json.loads(plugins_json)
    failed_plugins = list[str]()
    for plugin in plugins:
      package_name = plugin["package_name"]
      plugin_path = plugin["path"]
      plugin_name = plugin["registration_info"].get("name", package_name)
      try:
        if dry_run:
          print(f"🏃 DRY RUN: Would update {plugin_name}")
        else:
          self.safe_subprocess_run(["rye", "run", "ezpz", "update", plugin_name, plugin_path], check=True, text=True)
          print(f"✅ Updated {package_name}")
      except subprocess.CalledProcessError as e:
        print(f"❌ Failed to update {package_name}: {e}")
        failed_plugins.append(package_name)
    if failed_plugins:
      print(f"❌ Failed to update {len(failed_plugins)} plugins: {', '.join(failed_plugins)}")
      sys.exit(1)

  def check_publish(self, package_name: str, plugins_to_register: str, plugins_to_update: str) -> None:
    """Check if a plugin needs publishing."""
    plugins_to_register_list: list[PluginInfo] = json.loads(plugins_to_register)
    plugins_to_update_list: list[PluginInfo] = json.loads(plugins_to_update)
    needs_publishing = False
    publish_type = "none"

    for plugin in plugins_to_register_list:
      if plugin["package_name"] == package_name:
        needs_publishing = True
        publish_type = "new"
        break

    if not needs_publishing:
      for plugin in plugins_to_update_list:
        if plugin["package_name"] == package_name:
          needs_publishing = True
          publish_type = "update"
          break

    self.write_outputs({"needs-publishing": needs_publishing, "publish-type": publish_type})


def main() -> None:
  parser = argparse.ArgumentParser(description="EZPZ Plugin Manager")
  subparsers = parser.add_subparsers(dest="command", required=True)

  subparsers.add_parser("analyze", help="Analyze plugins")
  register_parser = subparsers.add_parser("register", help="Register new plugins")
  register_parser.add_argument("--dry-run", action="store_true")
  update_parser = subparsers.add_parser("update", help="Update existing plugins")
  update_parser.add_argument("--dry-run", action="store_true")
  check_publish = subparsers.add_parser("check-publish", help="Check if plugin needs publishing")
  check_publish.add_argument("--package-name", required=True)

  args = parser.parse_args()
  manager = PluginManager()

  if args.command == "analyze":
    manager.analyze()
  elif args.command == "register":
    manager.register(os.environ.get("PLUGINS_TO_REGISTER", "[]"), args.dry_run)  # type: ignore
  elif args.command == "update":
    manager.update(os.environ.get("PLUGINS_TO_UPDATE", "[]"), args.dry_run)  # type: ignore
  elif args.command == "check-publish":
    manager.check_publish(args.package_name, os.environ.get("PLUGINS_TO_REGISTER", "[]"), os.environ.get("PLUGINS_TO_UPDATE", "[]"))


if __name__ == "__main__":
  main()
