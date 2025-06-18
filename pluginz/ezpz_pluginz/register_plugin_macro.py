import logging
from typing import TYPE_CHECKING, Unpack, Callable, Sequence, TypedDict, cast

import libcst as cst
import libcst.matchers as m
from pydantic import BaseModel, ConfigDict
from libcst.matchers import MatcherDecoratableTransformer
from painlezz_macroz.macroz.noop import class_macro
from painlezz_macroz.visitorz.macro_metadata_collector import MacroMetadataCollector

from ezpz_pluginz.e_polars_namespace import EPolarsNS
from ezpz_pluginz.polars_class_provider import PolarsClassProvider

if TYPE_CHECKING:
  from ezpz_pluginz.register_plugin_macro import PolarsPluginMacroMetadataPD


class PolarsPluginMacroKwargs(TypedDict):
  import_: str
  type_hint: str
  attr_name: str
  polars_ns: str


# purpose is to be recognized by painlezz_macroz (not an actual decorator)
def ezpz_plugin_collect[T](**kwargs: Unpack[PolarsPluginMacroKwargs]) -> Callable[[T], T]:
  return class_macro


class PolarsPluginMacroMetadataPD(BaseModel):
  model_config = ConfigDict(frozen=True, from_attributes=True)

  polars_ns: EPolarsNS
  import_: str
  attr_name: str
  type_hint: str

  def registery_entry(self) -> str:
    return f"pl.api.{self.polars_ns.api_decorator}('{self.attr_name}')({self.type_hint})"


# libsct visitor
class PolarsPluginCollector(MacroMetadataCollector[PolarsPluginMacroMetadataPD, PolarsPluginMacroKwargs]):
  def __init__(self) -> None:
    super().__init__(
      ezpz_plugin_collect.__name__,
      lambda _args, kwargs: PolarsPluginMacroMetadataPD(
        import_=kwargs["import_"],
        type_hint=kwargs["type_hint"],
        attr_name=kwargs["attr_name"],
        polars_ns=EPolarsNS(kwargs["polars_ns"]),
      ),
    )


logger = logging.getLogger(__name__)


# libcst transformer (modifies polars source code)
class PluginPatcher(MatcherDecoratableTransformer):
  METADATA_DEPENDENCIES = (PolarsClassProvider,)

  def __init__(self, polars_ns_to_plugins: dict[str, set["PolarsPluginMacroMetadataPD"]]) -> None:
    super().__init__()
    self.polars_ns_to_plugins = polars_ns_to_plugins

  def visit_Module(self, node: cst.Module) -> None:
    if (polars_ns := self.get_metadata(PolarsClassProvider, node, None)) is None:
      raise ValueError()
    self.polars_ns = polars_ns
    self.plugins = self.polars_ns_to_plugins[self.polars_ns]
    self.has_added_imports = False
    self.imports = [cst.parse_module(plugin.import_).body[0] for plugin in self.plugins]

  # called when libcst leaves a ClassDef node that matches a polars namespace
  @m.leave(m.ClassDef(name=m.Name(value=m.MatchIfTrue(lambda name: name in EPolarsNS))))
  def add_new_attrs(self, original_node: cst.ClassDef, updated_node: cst.ClassDef) -> cst.ClassDef:
    if original_node.name.value != self.polars_ns:
      raise Exception("PANIC")
    plugin_nodes = list[cst.AnnAssign]()
    for plugin in self.plugins:
      logger.info(f"Adding {plugin}")
      plugin_nodes.append(cst.AnnAssign(target=cst.Name(plugin.attr_name), annotation=cst.Annotation(cst.parse_expression(plugin.type_hint)), value=None))
    new_body = list(updated_node.body.body)
    new_body = new_body[:1] + [cst.SimpleStatementLine(body=plugin_nodes)] + new_body[1:]
    return updated_node.with_changes(body=cst.IndentedBlock(body=cast("Sequence[cst.BaseStatement]", new_body)))

  @m.leave(m.If(test=m.Name("TYPE_CHECKING")))
  def add_imports_to_type_checking(self, original_node: cst.If, updated_node: cst.If) -> cst.If:
    logger.info("Adding plugin imports...")
    self.has_added_imports = True
    return updated_node.with_changes(body=updated_node.body.with_changes(body=[*original_node.body.body, *self.imports]))
