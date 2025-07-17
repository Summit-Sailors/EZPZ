# EZPZ Technical Analysis Polars Plugin

[![Rust](https://img.shields.io/badge/rust-1.88+-orange.svg)](https://rustlang.org)
[![Python](https://img.shields.io/badge/python-3.13+-blue.svg)](https://python.org)

A technical analysis library for Polars, powered by Rust. Get 70+ technical indicators seamlessly integrated into your Polars workflow with full type safety and good.

This plugin showcases how the [EZPZ](https://github.com/Summit-Sailors/EZPZ/tree/main/pluginz) plugins system works

## Features

- **Polars Native**: Seamlessly integrates with Polars DataFrames, LazyFrames and Series
- **70+ Indicators**: Comprehensive technical analysis toolkit
- **Type Safe**: Full type hints and IDE autocomplete support
- **Rust Powered**: Built on the [rust_ti](https://crates.io/crates/rust_ti) crate

## Installation

```bash
# Install EZPZ plugin system first
pip install ezpz_pluginz

# Install technical analysis plugin
pip install ezpz-rust-ti

# Mount the plugin
ezplugins mount
```

## Quick Start

```python
import numpy as np
import polars as pl

# Random seed for reproducibility
np.random.seed(42)

# Sample price data
n_periods = 100
base_price = 100.0
returns = np.random.normal(0, 0.02, n_periods)
prices = [base_price]
for ret in returns:
    prices.append(prices[-1] * (1 + ret))

# Sample Polars DataFrame
df = pl.DataFrame({
    "high": [p * (1 + abs(np.random.normal(0, 0.01))) for p in prices[1:]],
    "low": [p * (1 - abs(np.random.normal(0, 0.01))) for p in prices[1:]],
    "close": prices[1:],
    "volume": np.random.randint(1000, 10000, n_periods)
})

# Convert to LazyFrame for plugin operations
lf = df.lazy()

# Calculate single Ulcer Index
ulcer_single = lf.volatility_ti.ulcer_index_single("close")
print(f"Single Ulcer Index: {ulcer_single:.6f}")

# Calculate Ulcer Index series (bulk) with period=14
ulcer_series = lf.volatility_ti.ulcer_index_bulk("close", period=14)

# Integrate with Polars: Add Ulcer Index to DataFrame
result_df = (
    lf.collect()
    .with_columns(pl.Series("ulcer_index_14", [None] * (len(df) - len(ulcer_series)) + ulcer_series.to_list()))
    .select(["close", "ulcer_index_14"])
)

print("\nDataFrame with Ulcer Index:")
print(result_df.head(10))
```

## Available Attributes

### `basic_ti` - Basic Technical Indicators (Exposes methods from the BasicTI class)

```python
class BasicTI:
  def __new__(cls, lf: polars.LazyFrame) -> BasicTI: ...
  def mean_single(self, column: builtins.str) -> builtins.float:
    r"""
    Calculate the arithmetic mean of all values.
    """
  def median_single(self, column: builtins.str) -> builtins.float:
    r"""
    Calculate the median of all values.
    """
  def mode_single(self, column: builtins.str) -> builtins.float:
    r"""
    Calculate the mode of all values.
    """
  def variance_single(self, column: builtins.str) -> builtins.float:
    r"""
    Calculate the variance of all values.
    """
  def standard_deviation_single(self, column: builtins.str) -> builtins.float:
    r"""
    Calculate the standard deviation of all values.
    """
  def max_single(self, column: builtins.str) -> builtins.float:
    r"""
    Find the maximum value.
    """
  def min_single(self, column: builtins.str) -> builtins.float:
    r"""
    Find the minimum value.
    """
  def absolute_deviation_single(self, column: builtins.str, central_point: builtins.str) -> builtins.float:
    r"""
    Calculate the absolute deviation from a central point.
    """
  def log_difference_single(self, price_t: builtins.float, price_t_1: builtins.float) -> builtins.float:
    r"""
    Calculate the logarithmic difference between two price points.
    """
  def mean_bulk(self, column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Calculate rolling mean over a specified period.
    """
  def median_bulk(self, column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Calculate rolling median over a specified period.
    """
  def mode_bulk(self, column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Calculate rolling mode over a specified period.
    """
  def variance_bulk(self, column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Calculate rolling variance over a specified period.
    """
  def standard_deviation_bulk(self, column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Calculate rolling standard deviation over a specified period.
    """
  def absolute_deviation_bulk(self, column: builtins.str, period: builtins.int, central_point: builtins.str) -> polars.Series:
    r"""
    Calculate rolling absolute deviation over a specified period.
    """
  def log_bulk(self, column: builtins.str) -> polars.Series:
    r"""
    Calculate natural logarithm of all values.
    """
  def log_difference_bulk(self, column: builtins.str) -> polars.Series:
    r"""
    Calculate logarithmic differences between consecutive values.
    """
```

### `candle_ti` - Candle Pattern Analysis (Exposes methods from the CandleTI class)

```python
class CandleTI:
  def __new__(cls, lf: polars.LazyFrame) -> CandleTI: ...
  def moving_constant_envelopes_single(self, price_column: builtins.str, constant_model_type: builtins.str, difference: builtins.float) -> polars.DataFrame:
    r"""
    Moving Constant Envelopes - Creates upper and lower bands from moving constant of price
    """
  def mcginley_dynamic_envelopes_single(
    self, price_column: builtins.str, difference: builtins.float, previous_mcginley_dynamic: builtins.float
  ) -> polars.DataFrame:
    r"""
    McGinley Dynamic Envelopes - Variation of moving constant envelopes using McGinley Dynamic
    """
  def moving_constant_bands_single(
    self, price_column: builtins.str, constant_model_type: builtins.str, deviation_model: builtins.str, deviation_multiplier: builtins.float
  ) -> polars.DataFrame:
    r"""
    Moving Constant Bands - Extended Bollinger Bands with configurable models
    """
  def mcginley_dynamic_bands_single(
    self, price_column: builtins.str, deviation_model: builtins.str, deviation_multiplier: builtins.float, previous_mcginley_dynamic: builtins.float
  ) -> polars.DataFrame:
    r"""
    McGinley Dynamic Bands - Variation of moving constant bands using McGinley Dynamic
    """
  def ichimoku_cloud_single(
    self,
    high_column: builtins.str,
    low_column: builtins.str,
    close_column: builtins.str,
    conversion_period: builtins.int,
    base_period: builtins.int,
    span_b_period: builtins.int,
  ) -> polars.DataFrame:
    r"""
    Ichimoku Cloud - Calculates support and resistance levels
    """
  def donchian_channels_single(self, high_column: builtins.str, low_column: builtins.str) -> polars.DataFrame:
    r"""
    Donchian Channels - Produces bands from period highs and lows
    """
  def keltner_channel_single(
    self,
    high_column: builtins.str,
    low_column: builtins.str,
    close_column: builtins.str,
    constant_model_type: builtins.str,
    atr_constant_model_type: builtins.str,
    multiplier: builtins.float,
  ) -> polars.DataFrame:
    r"""
    Keltner Channel - Bands based on moving average and average true range
    """
  def supertrend_single(
    self, high_column: builtins.str, low_column: builtins.str, close_column: builtins.str, constant_model_type: builtins.str, multiplier: builtins.float
  ) -> polars.Series:
    r"""
    Supertrend - Trend indicator showing support and resistance levels
    """
  def moving_constant_envelopes_bulk(
    self, price_column: builtins.str, constant_model_type: builtins.str, difference: builtins.float, period: builtins.int
  ) -> polars.DataFrame:
    r"""
    Moving Constant Envelopes (Bulk) - Returns envelopes over time periods
    """
  def mcginley_dynamic_envelopes_bulk(
    self, price_column: builtins.str, difference: builtins.float, previous_mcginley_dynamic: builtins.float, period: builtins.int
  ) -> polars.DataFrame:
    r"""
    Mcginley dynamic envelopes
    """
  def moving_constant_bands_bulk(
    self,
    price_column: builtins.str,
    constant_model_type: builtins.str,
    deviation_model: builtins.str,
    deviation_multiplier: builtins.float,
    period: builtins.int,
  ) -> polars.DataFrame:
    r"""
    Moving Constant Bands (Bulk)
    """
  def mcginley_dynamic_bands_bulk(
    self,
    price_column: builtins.str,
    deviation_model: builtins.str,
    deviation_multiplier: builtins.float,
    previous_mcginley_dynamic: builtins.float,
    period: builtins.int,
  ) -> polars.DataFrame:
    r"""
    McGinley Dynamic Bands (Bulk)
    """
  def ichimoku_cloud_bulk(
    self,
    high_column: builtins.str,
    low_column: builtins.str,
    close_column: builtins.str,
    conversion_period: builtins.int,
    base_period: builtins.int,
    span_b_period: builtins.int,
  ) -> polars.DataFrame:
    r"""
    Ichimoku Cloud (Bulk) - Returns ichimoku components over time
    """
  def donchian_channels_bulk(self, high_column: builtins.str, low_column: builtins.str, period: builtins.int) -> polars.DataFrame:
    r"""
    Donchian Channels (Bulk) - Returns donchian bands over time
    """
  def keltner_channel_bulk(
    self,
    high_column: builtins.str,
    low_column: builtins.str,
    close_column: builtins.str,
    constant_model_type: builtins.str,
    atr_constant_model_type: builtins.str,
    multiplier: builtins.float,
    period: builtins.int,
  ) -> polars.DataFrame:
    r"""
    Keltner Channel (Bulk) - Returns keltner bands over time
    """
  def supertrend_bulk(
    self,
    high_column: builtins.str,
    low_column: builtins.str,
    close_column: builtins.str,
    constant_model_type: builtins.str,
    multiplier: builtins.float,
    period: builtins.int,
  ) -> polars.Series:
    r"""
    Supertrend (Bulk) - Returns supertrend values over time
    """
```

### `chart_trends_ti` - Chart Trend Analysis (Exposes methods from the ChartTrendsTI class)

```python
class ChartTrendsTI:
  def __new__(cls, lf: polars.LazyFrame) -> ChartTrendsTI: ...
  def peaks(self, price_column: builtins.str, period: builtins.int, closest_neighbor: builtins.int) -> builtins.list[tuple[builtins.float, builtins.int]]:
    r"""
    Find peaks in a price series over a given period
    """
  def valleys(self, price_column: builtins.str, period: builtins.int, closest_neighbor: builtins.int) -> builtins.list[tuple[builtins.float, builtins.int]]:
    r"""
    Find valleys in a price series over a given period
    """
  def peak_trend(self, price_column: builtins.str, period: builtins.int) -> tuple[builtins.float, builtins.float]:
    r"""
    Calculate peak trend (linear regression on peaks)
    """
  def valley_trend(self, price_column: builtins.str, period: builtins.int) -> tuple[builtins.float, builtins.float]:
    r"""
    Calculate valley trend (linear regression on valleys)
    """
  def overall_trend(self, price_column: builtins.str) -> tuple[builtins.float, builtins.float]:
    r"""
    Calculate overall trend (linear regression on all prices)
    """
  def break_down_trends(
    self,
    price_column: builtins.str,
    max_outliers: builtins.int,
    soft_r_squared_minimum: builtins.float,
    soft_r_squared_maximum: builtins.float,
    hard_r_squared_minimum: builtins.float,
    hard_r_squared_maximum: builtins.float,
    soft_standard_error_multiplier: builtins.float,
    hard_standard_error_multiplier: builtins.float,
    soft_reduced_chi_squared_multiplier: builtins.float,
    hard_reduced_chi_squared_multiplier: builtins.float,
  ) -> builtins.list[tuple[builtins.int, builtins.int, builtins.float, builtins.float]]:
    r"""
    Break down trends in a price series
    """
```

### `correlation_ti` - Correlation Analysis (Exposes methods from the CorrelationTI class)

```python
class CorrelationTI:
  def __new__(cls, lf: polars.LazyFrame) -> CorrelationTI: ...
  def correlate_asset_prices_single(
    self, price_column_a: builtins.str, price_column_b: builtins.str, constant_model_type: builtins.str, deviation_model: builtins.str
  ) -> builtins.float:
    r"""
    Correlation between two assets - Single value calculation
    Calculates correlation between prices of two assets using specified models
    Returns a single correlation value for the entire price series
    """
  def correlate_asset_prices_bulk(
    self, price_column_a: builtins.str, price_column_b: builtins.str, constant_model_type: builtins.str, deviation_model: builtins.str, period: builtins.int
  ) -> polars.Series:
    r"""
    Correlation between two assets - Rolling/Bulk calculation
    Calculates rolling correlation between prices of two assets using specified models
    Returns a series of correlation values for each period window
    """
```

### `ma_ti` - Moving Averages (Exposes methods from the MATI class)

```python
class MATI:
  def __new__(cls, lf: polars.LazyFrame) -> MATI: ...
  def moving_average_single(self, price_column: builtins.str, moving_average_type: builtins.str) -> builtins.float:
    r"""
    Moving Average (Single) - Calculates a single moving average value for a series of prices
    """
  def moving_average_bulk(self, price_column: builtins.str, moving_average_type: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Moving Average (Bulk) - Calculates moving averages over a rolling window
    """
  def mcginley_dynamic_single(self, price_column: builtins.str, previous_mcginley_dynamic: builtins.float, period: builtins.int) -> builtins.float:
    r"""
    McGinley Dynamic (Single) - Calculates a single McGinley Dynamic value
    """
  def mcginley_dynamic_bulk(self, price_column: builtins.str, previous_mcginley_dynamic: builtins.float, period: builtins.int) -> polars.Series:
    r"""
    McGinley Dynamic (Bulk) - Calculates McGinley Dynamic values over a series
    """
  def personalised_moving_average_single(
    self, price_column: builtins.str, alpha_nominator: builtins.float, alpha_denominator: builtins.float
  ) -> builtins.float:
    r"""
    Personalised Moving Average (Single) - Calculates a single personalised moving average
    """
  def personalised_moving_average_bulk(
    self, price_column: builtins.str, alpha_nominator: builtins.float, alpha_denominator: builtins.float, period: builtins.int
  ) -> polars.Series:
    r"""
    Personalised Moving Average (Bulk) - Calculates personalised moving averages over a rolling window
    """
```

### `momentum_ti` - Momentum Indicators (Exposes methods from the MomentumTI class)

```python
class MomentumTI:
  def __new__(cls, lf: polars.LazyFrame) -> MomentumTI: ...
  def aroon_up_single(self, high_column: builtins.str) -> builtins.float:
    r"""
    Aroon Up indicator
    """
  def aroon_down_single(self, low_column: builtins.str) -> builtins.float:
    r"""
    Aroon Down indicator

    Calculates the Aroon Down indicator, which measures the time since the lowest low
    within a given period as a percentage.
    """
  def aroon_oscillator_single(self, aroon_up: builtins.float, aroon_down: builtins.float) -> builtins.float:
    r"""
    Aroon Oscillator

    Calculates the Aroon Oscillator by subtracting Aroon Down from Aroon Up.
    Values range from -100 to +100, indicating trend strength and direction.
    """
  def aroon_indicator_single(self, high_column: builtins.str, low_column: builtins.str) -> tuple[builtins.float, builtins.float, builtins.float]:
    r"""
    Aroon Indicator (complete calculation)

    Calculates all three Aroon components: Aroon Up, Aroon Down, and Aroon Oscillator
    in a single function call.
    """
  def long_parabolic_time_price_system_single(
    self, previous_sar: builtins.float, extreme_point: builtins.float, acceleration_factor: builtins.float, low: builtins.float
  ) -> builtins.float:
    r"""
    Long Parabolic Time Price System (Parabolic SAR for long positions)

    Calculates the Parabolic SAR (Stop and Reverse) for long positions, used to determine
    potential reversal points in price movement.
    """
  def short_parabolic_time_price_system_single(
    self, previous_sar: builtins.float, extreme_point: builtins.float, acceleration_factor: builtins.float, high: builtins.float
  ) -> builtins.float:
    r"""
    Short Parabolic Time Price System (Parabolic SAR for short positions)

    Calculates the Parabolic SAR (Stop and Reverse) for short positions, used to determine
    potential reversal points in price movement.
    """
  def volume_price_trend_single(
    self, price_column: builtins.str, previous_price: builtins.float, volume: builtins.float, previous_volume_price_trend: builtins.float
  ) -> builtins.float:
    r"""
    Volume Price Trend

    Calculates the Volume Price Trend indicator, which combines price and volume
    to show the relationship between volume and price changes.
    """
  def true_strength_index_single(
    self, price_column: builtins.str, first_constant_model: builtins.str, first_period: builtins.int, second_constant_model: builtins.str
  ) -> builtins.float:
    r"""
    True Strength Index

    Calculates the True Strength Index, a momentum oscillator that uses price changes
    smoothed by two exponential moving averages.
    """
  def relative_strength_index_bulk(self, price_column: builtins.str, constant_model_type: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Relative Strength Index (RSI) - bulk calculation

    Calculates RSI values for an entire series of prices. RSI measures the speed and change
    of price movements, oscillating between 0 and 100.
    """
  def stochastic_oscillator_bulk(self, price_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Stochastic Oscillator - bulk calculation

    Calculates the Stochastic Oscillator, which compares a security's closing price
    to its price range over a given time period.
    """
  def slow_stochastic_bulk(self, stochastic_column: builtins.str, constant_model_type: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Slow Stochastic - bulk calculation

    Calculates the Slow Stochastic by smoothing the regular Stochastic Oscillator
    to reduce noise and false signals.
    """
  def slowest_stochastic_bulk(self, slow_stochastic_column: builtins.str, constant_model_type: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Slowest Stochastic - bulk calculation

    Calculates the Slowest Stochastic by applying additional smoothing to the Slow Stochastic
    for even more noise reduction.
    """
  def williams_percent_r_bulk(self, high_column: builtins.str, low_column: builtins.str, close_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Williams %R - bulk calculation

    Calculates Williams %R, a momentum indicator that measures overbought and oversold levels.
    Values range from -100 to 0, where -20 and above indicates overbought, -80 and below indicates oversold.
    """
  def money_flow_index_bulk(self, price_column: builtins.str, volume_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Money Flow Index - bulk calculation

    Calculates the Money Flow Index, a volume-weighted RSI that measures buying and selling pressure.
    Values range from 0 to 100, where >80 indicates overbought and <20 indicates oversold.
    """
  def rate_of_change_bulk(self, price_column: builtins.str) -> polars.Series:
    r"""
    Rate of Change - bulk calculation

    Calculates the Rate of Change, which measures the percentage change in price
    from one period to the next.
    """
  def on_balance_volume_bulk(self, price_column: builtins.str, volume_column: builtins.str, previous_obv: builtins.float) -> polars.Series:
    r"""
    On Balance Volume (Bulk) - Calculates cumulative volume indicator
    Adds volume on up days and subtracts volume on down days to measure buying and selling pressure
    """
  def commodity_channel_index_bulk(
    self,
    price_column: builtins.str,
    constant_model_type: builtins.str,
    deviation_model: builtins.str,
    constant_multiplier: builtins.float,
    period: builtins.int,
  ) -> polars.Series:
    r"""
    Commodity Channel Index (Bulk) - Calculates CCI over rolling periods
    Measures the variation of a security's price from its statistical mean
    Values typically range from -100 to +100
    """
  def mcginley_dynamic_commodity_channel_index_bulk(
    self,
    price_column: builtins.str,
    previous_mcginley_dynamic: builtins.float,
    deviation_model: builtins.str,
    constant_multiplier: builtins.float,
    period: builtins.int,
  ) -> tuple[polars.Series, polars.Series]:
    r"""
    McGinley Dynamic Commodity Channel Index (Bulk) - CCI using McGinley Dynamic MA
    Uses McGinley Dynamic as the moving average, which adapts to market conditions
    better than traditional moving averages
    """
  def macd_line_bulk(
    self, price_column: builtins.str, short_period: builtins.int, short_period_model: builtins.str, long_period: builtins.int, long_period_model: builtins.str
  ) -> polars.Series:
    r"""
    MACD Line (Bulk) - Calculates Moving Average Convergence Divergence line
    Subtracts the long-period moving average from the short-period moving average
    """
  def signal_line_bulk(self, macd_column: builtins.str, constant_model_type: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Signal Line (Bulk) - Calculates MACD Signal Line
    Applies a moving average to the MACD line for generating buy/sell signals
    """
  def mcginley_dynamic_macd_line_bulk(
    self,
    price_column: builtins.str,
    short_period: builtins.int,
    previous_short_mcginley: builtins.float,
    long_period: builtins.int,
    previous_long_mcginley: builtins.float,
  ) -> polars.DataFrame:
    r"""
    McGinley Dynamic MACD Line (Bulk) - MACD using McGinley Dynamic moving averages
    Provides better adaptation to market volatility and reduces lag compared to traditional MACD
    """
  def chaikin_oscillator_bulk(
    self,
    high_column: builtins.str,
    low_column: builtins.str,
    close_column: builtins.str,
    volume_column: builtins.str,
    short_period: builtins.int,
    long_period: builtins.int,
    previous_accumulation_distribution: builtins.float,
    short_period_model: builtins.str,
    long_period_model: builtins.str,
  ) -> tuple[polars.Series, polars.Series]:
    r"""
    Chaikin Oscillator (Bulk) - Applies MACD to Accumulation/Distribution line
    Measures the momentum of the Accumulation/Distribution line
    """
  def percentage_price_oscillator_bulk(
    self, price_column: builtins.str, short_period: builtins.int, long_period: builtins.int, constant_model_type: builtins.str
  ) -> polars.Series:
    r"""
    Percentage Price Oscillator (Bulk) - MACD expressed as percentage
    Similar to MACD but expressed as a percentage for easier comparison across securities
    """
  def chande_momentum_oscillator_bulk(self, price_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Chande Momentum Oscillator (Bulk) - Measures momentum using gains and losses
    Calculates the difference between sum of gains and losses over a period
    Values range from -100 to +100
    """
```

### `other_ti` - Other Technical Indicators (Exposes methods from the OtherTI class)

```python
class OtherTI:
  r"""
  Other Technical Indicators - A collection of other analysis functions for financial data
  """
  def __new__(cls, lf: polars.LazyFrame) -> OtherTI: ...
  def return_on_investment_single(self, price_column: builtins.str, investment: builtins.float) -> tuple[builtins.float, builtins.float]:
    r"""
    Return on Investment - Calculates investment value and percentage change for a single period
    Uses the first and last values from the price column as start and end prices
    """
  def return_on_investment_bulk(self, price_column: builtins.str, investment: builtins.float) -> tuple[polars.Series, polars.Series]:
    r"""
    Return on Investment Bulk - Calculates ROI for a series of consecutive price periods
    Uses the price column as price values for consecutive period calculations
    """
  def true_range(self, high_column: builtins.str, low_column: builtins.str, close_column: builtins.str) -> polars.Series:
    r"""
    True Range - Calculates the greatest price movement for a single period
    Uses the provided high/low/close columns to calculate true range
    """
  def average_true_range_single(
    self, high_column: builtins.str, low_column: builtins.str, close_column: builtins.str, constant_model_type: builtins.str
  ) -> builtins.float:
    r"""
    Average True Range - Calculates the moving average of true range values for a single result
    Uses the provided high/low/close columns to calculate ATR from the entire price series
    """
  def average_true_range_bulk(
    self, high_column: builtins.str, low_column: builtins.str, close_column: builtins.str, constant_model_type: builtins.str, period: builtins.int
  ) -> polars.Series:
    r"""
    Average True Range Bulk - Calculates rolling ATR values over specified periods
    Uses the provided high/low/close columns for rolling ATR calculations
    """
  def internal_bar_strength(self, high_column: builtins.str, low_column: builtins.str, close_column: builtins.str) -> polars.Series:
    r"""
    Internal Bar Strength - Calculates buy/sell oscillator based on close position within high-low range
    Uses the provided high/low/close columns to calculate IBS values
    """
  def positivity_indicator(
    self, open_column: builtins.str, close_column: builtins.str, signal_period: builtins.int, constant_model_type: builtins.str
  ) -> tuple[polars.Series, polars.Series]:
    r"""
    Positivity Indicator - Generates trading signals based on open vs previous close comparison
    Uses the provided open/close columns for signal generation
    """
```

### `std_ti` - Standard Technical Indicators (Exposes methods from the StandardTI class)

```python
class StandardTI:
  def __new__(cls, lf: polars.LazyFrame) -> StandardTI: ...
  def sma_single(self, price_column: builtins.str) -> builtins.float:
    r"""
    Simple Moving Average (Single) - calculates the mean of all values in the column
    """
  def sma_bulk(self, price_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Simple Moving Average (Bulk) - calculates the mean over a rolling window
    """
  def smma_single(self, price_column: builtins.str) -> builtins.float:
    r"""
    Smoothed Moving Average (Single) - single value calculation
    """
  def smma_bulk(self, price_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Smoothed Moving Average (Bulk) - puts more weight on recent prices
    """
  def ema_single(self, price_column: builtins.str) -> builtins.float:
    r"""
    Exponential Moving Average (Single) - single value calculation
    """
  def ema_bulk(self, price_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Exponential Moving Average (Bulk) - puts exponentially more weight on recent prices
    """
  def bollinger_bands_single(self, price_column: builtins.str) -> tuple[builtins.float, builtins.float, builtins.float]:
    r"""
    Bollinger Bands (Single) - single value calculation (requires exactly 20 periods)
    """
  def bollinger_bands_bulk(self, price_column: builtins.str) -> polars.DataFrame:
    r"""
    Bollinger Bands (Bulk) - returns three series: lower band, middle (SMA), upper band
    Standard period is 20 with 2 standard deviations
    """
  def macd_single(self, price_column: builtins.str) -> tuple[builtins.float, builtins.float, builtins.float]:
    r"""
    MACD (Single) - single value calculation (requires exactly 34 periods)
    """
  def macd_bulk(self, price_column: builtins.str) -> polars.DataFrame:
    r"""
    MACD (Bulk) - Moving Average Convergence Divergence
    Returns three series: MACD line, Signal line, Histogram
    Standard periods: 12, 26, 9
    """
  def rsi_single(self, price_column: builtins.str) -> builtins.float:
    r"""
    RSI (Single) - single value calculation (requires exactly 14 periods)
    """
  def rsi_bulk(self, price_column: builtins.str) -> polars.Series:
    r"""
    RSI (Bulk) - Relative Strength Index
    Standard period is 14 using smoothed moving average
    """
```

### `strength_ti` - Strength Indicators (Exposes methods from the StrengthTI class)

```python
class StrengthTI:
  def __new__(cls, lf: polars.LazyFrame) -> StrengthTI: ...
  def accumulation_distribution_single(
    self,
    high_column: builtins.str,
    low_column: builtins.str,
    close_column: builtins.str,
    volume_column: builtins.str,
    previous_ad: typing.Optional[builtins.float],
  ) -> builtins.float:
    r"""
    Accumulation Distribution (Single) - Shows whether the stock is being accumulated or distributed
    Single value calculation using the last available values
    """
  def accumulation_distribution_bulk(
    self,
    high_column: builtins.str,
    low_column: builtins.str,
    close_column: builtins.str,
    volume_column: builtins.str,
    previous_ad: typing.Optional[builtins.float],
  ) -> polars.Series:
    r"""
    Accumulation Distribution (Bulk) - Shows whether the stock is being accumulated or distributed
    Returns a series of accumulation/distribution values
    """
  def positive_volume_index_single(
    self, close_column: builtins.str, volume_column: builtins.str, previous_pvi: typing.Optional[builtins.float]
  ) -> builtins.float:
    r"""
    Positive Volume Index (Single) - Measures volume trend strength when volume increases
    Single value calculation using the last available values
    """
  def positive_volume_index_bulk(self, close_column: builtins.str, volume_column: builtins.str, previous_pvi: typing.Optional[builtins.float]) -> polars.Series:
    r"""
    Positive Volume Index (Bulk) - Measures volume trend strength when volume increases
    Returns a series of positive volume index values
    """
  def negative_volume_index_single(
    self, close_column: builtins.str, volume_column: builtins.str, previous_nvi: typing.Optional[builtins.float]
  ) -> builtins.float:
    r"""
    Negative Volume Index (Single) - Measures volume trend strength when volume decreases
    Single value calculation using the last available values
    """
  def negative_volume_index_bulk(self, close_column: builtins.str, volume_column: builtins.str, previous_nvi: typing.Optional[builtins.float]) -> polars.Series:
    r"""
    Negative Volume Index (Bulk) - Measures volume trend strength when volume decreases
    Returns a series of negative volume index values
    """
  def relative_vigor_index_single(
    self, open_column: builtins.str, high_column: builtins.str, low_column: builtins.str, close_column: builtins.str, constant_model_type: builtins.str
  ) -> builtins.float:
    r"""
    Relative Vigor Index (Single) - Measures the strength of an asset by looking at previous prices
    Single value calculation using all available values
    """
  def relative_vigor_index_bulk(
    self,
    open_column: builtins.str,
    high_column: builtins.str,
    low_column: builtins.str,
    close_column: builtins.str,
    constant_model_type: builtins.str,
    period: builtins.int,
  ) -> polars.Series:
    r"""
    Relative Vigor Index (Bulk) - Measures the strength of an asset by looking at previous prices
    Returns a series of relative vigor index values
    """
```

### `trend_ti` - Trend Indicators (Exposes methods from the TrendTI class)

```python
class TrendTI:
  r"""
  Trend Technical Indicators - A collection of trend analysis functions for financial data
  """
  def __new__(cls, lf: polars.LazyFrame) -> TrendTI: ...
  def aroon_up_single(self, high_column: builtins.str) -> builtins.float:
    r"""
    Aroon Up (Single) - Measures the strength of upward price momentum
    Calculates the percentage of time since the highest high within the series
    """
  def aroon_down_single(self, low_column: builtins.str) -> builtins.float:
    r"""
    Aroon Down (Single) - Measures the strength of downward price momentum
    Calculates the percentage of time since the lowest low within the series
    """
  def aroon_oscillator_single(self, high_column: builtins.str, low_column: builtins.str) -> builtins.float:
    r"""
    Aroon Oscillator (Single) - Calculates the difference between Aroon Up and Aroon Down
    Provides a single measure of trend direction and strength
    """
  def aroon_indicator_single(self, high_column: builtins.str, low_column: builtins.str) -> tuple[builtins.float, builtins.float, builtins.float]:
    r"""
    Aroon Indicator (Single) - Calculates complete Aroon system in one call
    Computes Aroon Up, Aroon Down, and Aroon Oscillator
    """
  def true_strength_index_single(
    self, price_column: builtins.str, first_constant_model: builtins.str, first_period: builtins.int, second_constant_model: builtins.str
  ) -> builtins.float:
    r"""
    True Strength Index (Single) - Momentum oscillator using double-smoothed price changes
    Filters out price noise to provide clearer momentum signals
    """
  def aroon_up_bulk(self, high_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Aroon Up (Bulk) - Calculates rolling Aroon Up indicator over specified period
    Measures upward momentum strength for each period in the time series
    """
  def aroon_down_bulk(self, low_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Aroon Down (Bulk) - Calculates rolling Aroon Down indicator over specified period
    Measures downward momentum strength for each period in the time series
    """
  def aroon_oscillator_bulk(self, high_column: builtins.str, low_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Aroon Oscillator (Bulk) - Calculates rolling Aroon Oscillator over specified period
    Computes the difference between Aroon Up and Aroon Down for each period
    """
  def aroon_indicator_bulk(self, high_column: builtins.str, low_column: builtins.str, period: builtins.int) -> polars.DataFrame:
    r"""
    Aroon Indicator (Bulk) - Calculates complete Aroon system for time series data
    Computes Aroon Up, Aroon Down, and Aroon Oscillator for each period
    """
  def parabolic_time_price_system_bulk(
    self,
    high_column: builtins.str,
    low_column: builtins.str,
    acceleration_factor_start: builtins.float,
    acceleration_factor_max: builtins.float,
    acceleration_factor_step: builtins.float,
    start_position: builtins.str,
    previous_sar: builtins.float,
  ) -> polars.Series:
    r"""
    Parabolic Time Price System (Bulk) - Calculates Stop and Reverse points
    Provides trailing stop levels for trend-following system
    """
  def directional_movement_system_bulk(
    self, high_column: builtins.str, low_column: builtins.str, close_column: builtins.str, period: builtins.int, constant_model_type: builtins.str
  ) -> polars.DataFrame:
    r"""
    Directional Movement System (Bulk) - Calculates complete DMS indicators
    Computes +DI, -DI, ADX, and ADXR for trend strength analysis
    """
  def volume_price_trend_bulk(self, price_column: builtins.str, volume_column: builtins.str, previous_volume_price_trend: builtins.float) -> polars.Series:
    r"""
    Volume Price Trend (Bulk) - Combines price and volume to show momentum
    Shows the relationship between price movement and volume flow
    """
  def true_strength_index_bulk(
    self,
    price_column: builtins.str,
    first_constant_model: builtins.str,
    first_period: builtins.int,
    second_constant_model: builtins.str,
    second_period: builtins.int,
  ) -> polars.Series:
    r"""
    True Strength Index (Bulk) - Double-smoothed momentum oscillator
    Uses double-smoothed price changes to filter noise and provide clearer signals
    """
```

### `volatility_ti` - Volatility Indicators (Exposes methods from the VolatilityTI class)

```python
class VolatilityTI:
  def __new__(cls, lf: polars.LazyFrame) -> VolatilityTI: ...
  def ulcer_index_single(self, price_column: builtins.str) -> builtins.float:
    r"""
    Ulcer Index (Single) - Calculates how quickly the price is able to get back to its former high
    Can be used instead of standard deviation for volatility measurement
    """
  def ulcer_index_bulk(self, price_column: builtins.str, period: builtins.int) -> polars.Series:
    r"""
    Ulcer Index (Bulk) - Calculates rolling Ulcer Index over specified period
    Returns a series of Ulcer Index values
    """
  def volatility_system(
    self,
    high_column: builtins.str,
    low_column: builtins.str,
    close_column: builtins.str,
    period: builtins.int,
    constant_multiplier: builtins.float,
    constant_model_type: builtins.str,
  ) -> polars.Series:
    r"""
    Volatility System - Calculates Welles volatility system with Stop and Reverse (SaR) points
    Uses trend analysis to determine long/short positions and calculate SaR levels
    Constant multiplier typically between 2.8-3.1 (Welles used 3.0)
    """
```

## Note

For more detailed API documentation, view the [stub_file](python/ezpz_rust_ti/_ezpz_rust_ti.pyi)

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## Acknowledgments

- [Polars](https://pola.rs/) - The amazing DataFrame library
- [PyO3](https://pyo3.rs/) - Rust bindings for Python
- [rust_ti](https://crates.io/crates/rust_ti) - Technical analysis algorithms
