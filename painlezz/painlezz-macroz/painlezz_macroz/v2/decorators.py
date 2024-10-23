from typing import TYPE_CHECKING, Callable

if TYPE_CHECKING:
  from painlezz_macroz.v2.base import MacroBase


def macro[T](*args: "MacroBase[T]") -> Callable[[T], T]:
  def decorator(target: T) -> T:
    for arg in args:
      target = arg(target)
    return target

  return decorator
