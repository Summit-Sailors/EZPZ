#!/usr/bin/env python3
# ruff: noqa: T201

import os
import sys
import json
import subprocess


def main() -> None:
  print("🆕 Registering new plugins...")

  plugins_to_register = os.environ.get("PLUGINS_TO_REGISTER", "[]")
  dry_run = os.environ.get("DRY_RUN", "false").lower() == "true"

  try:
    plugins = json.loads(plugins_to_register)
  except json.JSONDecodeError as e:
    print(f"❌ Failed to parse PLUGINS_TO_REGISTER: {e}")
    sys.exit(1)

  if not plugins:
    print("No plugins to register")
    return

  failed_plugins = list[str]()

  for plugin in plugins:
    package_name: str = plugin["package_name"]
    plugin_path: str = plugin["path"]

    print(f"📝 Registering plugin: {package_name}")

    try:
      if dry_run:
        print(f"🏃 DRY RUN: Would register {package_name} from {plugin_path}")
      else:
        cmd = ["rye", "run", "ezplugins", "register", plugin_path]
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        print(f"✅ Successfully registered {package_name}")
        print(result.stdout)
    except subprocess.CalledProcessError as e:
      print(f"❌ Failed to register {package_name}: {e}")
      print(f"stdout: {e.stdout}")
      print(f"stderr: {e.stderr}")
      failed_plugins.append(package_name)
      continue

  if failed_plugins:
    print(f"\n❌ Failed to register {len(failed_plugins)} plugins: {', '.join(failed_plugins)}")
    sys.exit(1)
  else:
    print(f"\n✅ Successfully registered {len(plugins)} plugins")


if __name__ == "__main__":
  main()
