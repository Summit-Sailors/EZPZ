from typing import TYPE_CHECKING, Self

import yaml
from pydantic import BaseModel

if TYPE_CHECKING:
  from pathlib import Path


class BaseYamlModel(BaseModel):
  def to_yaml(self) -> str:
    return yaml.safe_dump(self.model_dump(mode="json"), sort_keys=False)

  @classmethod
  def from_yaml(cls, content: str) -> Self:
    return cls.model_validate(yaml.safe_load(content))

  @classmethod
  def from_yaml_file(cls, lockfile_path: "Path") -> Self:
    return cls.from_yaml(lockfile_path.read_text())

  def to_yaml_file(self, lockfile_path: "Path") -> None:
    lockfile_path.write_text(self.to_yaml())
