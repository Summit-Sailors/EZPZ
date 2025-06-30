import datetime
from typing import Any


def register_plugin() -> dict[str, Any]:
  now = datetime.datetime.now(datetime.UTC).isoformat()
  return {
    "name": "rust-ti",
    "package_name": "ezpz-rust-ti",
    "description": "Rust-powered technical analysis indicators for Polars DataFrames",
    "aliases": ["ta", "technical-analysis", "indicators"],
    "version": "0.1.0",
    "author": "Summit Sailors",
    "homepage": "https://github.com/Summit-Sailors/EZPZ/tree/main/ezpz-rust-ti",
    "created_at": now,
    "updated_at": now,
  }
