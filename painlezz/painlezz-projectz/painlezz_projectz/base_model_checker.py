from typing import Any
from pathlib import Path

import toml
import libcst as cst

pyproject_file = Path().cwd().joinpath("pyproject.toml")
pyproject_data = toml.loads(pyproject_file.read_text())
workspace_members = pyproject_data["tool"]["rye"]["workspace"]["members"]
known_first_party = pyproject_data["tool"]["ruff"]["lint"]["isort"]["known-first-party"]
runtime_evaluated_base_classes = pyproject_data["tool"]["ruff"]["lint"]["flake8-type-checking"]["runtime-evaluated-base-classes"]
runtime_evaluated_decorators = pyproject_data["tool"]["ruff"]["lint"]["flake8-type-checking"]["runtime-evaluated-decorators"]


class BaseModelInheritanceChecker(cst.CSTVisitor):
  def __init__(self) -> None:
    self.base_models: list[str] = ["BaseModel", "SQLModel", "BaseDBModel", "DeclarativeBase", "IdMixin"]
    self.classes: dict[str, str] = {}
    self.classes_to_add: list[str] = []

  def visit_ClassDef(self, node: cst.ClassDef) -> None:
    class_name = node.name.value
    base_classes: list[Any] = [base.value for base in node.bases]
    if len(base_classes) > 0:
      self.classes[class_name] = base_classes[0]

  def search(self) -> None:
    for key in self.classes:
      current_class = key
      visited: set[str] = set()
      while current_class:
        if current_class in self.base_models:
          self.classes_to_add.append(key)
          break
        if current_class in visited:
          break
        visited.add(current_class)
        current_class = self.classes.get(current_class, "")


if __name__ == "__main__":
  visitor = BaseModelInheritanceChecker()
  for workspace in workspace_members:
    for folder in Path.cwd().rglob(workspace):
      for file in folder.rglob("*.{py,pyi}"):
        module = cst.parse_module(file.read_text(encoding="utf8"))
        module.visit(visitor)
  visitor.search()
  with Path.open(pyproject_file, mode="r", encoding="utf-8") as f:
    pyproject_data = toml.load(f)
    pyproject_data["tool"]["ruff"]["lint"]["flake8-type-checking"]["runtime-evaluated-base-classes"] = visitor.classes_to_add
  with Path.open(pyproject_file, "w", encoding="utf-8") as f:
    toml.dump(pyproject_data, f)
