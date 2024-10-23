# This file is automatically generated by pyo3_stub_gen
# ruff: noqa: E501, F401

from typing import Self

import polars

from ezpz_pluginz.decorator import ezpz_plugin_collect

@ezpz_plugin_collect(polars_ns="DataFrame", attr_name="viewer", import_="from ezpz_guiz import _ezpz_guiz", type_hint="_ezpz_guiz.DataFrameViewer")
class DataFrameViewer:
  def __new__(cls, py_df: polars.DataFrame) -> Self: ...
  def view(self, window_title: str, width: int, height: int) -> None: ...

@ezpz_plugin_collect(
  polars_ns="LazyFrame", attr_name="ezprofiler", import_="from ezpz_pluginz.test_plugin import LazyPluginImpl", type_hint="_ezpz_guiz.LazyFrameProfileViewer"
)
class LazyFrameProfileViewer:
  def __new__(cls, py_lf: polars.LazyFrame) -> Self: ...
  def view(self, window_title: str, width: int, height: int) -> None: ...