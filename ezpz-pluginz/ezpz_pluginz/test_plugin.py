from ezpz_pluginz.register_plugin_macro import ezpz_plugin_collect


@ezpz_plugin_collect(
  polars_ns="LazyFrame", attr_name="plugin_namespace", import_="from ezpz_pluginz.test_plugin import LazyPluginImpl", type_hint="LazyPluginImpl"
)
class LazyPluginImpl:
  pass


@ezpz_plugin_collect(
  polars_ns="DataFrame", attr_name="not_lazy_plugin_namespace", import_="from ezpz_pluginz.test_plugin import PluginImpl", type_hint="PluginImpl"
)
class PluginImpl:
  pass
