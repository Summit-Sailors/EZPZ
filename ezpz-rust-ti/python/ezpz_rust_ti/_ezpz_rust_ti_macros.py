from ezpz_rust_ti._ezpz_rust_ti import MATI, BasicTI, OtherTI, TrendTI, CandleTI, MomentumTI, StandardTI, StrengthTI, VolatilityTI, ChartTrendsTI, CorrelationTI
from ezpz_pluginz.register_plugin_macro import ezpz_plugin_collect

# Basic Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="basic_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import BasicTI", type_hint="BasicTI")(BasicTI)

# Candle Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="candle_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import CandleTI", type_hint="CandleTI")(CandleTI)

# Chart Trends Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="chart_trends_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import ChartTrendsTI", type_hint="ChartTrendsTI")(
  ChartTrendsTI
)

# Correlation Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="correlation_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import CorrelationTI", type_hint="CorrelationTI")(
  CorrelationTI
)

# Moving Average Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="ma_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import MATI", type_hint="MATI")(MATI)

# Momentum Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="momentum_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import MomentumTI", type_hint="MomentumTI")(
  MomentumTI
)

# Other Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="other_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import OtherTI", type_hint="OtherTI")(OtherTI)

# Standard Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="standard_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import StandardTI", type_hint="StandardTI")(
  StandardTI
)

# Strength Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="strength_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import StrengthTI", type_hint="StrengthTI")(
  StrengthTI
)

# Trend Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="trend_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import TrendTI", type_hint="TrendTI")(TrendTI)

# Volatility Technical Indicators
ezpz_plugin_collect(polars_ns="Series", attr_name="volatility_ti", import_="from ezpz_rust_ti._ezpz_rust_ti import VolatilityTI", type_hint="VolatilityTI")(
  VolatilityTI
)
