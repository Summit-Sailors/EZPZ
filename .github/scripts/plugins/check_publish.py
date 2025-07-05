#!/usr/bin/env python3
# ruff: noqa: T201

import os
import sys
import json
from pathlib import Path


def main() -> None:
  package_name = os.environ.get("PACKAGE_NAME", "")
  plugins_to_register = os.environ.get("PLUGINS_TO_REGISTER", "[]")
  plugins_to_update = os.environ.get("PLUGINS_TO_UPDATE", "[]")

  if not package_name:
    print("❌ PACKAGE_NAME environment variable not set")
    sys.exit(1)

  try:
    plugins_to_register = json.loads(plugins_to_register)
    plugins_to_update = json.loads(plugins_to_update)
  except json.JSONDecodeError as e:
    print(f"❌ Failed to parse plugin lists: {e}")
    sys.exit(1)

  needs_publishing = False
  publish_type = "none"

  # always publish new plugins
  for plugin in plugins_to_register:
    if plugin["package_name"] == package_name:
      needs_publishing = True
      publish_type = "new"
      break

  # Publish only if significant changes for updates
  if not needs_publishing:
    for plugin in plugins_to_update:
      if plugin["package_name"] == package_name:
        # For updates, we assume if it made it to the update list,
        # it has significant changes worth publishing TODO: more checks
        needs_publishing = True
        publish_type = "update"
        break

  # Set GitHub outputs
  with Path(os.environ["GITHUB_OUTPUT"]).open("a") as f:
    f.write(f"needs-publishing={str(needs_publishing).lower()}\n")
    f.write(f"publish-type={publish_type}\n")

  print(f"Plugin {package_name} needs publishing: {needs_publishing} (type: {publish_type})")


if __name__ == "__main__":
  main()
