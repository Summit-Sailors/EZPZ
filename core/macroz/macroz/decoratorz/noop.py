from typing import Callable, ParamSpec
from functools import wraps

M = ParamSpec("M")
P = ParamSpec("P")


def class_macro[T](cls: T) -> T:
  return cls


def func_macro[**P, R](func: Callable[P, R]) -> Callable[P, R]:
  @wraps(func)
  def noop_wrapper(*args: P.args, **kwargs: P.kwargs) -> R:
    return func(*args, **kwargs)

  return noop_wrapper
