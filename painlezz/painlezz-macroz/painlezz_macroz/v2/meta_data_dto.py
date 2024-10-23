import logging
from typing import TYPE_CHECKING
from dataclasses import dataclass

if TYPE_CHECKING:
  from painlezz_macroz.v2.e_macro_kind import MacroPhase

logger = logging.getLogger(__name__)


@dataclass
class MacroMetadata[T]:
  phase: "MacroPhase"
  target: T
  dependencies: set[str]
