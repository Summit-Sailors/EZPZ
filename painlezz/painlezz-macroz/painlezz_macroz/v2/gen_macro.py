import logging
from typing import cast

import libcst

from painlezz_macroz.v2.base import MacroBase
from painlezz_macroz.v2.e_macro_kind import MacroPhase
from painlezz_macroz.v2.meta_data_dto import MacroMetadata

logger = logging.getLogger(__name__)


class DependencyCollector(libcst.CSTVisitor):
  def __init__(self) -> None:
    self.dependencies = set[str]()

  def visit_Annotation(self, node: libcst.Annotation) -> None:
    if isinstance(node.annotation, libcst.Name):
      self.dependencies.add(node.annotation.value)


class RemoveDecoratorTransformer(libcst.CSTTransformer):
  def __init__(self, name: str) -> None:
    self.name = name

  def leave_Decorator(self, original_node: libcst.Decorator, updated_node: libcst.Decorator) -> libcst.RemovalSentinel | libcst.Decorator:
    if original_node.decorator == libcst.Name(value=self.name):
      return libcst.RemovalSentinel.REMOVE
    return updated_node


class GenerationalMacro[T](MacroBase[T]):
  def __call__(self, target: T) -> T:
    metadata = self.collect_metadata(libcst.parse_module(f"{target}"))
    self.cache[str(target)] = metadata
    return target

  def collect_metadata(self, node: libcst.CSTNode) -> MacroMetadata[T]:
    dep_collector = DependencyCollector()
    node.visit(dep_collector)
    return MacroMetadata(phase=MacroPhase.GENERATE, target=cast(T, node), dependencies=dep_collector.dependencies)

  def transform(self, node: libcst.CSTNode) -> libcst.CSTNode:
    template = self.jinja_env.get_template("generational_macro.j2")
    return libcst.parse_module(template.render(node=node))

  def generate_code(self) -> str:
    return libcst.parse_module("").code_for_node(self.transform(libcst.parse_module("")))

  def build_time_hook(self) -> None:
    for metadata in self.cache.values():
      self.remove_decorator(libcst.parse_module(f"{metadata.target}"))

  def runtime_hook(self) -> None:
    pass

  def remove_decorator(self, node: libcst.CSTNode) -> libcst.CSTNode | libcst.RemovalSentinel | libcst.FlattenSentinel[libcst.CSTNode]:
    return node.visit(RemoveDecoratorTransformer(self.__class__.__name__))
