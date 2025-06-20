from ezpz_rust_ti._ezpz_rust_ti import (
  MATI as RustMATI,
  BasicTI as RustBasicTI,
  OtherTI as RustOtherTI,
  TrendTI as RustTrendTI,
  CandleTI as RustCandleTI,
  MomentumTI as RustMomentumTI,
  StandardTI as RustStandardTI,
  StrengthTI as RustStrengthTI,
  VolatilityTI as RustVolatilityTI,
  ChartTrendsTI as RustChartTrendsTI,
  CorrelationTI as RustCorrelationTI,
)
from ezpz_pluginz.register_plugin_macro import ezpz_plugin_collect


# Basic Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="basic_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import BasicTI", type_hint="BasicTI")
class BasicTI(RustBasicTI):
  pass


# Candle Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="candle_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import CandleTI", type_hint="CandleTI")
class CandleTI(RustCandleTI):
  pass


# Chart Trends Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="chart_trends_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import ChartTrendsTI", type_hint="ChartTrendsTI")
class ChartTrendsTI(RustChartTrendsTI):
  pass


# Correlation Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="correlation_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import CorrelationTI", type_hint="CorrelationTI")
class CorrelationTI(RustCorrelationTI):
  pass


# Moving Average Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="ma_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import MATI", type_hint="MATI")
class MATI(RustMATI):
  pass


# Momentum Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="momentum_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import MomentumTI", type_hint="MomentumTI")
class MomentumTI(RustMomentumTI):
  pass


# Other Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="other_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import OtherTI", type_hint="OtherTI")
class OtherTI(RustOtherTI):
  pass


# Standard Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="standard_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import StandardTI", type_hint="StandardTI")
class StandardTI(RustStandardTI):
  pass


# Strength Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="strength_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import StrengthTI", type_hint="StrengthTI")
class StrengthTI(RustStrengthTI):
  pass


# Trend Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="trend_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import TrendTI", type_hint="TrendTI")
class TrendTI(RustTrendTI):
  pass


# Volatility Technical Indicators
@ezpz_plugin_collect(polars_ns="Series", attr_name="volatility_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import VolatilityTI", type_hint="VolatilityTI")
class VolatilityTI(RustVolatilityTI):
  pass
