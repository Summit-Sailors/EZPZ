import sys
from pathlib import Path

import toml

# To run python py-utils/py_utils/addWorkspace.py [workspace_name]


class WorkspaceNameException(Exception):
  def __init__(self) -> None:
    super().__init__("Workspace name is needed")


ROOT_DIR = Path(__file__).parent.parent.parent

if len(sys.argv) < 2:
  raise WorkspaceNameException

workspace_name = sys.argv[1]
workspace_dir = Path(ROOT_DIR, workspace_name.replace("_", "-"))
Path.mkdir(workspace_dir)
Path.mkdir(Path(workspace_dir, workspace_name))
with Path.open(Path(workspace_dir, workspace_name, "__init__.py"), "w") as f:
  pass
with Path.open(Path(workspace_dir, "README.md"), "w") as f:
  pass
toml_data = {
  "build-system": {"build-backend": "hatchling.build", "requires": ["hatchling"]},
  "project": {
    "authors": [],
    "dependencies": [],
    "description": "",
    "name": workspace_name,
    "readme": "README.md",
    "requires-python": ">=3.13,<3.14",
    "version": "0.0.1",
  },
}
with Path.open(Path(workspace_dir, "pyproject.toml"), "w", encoding="utf-8") as f:
  toml.dump(toml_data, f)

with Path.open(Path(ROOT_DIR, "pyproject.toml"), "r", encoding="utf-8") as f:
  pyproject_data = toml.load(f)
  pyproject_data["tool"]["ruff"]["lint"]["isort"]["known-first-party"].append(workspace_name)

with Path.open(Path(ROOT_DIR, "pyproject.toml"), "w", encoding="utf-8") as f:
  toml.dump(pyproject_data, f)
