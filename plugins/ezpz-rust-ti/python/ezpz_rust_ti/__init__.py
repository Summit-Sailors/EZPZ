from typing import Any


def register_plugin() -> dict[str, Any]:
  return {
    "name": "rust-ti",
    "package_name": "ezpz-rust-ti",
    "description": "Rust-powered technical analysis indicators for Polars DataFrames",
    "aliases": ["ta", "technical-analysis", "indicators"],
    "version": "0.1.0",
    "author": "Summit Sailors",
    "category": "Technical analysis",
    "homepage": "https://github.com/Summit-Sailors/EZPZ/tree/main/ezpz-rust-ti",
    "metadata_": {
      "tags": ["testing", "development", "api"],
      "license": "MIT",
      "python_version": ">=3.8",
      "dependencies": ["requests", "pydantic"],
      "documentation": "https://docs.example.com/plugin",
      "support_email": "support@example.com",
    },
  }
