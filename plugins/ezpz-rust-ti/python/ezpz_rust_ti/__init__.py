from typing import Any


def register_plugin() -> dict[str, Any]:
  return {
    "name": "rust-ti",
    "package_name": "ezpz-rust-ti",
    "description": "Rust-powered technical analysis indicators for Polars LazyFrame",
    "aliases": ["ta", "technical-analysis", "indicators"],
    "version": "0.1.0",
    "author": "Summit Sailors",
    "category": "Technical analysis",
    "homepage": "https://github.com/Summit-Sailors/EZPZ/tree/main/ezpz-rust-ti",
    "metadata_": {
      "tags": ["polars", "indicators", "plugins"],
      "license": "MIT",
      "python_version": ">=3.13",
      "dependencies": ["ezpz-pluginz", "polars==1.31.0", "pyarrow==20.0.0"],
      "documentation": "https://github.com/Summit-Sailors/EZPZ/blob/main/ezpz-rust-ti/README.md",
      "support_email": "oketchs702@gmail.com",
    },
  }
