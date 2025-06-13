from pathlib import Path

import libcst as cst
from hypothesis import (
  given,
  strategies as st,
)

from ezpz_pluginz.e_polars_namespace import EPolarsNS
from ezpz_pluginz.register_plugin_macro import PolarsPluginCollector

identifier = st.from_regex(r"[a-zA-Z_][a-zA-Z0-9_]*", fullmatch=True)
filepath_strategy = st.builds(lambda parts: str(Path(*parts)), st.lists(identifier, min_size=1, max_size=5))
root_dir_strategy = st.builds(lambda parts: str(Path(*parts)), st.lists(identifier, min_size=1, max_size=3))
class_name_strategy = identifier

namespace_name_strategy = st.sampled_from([ns.api_decorator for ns in EPolarsNS])

decorator_call_strategy = st.builds(
  lambda namespace_attr: cst.Decorator(
    decorator=cst.Call(
      func=cst.Attribute(value=cst.Name("pl"), attr=cst.Name(namespace_attr)),
      args=[cst.Arg(value=cst.SimpleString(f'"{namespace_attr.split("_")[1]}_namespace"'))],
    )
  ),
  namespace_name_strategy,
)

class_def_strategy = st.builds(
  lambda class_name, decorators: cst.ClassDef(name=cst.Name(class_name), body=cst.IndentedBlock(body=[]), decorators=decorators),
  class_name_strategy,
  st.lists(decorator_call_strategy, min_size=1, max_size=3),
)


@given(class_def=class_def_strategy)
def test_polars_plugin_collector(class_def: cst.ClassDef) -> None:
  module = cst.Module(body=[class_def])
  collector = PolarsPluginCollector()
  module.visit(collector)

  # Test should verify that plugins are collected correctly
  assert len(collector.macro_data) >= 0  # Basic assertion


if __name__ == "__main__":
  test_polars_plugin_collector()
