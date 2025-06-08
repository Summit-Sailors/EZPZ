import subprocess
from enum import Enum
from typing import Iterable
from pathlib import Path
from dataclasses import dataclass

import rich

__all__ = ["Formatter"]

ROOT_DIR_PATH = Path(__file__).parent.parent.parent

RUSTFMT_CFG = ROOT_DIR_PATH.joinpath(".rustfmt.toml")
PRETTIER_CFG = ROOT_DIR_PATH.joinpath(".prettierrc.yml")
RUFF_CFG = ROOT_DIR_PATH.joinpath("pyproject.toml")
TAPLO_CFG = ROOT_DIR_PATH.joinpath("taplo.toml")


class FileExtension(Enum):
  PY = ".py"
  PYI = ".pyi"
  TOML = ".toml"
  JS = ".js"
  JSX = ".jsx"
  TS = ".ts"
  TSX = ".tsx"
  CSS = ".css"
  SCSS = ".scss"
  JSON = ".json"
  MD = ".md"
  YML = ".yml"
  YAML = ".yaml"
  RS = ".rs"


@dataclass
class _Formatter:
  cmds: list[str]
  cfg: "Path | None"


_FORMATTERS: dict[FileExtension, _Formatter] = {
  FileExtension.PY: _Formatter(cmds=["rye run ruff check --fix", "rye run ruff format"], cfg=RUFF_CFG),
  FileExtension.PYI: _Formatter(cmds=["rye run ruff check --fix", "rye run ruff format"], cfg=RUFF_CFG),
  FileExtension.TOML: _Formatter(cmds=["taplo format"], cfg=TAPLO_CFG),
  FileExtension.JS: _Formatter(cmds=["pnpm prettier -w"], cfg=PRETTIER_CFG),
  FileExtension.JSX: _Formatter(cmds=["pnpm prettier -w"], cfg=PRETTIER_CFG),
  FileExtension.TS: _Formatter(cmds=["pnpm prettier -w"], cfg=PRETTIER_CFG),
  FileExtension.TSX: _Formatter(cmds=["pnpm prettier -w"], cfg=PRETTIER_CFG),
  FileExtension.CSS: _Formatter(cmds=["pnpm prettier -w"], cfg=PRETTIER_CFG),
  FileExtension.SCSS: _Formatter(cmds=["pnpm prettier -w"], cfg=PRETTIER_CFG),
  FileExtension.JSON: _Formatter(cmds=["pnpm prettier -w"], cfg=PRETTIER_CFG),
  FileExtension.MD: _Formatter(cmds=["pnpm prettier -w"], cfg=PRETTIER_CFG),
  FileExtension.YML: _Formatter(cmds=["pnpm prettier -w"], cfg=PRETTIER_CFG),
  FileExtension.YAML: _Formatter(cmds=["pnpm prettier -w"], cfg=PRETTIER_CFG),
  FileExtension.RS: _Formatter(cmds=["rustfmt"], cfg=RUSTFMT_CFG),
}


class Formatter:
  @staticmethod
  def format_file(file_path: "Path") -> None:
    if (ext_str := file_path.suffix.lower()) not in FileExtension:
      return
    formatter = _FORMATTERS[FileExtension(ext_str)]
    for cmd_stem in formatter.cmds:
      cmd = f"{cmd_stem} {file_path!s}"
      if formatter.cfg and formatter.cfg.exists():
        cmd += f" --config {formatter.cfg}"
      p = subprocess.run(cmd, shell=True, check=False, capture_output=True)
      rich.print(p.stdout)
      rich.print(p.stderr)

  @classmethod
  def format_paths(cls, paths: "Iterable[Path]") -> None:
    for path in paths:
      if path.is_file():
        cls.format_file(path)
      else:
        cls.format_paths(path.iterdir())
