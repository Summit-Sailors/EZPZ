import logging
from typing import TYPE_CHECKING

import libcst.matchers as m
from libcst.metadata import BatchableMetadataProvider

from ezpz_enumz import EPolarsNS

if TYPE_CHECKING:
  import libcst as cst

logger = logging.getLogger(__name__)


class PolarsClassProvider(BatchableMetadataProvider[str]):
  def __init__(self) -> None:
    logger.info("Finding Polars Class in file")
    super().__init__()
    self.polars_ns = None

  def visit_ClassDef(self, node: "cst.ClassDef") -> None:
    if m.matches(node, m.ClassDef(name=m.Name(value=m.MatchIfTrue(lambda name: name in EPolarsNS)))):
      self.polars_ns = node.name.value

  def leave_Module(self, original_node: "cst.Module") -> None:
    self.set_metadata(original_node, self.polars_ns)
