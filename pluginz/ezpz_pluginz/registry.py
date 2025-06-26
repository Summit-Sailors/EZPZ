from typing import Any
from dataclasses import dataclass


@dataclass
class PluginInfo:
  """Information about an EZPZ plugin."""

  name: str
  package_name: str
  description: str
  aliases: list[str]
  version: str | None = None
  author: str | None = None
  homepage: str | None = None


def register_plugin() -> dict[str, Any]:
  """
  Plugin developers should implement this function in their package
  and register it as an entry point under 'ezpz.plugins' group.

  This is a template function that plugin developers should copy
  and modify for their specific plugin.

  # Returns:
      dict containing plugin information that will be converted to PluginInfo

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
    "name": "example-plugin",
    "package_name": "ezpz-example-plugin",
    "description": "An example EZPZ plugin",
    "aliases": ["example", "demo"],
    "version": "1.0.0",
    "author": "Plugin Developer",
    "homepage": "https://github.com/developer/ezpz-example-plugin",
  }
