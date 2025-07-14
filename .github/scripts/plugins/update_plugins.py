#!/usr/bin/env python3
# ruff: noqa: T201

import os
import sys
import json
import subprocess


def main() -> None:
  print("🔄 Updating existing plugins...")

  plugins_to_update = os.environ.get("PLUGINS_TO_UPDATE", "[]")
  dry_run = os.environ.get("DRY_RUN", "false").lower() == "true"

  try:
    plugins = json.loads(plugins_to_update)
  except json.JSONDecodeError as e:
    print(f"❌ Failed to parse PLUGINS_TO_UPDATE: {e}")
    sys.exit(1)

  if not plugins:
    print(" No plugins to update")
    return

  failed_plugins = list[str]()

  for plugin in plugins:
    package_name = plugin["package_name"]
    plugin_path = plugin["path"]
    plugin_name = plugin["registration_info"]["name"]

    print(f"🔄 Updating plugin: {package_name} ({plugin_name})")

    try:
      if dry_run:
        print(f"🏃 DRY RUN: Would update {plugin_name} from {plugin_path}")
      else:
        cmd = ["rye", "run", "ezplugins", "update", plugin_name, plugin_path]
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)  # noqa: S603
        print(f"✅ Successfully updated {package_name}")
        print(result.stdout)
    except subprocess.CalledProcessError as e:
      print(f"❌ Failed to update {package_name}: {e}")
      print(f"stdout: {e.stdout}")
      print(f"stderr: {e.stderr}")
      failed_plugins.append(package_name)
      continue

  if failed_plugins:
    print(f"\n❌ Failed to update {len(failed_plugins)} plugins: {', '.join(failed_plugins)}")
    sys.exit(1)
  else:
    print(f"\n✅ Successfully updated {len(plugins)} plugins")


if __name__ == "__main__":
  main()
