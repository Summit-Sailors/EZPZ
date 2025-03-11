import sys
import inspect
import logging
import importlib
import importlib.util
import importlib.metadata
from pathlib import Path
from itertools import chain

import libcst as cst

from ezpz_pluginz.lockfile import EZPZ_TOML_FILENAME, EZPZ_LOCKFILE_FILENAME, PolarsPluginLockfilePD
from ezpz_pluginz.toml_schema import EzpzPluginConfig
from ezpz_pluginz.e_polars_namespace import EPolarsNS
from ezpz_pluginz.register_plugin_macro import PluginPatcher

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


def mount_plugins() -> None:
  ezpz_pluginz_config = EzpzPluginConfig.from_toml_path(Path.cwd().joinpath(EZPZ_TOML_FILENAME))
  lockfile = PolarsPluginLockfilePD.generate()
  lockfile.to_yaml_file(Path(EZPZ_LOCKFILE_FILENAME))
  polars_ns_to_plugins = dict(chain(lockfile.project_plugins.items(), lockfile.site_plugins.items()))
  pp = PluginPatcher(polars_ns_to_plugins)
  polars_module = importlib.import_module("polars")
  for ns in polars_ns_to_plugins:
    logger.info(f"Preparing to patch polars namespace {ns}...")
    filepath = Path(inspect.getfile(getattr(polars_module, ns)))
    backup_path = filepath.with_suffix(".bak")
    ext = ".bak" if backup_path.is_file() else ".py"
    source_code = filepath.with_suffix(ext).read_text()
    if not backup_path.is_file():
      logger.info("Creating backup of polars file...")
      backup_path.write_text(source_code)
    else:
      logger.info("Backup file already exists")
    module = cst.parse_module(source_code)
    wrapper = cst.MetadataWrapper(module)
    logger.info("Patching...")
    new_code = wrapper.visit(pp).code
    logger.info("Saving...")
    filepath.write_text(new_code)
    logger.info("Complete")
  if ezpz_pluginz_config and ezpz_pluginz_config.site_customize:
    if hasattr(sys, "real_prefix") or (hasattr(sys, "base_prefix") and sys.base_prefix != sys.prefix):
      venv_site_path = Path(sys.prefix) / "lib" / f"python{sys.version_info.major}.{sys.version_info.minor}" / "site-packages"
    else:
      logger.warning("WARNING: The system python is executing, running ezpz plugins sitecustomize registry mouting is not advised.")
      return
    if venv_site_path.exists():
      venv_site_path.joinpath("sitecustomize.py").write_text(lockfile.generate_registry())


def unmount_plugins() -> None:
  polars_module = importlib.import_module("polars")
  for ns in EPolarsNS:
    filepath = Path(inspect.getfile(getattr(polars_module, ns.value)))
    backup_path = filepath.with_suffix(".bak")
    if backup_path.is_file():
      filepath.write_text(backup_path.read_text())
      backup_path.unlink()
  if hasattr(sys, "real_prefix") or (hasattr(sys, "base_prefix") and sys.base_prefix != sys.prefix):
    venv_site_path = Path(sys.prefix) / "lib" / f"python{sys.version_info.major}.{sys.version_info.minor}" / "site-packages"
  else:
    logger.warning("WARNING: The system python is executing, running ezpz plugins sitecustomize registry mouting is not advised.")
    return
  if venv_site_path.exists():
    sitecustomize = venv_site_path.joinpath("sitecustomize.py")
    if sitecustomize.exists():
      sitecustomize.unlink()
