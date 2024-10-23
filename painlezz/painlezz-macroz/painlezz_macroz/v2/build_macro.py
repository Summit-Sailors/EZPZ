import logging
from typing import TYPE_CHECKING, ParamSpec, cast

import libcst
from jinja2 import Template

from painlezz_macroz.v2.base import MacroBase
from painlezz_macroz.v2.e_macro_kind import MacroPhase
from painlezz_macroz.v2.meta_data_dto import MacroMetadata

if TYPE_CHECKING:
  from jinja2 import Environment

P = ParamSpec("P")

logger = logging.getLogger(__name__)


class BuildtimeTransformer[T](libcst.CSTTransformer):
  def __init__(self, jinja_env: "Environment", target: T) -> None:
    super().__init__()
    self.class_template = Template("./templates/buildtime_class_transform.j2")
    self.function_template = Template("./templates/buildtime_function_transform.j2")
    self.target = target

  def leave_ClassDef(self, original_node: libcst.ClassDef, updated_node: libcst.ClassDef) -> libcst.ClassDef:
    if original_node.name.value == str(self.target):
      return libcst.ensure_type(libcst.parse_module(self.class_template.render(node=updated_node)).body[0], libcst.ClassDef)
    return updated_node

  def leave_FunctionDef(self, original_node: libcst.FunctionDef, updated_node: libcst.FunctionDef) -> libcst.FunctionDef:
    if original_node.name.value == str(self.target):
      return libcst.ensure_type(libcst.parse_module(self.function_template.render(node=updated_node)).body[0], libcst.FunctionDef)
    return updated_node


class BuildtimeMacro[T](MacroBase[T]):
  def __init__(self) -> None:
    super().__init__()
    self.generated_module: str = ""

  def __call__(self, target: T) -> T:
    metadata = self.collect_metadata(libcst.parse_module(f"{target}"))
    self.cache[str(target)] = metadata
    return target

  def collect_metadata(self, node: libcst.CSTNode) -> MacroMetadata[T]:
    dependencies = set[str]()

    class DependencyCollector(libcst.CSTVisitor):
      def visit_Annotation(self, node: libcst.Annotation) -> None:
        if isinstance(node.annotation, libcst.Name):
          dependencies.add(node.annotation.value)

    node.visit(DependencyCollector())
    return MacroMetadata(phase=MacroPhase.BUILDTIME, target=cast(T, node), dependencies=dependencies)

  def transform(self, node: libcst.CSTNode) -> libcst.CSTNode:
    template = self.jinja_env.get_template("buildtime_macro.j2")
    return libcst.parse_module(template.render(node=node))

  def generate_code(self) -> str:
    self.analyze_dependencies()
    template = self.jinja_env.get_template("dependency_resolution_module.j2")
    return template.render(dependency_graph=self.dependency_graph)

  def build_time_hook(self) -> None:
    tree = libcst.parse_module("")
    for metadata in self.cache.values():
      tree = self.apply_transformations(tree, metadata.target)
    self.generated_module = self.generate_code()
    logger.info(f"Generated dependency resolution module:\n{self.generated_module}")

  def runtime_hook(self) -> None:
    pass

  def apply_transformations(self, tree: libcst.Module, target: T) -> libcst.Module:
    return tree.visit(BuildtimeTransformer(self.jinja_env, target))
