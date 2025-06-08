from ezpz_guiz._ezpz_guiz import DataFrameViewer, LazyFrameViewer
from ezpz_pluginz.register_plugin_macro import ezpz_plugin_collect

ezpz_plugin_collect(polars_ns="DataFrame", attr_name="viewer", import_="from ezpz_guiz import _ezpz_guiz", type_hint="_ezpz_guiz.DataFrameViewer")(
  DataFrameViewer
)

ezpz_plugin_collect(
  polars_ns="LazyFrame", attr_name="ezprofiler", import_="from ezpz_pluginz.test_plugin import LazyPluginImpl", type_hint="_ezpz_guiz.LazyFrameProfileViewer"
)(LazyFrameViewer)
