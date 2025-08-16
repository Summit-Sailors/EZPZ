import os
import tomllib
from typing import Any
from pathlib import Path

from ezpz_pluginz.logger import setup_logger

logger = setup_logger("Config")

# Registry configuration
DEFAULT_REGISTRY_URL = "http://127.0.0.1:8080"
REGISTRY_URL = os.getenv("EZPZ_REGISTRY_URL", DEFAULT_REGISTRY_URL)
API_VERSION = "v1"
REQUEST_TIMEOUT = 30.0

# HTTP status codes
HTTP_UNAUTHORIZED = 401
HTTP_NOT_FOUND = 404
HTTP_SERVER_ERROR = 500

# Pagination
DEFAULT_BATCH_SIZE = 100
DEFAULT_PAGE_START = 1

# Default values
DEFAULT_VERSION = "0.0.1"
DEFAULT_HOMEPAGE = "https://github.com/Summit-Sailors/EZPZ.git"

# Local storage
LOCAL_REGISTRY_DIR = Path.home() / ".ezpz" / "registry"
LOCAL_REGISTRY_FILE = LOCAL_REGISTRY_DIR / "plugins.json"


def load_ezpz_config() -> dict[str, Any]:
  config_file = Path("ezpz.toml")
  if config_file.exists():
    try:
      with config_file.open("rb") as f:
        return tomllib.load(f).get("ezpz_pluginz", {})
    except Exception:
      logger.warning("Failed to load ezpz.toml")
      return {}

  pyproject_file = Path("pyproject.toml")
  if pyproject_file.exists():
    try:
      with pyproject_file.open("rb") as f:
        return tomllib.load(f).get("tool", {}).get("ezpz", {})
    except Exception:
      logger.warning("Failed to load pyproject.toml")
      return {}

  logger.warning("Neither ezpz.toml nor pyproject.toml with [tool.ezpz_pluginz] found")
  return {}
