import logging
from typing import TYPE_CHECKING, Callable, ParamSpec

if TYPE_CHECKING:
  from painlezz_macroz.v2.base import MacroBase

P = ParamSpec("P")

logger = logging.getLogger(__name__)


class MacroRegistry:
  @staticmethod
  def register[T](macro: "MacroBase[T]") -> Callable[[T], T]:
    def decorator(target: T) -> T:
      return macro(target)

    return decorator
