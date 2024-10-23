import logging
from abc import ABCMeta, abstractmethod
from typing import TYPE_CHECKING, Callable

import libcst
from networkx import DiGraph

if TYPE_CHECKING:
  from painlezz_macroz.v2.meta_data_dto import MacroMetadata

logger = logging.getLogger(__name__)

type TDecoratableNode = libcst.FunctionDef | libcst.ClassDef

type TMacroTransformer[T: TDecoratableNode] = Callable[[T], T]


class MacroBase[TTarget](metaclass=ABCMeta):
  def __init__(self) -> None:
    self.cache: dict[str, MacroMetadata[TTarget]] = {}
    self.dependency_graph = DiGraph()

  @abstractmethod
  def __call__(self, target: TTarget) -> TTarget: ...

  @abstractmethod
  def collect_metadata(self, node: TDecoratableNode) -> "MacroMetadata[TTarget]": ...

  @abstractmethod
  def transform(self, node: TDecoratableNode) -> TDecoratableNode: ...

  @abstractmethod
  def generate_code(self) -> str: ...

  @abstractmethod
  def build_time_hook(self) -> None: ...

  @abstractmethod
  def runtime_hook(self) -> None: ...

  def analyze_dependencies(self) -> None:
    for target, metadata in self.cache.items():
      self.dependency_graph.add_node(target)
      for dep in metadata.dependencies:
        self.dependency_graph.add_edge(target, dep)
