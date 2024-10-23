import logging
from typing import ParamSpec, cast
from functools import wraps, lru_cache

import libcst

from painlezz_macroz.v2.base import MacroBase
from painlezz_macroz.v2.e_macro_kind import MacroPhase
from painlezz_macroz.v2.meta_data_dto import MacroMetadata

P = ParamSpec("P")

logger = logging.getLogger(__name__)


class RuntimeMacro[T](MacroBase[T]):
  def __call__(self, target: T) -> T:
    metadata = self.collect_metadata(libcst.parse_module(f"{target}"))
    self.cache[str(target)] = metadata
    return self.wrap_target(target)

  def collect_metadata(self, node: libcst.CSTNode) -> MacroMetadata[T]:
    dependencies = set[str]()

    class DependencyCollector(libcst.CSTVisitor):
      def visit_Annotation(self, node: libcst.Annotation) -> None:
        if isinstance(node.annotation, libcst.Name):
          dependencies.add(node.annotation.value)

    node.visit(DependencyCollector())
    return MacroMetadata(phase=MacroPhase.RUNTIME, target=cast(T, node), dependencies=dependencies)

  def transform(self, node: libcst.CSTNode) -> libcst.CSTNode:
    template = self.jinja_env.get_template("runtime_macro.j2")
    return libcst.parse_module(template.render(node=node))

  def generate_code(self) -> str:
    empty_module = libcst.parse_module("")
    return empty_module.code_for_node(self.transform(empty_module))

  def build_time_hook(self) -> None:
    pass

  def runtime_hook(self) -> None:
    for metadata in self.cache.values():
      self.wrap_target(metadata.target)

  @lru_cache(maxsize=None)
  def wrap_target(self, target: T) -> T:
    if callable(target):

      @wraps(target)
      def wrapper(*args: P.args, **kwargs: P.kwargs) -> T:
        # Resolve dependencies from the generated module
        return target(*args, **kwargs)

      return cast(T, wrapper)
    return target
