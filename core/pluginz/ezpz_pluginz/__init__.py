import sys
import shutil
import inspect
import importlib
from pathlib import Path
from itertools import chain

import libcst as cst

from ezpz_pluginz.logger import setup_logger
from ezpz_pluginz.lockfile import EZPZ_TOML_FILENAME, EZPZ_PROJECT_LOCKFILE_FILENAME, PolarsPluginLockfilePD
from ezpz_pluginz.toml_schema import EzpzPluginConfig
from ezpz_pluginz.e_polars_namespace import EPolarsNS
from ezpz_pluginz.register_plugin_macro import PluginPatcher

logger = setup_logger("ENTRY")


def mount_plugins() -> None:
  ezpz_pluginz_config = None
  ezpz_toml_path = Path.cwd().joinpath(EZPZ_TOML_FILENAME)

  if ezpz_toml_path.exists():
    try:
      ezpz_pluginz_config = EzpzPluginConfig.from_toml_path(ezpz_toml_path)
    except Exception as e:
      logger.warning(f"Failed to load ezpz.toml: {e}")
  else:
    pyproject_toml_path = Path.cwd().joinpath("pyproject.toml")
    if pyproject_toml_path.exists():
      try:
        ezpz_pluginz_config = EzpzPluginConfig.from_toml_path(pyproject_toml_path)
      except Exception as e:
        logger.warning(f"Failed to load pyproject.toml: {e}")

  lockfile = PolarsPluginLockfilePD.generate()
  lockfile.to_yaml_file(Path(EZPZ_PROJECT_LOCKFILE_FILENAME))

  # plugin-level lock files using the same lockfile data
  lockfile.generate_and_save_plugin_lockfiles()

  polars_ns_to_plugins = dict(chain(lockfile.project_plugins.items(), lockfile.site_plugins.items()))
  pp = PluginPatcher(polars_ns_to_plugins)

  polars_module = importlib.import_module("polars")
  patched_dir = Path.cwd() / ".patched"
  patched_dir.mkdir(exist_ok=True)

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

    local_copy_path = patched_dir / f"{ns.lower()}.py"
    local_copy_path.write_text(new_code)

    logger.info(f"Patched copy saved to {local_copy_path}")

  if ezpz_pluginz_config and ezpz_pluginz_config.site_customize:
    if hasattr(sys, "real_prefix") or (hasattr(sys, "base_prefix") and sys.base_prefix != sys.prefix):
      venv_site_path = Path(sys.prefix) / "lib" / f"python{sys.version_info.major}.{sys.version_info.minor}" / "site-packages"
    else:
      logger.warning("WARNING: The system python is executing, running ezpz plugins sitecustomize registry mouting is not advised.")
      return

    if venv_site_path.exists():
      sitecustomize_code = lockfile.generate_registry()
      sitecustomize_path = venv_site_path.joinpath("sitecustomize.py")
      sitecustomize_path.write_text(sitecustomize_code)

      (patched_dir / "sitecustomize.py").write_text(sitecustomize_code)
      logger.info(f"sitecustomize.py saved to {patched_dir / 'sitecustomize.py'}")


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

  patched_dir = Path.cwd() / ".patched"
  if patched_dir.exists():
    shutil.rmtree(patched_dir)
    logger.info(f"Removed .patched directory: {patched_dir}")
