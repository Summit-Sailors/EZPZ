import logging
from typing import TYPE_CHECKING

from painlezz_macroz.v2.build_macro import BuildtimeMacro

if TYPE_CHECKING:
  from painlezz_macroz.v2.base import MacroBase


logger = logging.getLogger(__name__)


class HatchlingPlugin:
  def __init__[T](self, macros: list["MacroBase[T]"]) -> None:
    self.macros = macros

  def hook(self) -> None:
    for macro in self.macros:
      if isinstance(macro, BuildtimeMacro):
        macro.build_time_hook()
