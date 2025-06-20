import ast
from typing import Any, Callable, Iterable, cast

import libcst as cst
import libcst.matchers as m
from pydantic import BaseModel
from libcst.display import dump

type JSONSerializable = str | int | float | bool | None | list[JSONSerializable] | dict[str, JSONSerializable]


type TMetadataCallback[T: BaseModel, TMacroKwargs: dict[str, JSONSerializable]] = Callable[[Iterable[JSONSerializable], TMacroKwargs], T]


class MacroMetadataCollector[T: BaseModel, TMacroKwargs: Any](m.MatcherDecoratableVisitor):  # we bound TMacroKwargs to Any to allow TypedDict
  callback: TMetadataCallback[T, TMacroKwargs]
  macro_data: list[T]
  macro_name: str

  def __init__(self, macro_name: str, callback: TMetadataCallback[T, TMacroKwargs] | None = None) -> None:
    super().__init__()
    if callback is None and not hasattr(self, "callback"):
      raise AttributeError("no callback method available")
    if callback is not None:
      self.callback = callback
    self.macro_name = macro_name
    self.macro_data = list[T]()

  @m.leave(m.Decorator(decorator=m.Call(func=m.Name())))
  def collect_macro_metadata(self, node: cst.Decorator) -> None:
    match node.decorator:
      case cst.Call(func=cst.Name(decorator_name), args=decorator_args) if decorator_name == self.macro_name:
        args: list[JSONSerializable] = []
        kwargs = cast("TMacroKwargs", {})
        for arg in decorator_args:
          # Extract the value from the argument, not the entire node
          evaled = ast.literal_eval(arg.value.value) if isinstance(arg.value, cst.SimpleString) else ast.literal_eval(dump(arg.value))

          if arg.keyword is None:
            args.append(evaled)
          else:
            kwargs[arg.keyword.value] = evaled

        # Move this outside the loop - we want one callback per decorator, not per argument
        self.macro_data.append(self.callback(args, kwargs))
      case _:
        pass
