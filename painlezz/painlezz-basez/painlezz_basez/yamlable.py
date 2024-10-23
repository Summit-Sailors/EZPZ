import sys
import logging
from typing import TYPE_CHECKING, Self

import yaml
from pydantic import BaseModel

if TYPE_CHECKING:
  from pathlib import Path


logger = logging.getLogger(__name__)


class BaseYamlModel(BaseModel):
  def to_yaml(self) -> str:
    return yaml.safe_dump(self.model_dump(mode="json"), sort_keys=False)

  @classmethod
  def from_yaml(cls, content: str) -> Self:
    return cls.model_validate(yaml.safe_load(content))

  @classmethod
  def from_yaml_file(cls, lockfile_path: "Path") -> Self:
    if not lockfile_path.exists():
      logger.error(f"Lock file {lockfile_path} does not exist. Please run the resolver first.")
      sys.exit(1)
    try:
      return cls.from_yaml(lockfile_path.read_text())
    except Exception as e:
      logger.exception(f"Failed to load lock file: {e}")
      sys.exit(1)

  def to_yaml_file(self, lockfile_path: "Path") -> None:
    try:
      lockfile_path.write_text(self.to_yaml())
      logger.info(f"Lock file saved at {lockfile_path}")
    except Exception as e:
      logger.exception(f"Failed to save lock file: {e}")
      sys.exit(1)
