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
  if not config_file.exists():
    return {}

  try:
    with config_file.open("rb") as f:
      return tomllib.load(f)
  except Exception:
    logger.warning("Failed to load ezpz.toml")
    return {}


def get_package_manager_from_config() -> str | None:
  config = load_ezpz_config()
  return config.get("ezpz_pluginz", {}).get("package_manager")


def check_ezpz_config() -> bool:
  return Path("ezpz.toml").exists()


def create_default_ezpz_config(project_name: str = "my-ezpz-project") -> None:
  config_content = f"""[ezpz_pluginz]
name = "{project_name}"
include = [
    "src/",
    "*.py"
]
site_customize = true
package_manager = "pip"  # Options: pip, uv, rye, poetry, pipenv, conda, mamba
"""
  Path("ezpz.toml").write_text(config_content)
