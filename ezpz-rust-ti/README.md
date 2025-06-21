# EZPZ Technical Analysis Polars Plugin

[![Rust](https://img.shields.io/badge/rust-1.88+-orange.svg)](https://rustlang.org)
[![Python](https://img.shields.io/badge/python-3.13+-blue.svg)](https://python.org)

A technical analysis library for Polars, powered by Rust. Get 70+ technical indicators seamlessly integrated into your Polars workflow with full type safety and exceptional performance.

This plugin showcases how the [EZPZ](https://github.com/Summit-Sailors/EZPZ/tree/main/pluginz) plugins system works

## Features

- **Polars Native**: Seamlessly integrates with Polars DataFrames, LazyFrames and Series
- **70+ Indicators**: Comprehensive technical analysis toolkit
- **Type Safe**: Full type hints and IDE autocomplete support
- **Rust Powered**: Built on the high-performance [rust_ti](https://crates.io/crates/rust_ti) crate

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
from datetime import date

import polars as pl

df = pl.select(
  timestamp=pl.date_range(start=date(2023, 1, 1), end=date(2023, 12, 31), interval="1d"),
).with_columns(
  [
    pl.Series("open", [100 + i * 0.1 for i in range(365)]),
    pl.Series("high", [101 + i * 0.1 for i in range(365)]),
    pl.Series("low", [99 + i * 0.1 for i in range(365)]),
    pl.Series("close", [100.5 + i * 0.1 for i in range(365)]),
    pl.Series("volume", [1000 + i * 10 for i in range(365)]),
  ]
)

print(f"DataFrame shape: {df.shape}")
print(df.head())

# Get the close price series
close = df["close"]

# Calculate technical indicators - it's that simple!
sma_20 = pl.Series.standard_ti.sma_bulk(close, 20)  # Simple Moving Average
print(f"SMA(20) last 5 values: {sma_20.tail(5)}")
```

## Available Attributes

### `basic_ti` - Basic Technical Indicators (Exposes methods from the BasicTI class)

```python
class BasicTI:
  @staticmethod
  def mean_single(prices: polars.Series) -> float:
    r"""
    Calculate the arithmetic mean of all values.
    """
  @staticmethod
  def median_single(prices: polars.Series) -> float:
    r"""
    Calculate the median of all values.
    """
  @staticmethod
  def mode_single(prices: polars.Series) -> float:
    r"""
    Calculate the mode of all values.
    """
  @staticmethod
  def variance_single(prices: polars.Series) -> float:
    r"""
    Calculate the variance of all values.
    """
  @staticmethod
  def standard_deviation_single(prices: polars.Series) -> float:
    r"""
    Calculate the standard deviation of all values.
    """
  @staticmethod
  def max_single(prices: polars.Series) -> float:
    r"""
    Find the maximum value.
    """
  @staticmethod
  def min_single(prices: polars.Series) -> float:
    r"""
    Find the minimum value.
    """
  @staticmethod
  def absolute_deviation_single(prices: polars.Series, central_point: str) -> float:
    r"""
    Calculate the absolute deviation from a central point.
    """
  @staticmethod
  def log_difference_single(price_t: float, price_t_1: float) -> float:
    r"""
    Calculate the logarithmic difference between two price points.
    """
  @staticmethod
  def mean_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Calculate rolling mean over a specified period.
    """
  @staticmethod
  def median_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Calculate rolling median over a specified period.
    """
  @staticmethod
  def mode_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Calculate rolling mode over a specified period.
    """
  @staticmethod
  def variance_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Calculate rolling variance over a specified period.
    """
  @staticmethod
  def standard_deviation_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Calculate rolling standard deviation over a specified period.
    """
  @staticmethod
  def absolute_deviation_bulk(prices: polars.Series, period: int, central_point: str) -> polars.Series:
    r"""
    Calculate rolling absolute deviation over a specified period.
    """
  @staticmethod
  def log_bulk(prices: polars.Series) -> polars.Series:
    r"""
    Calculate natural logarithm of all values.
    """
  @staticmethod
  def log_difference_bulk(prices: polars.Series) -> polars.Series:
    r"""
    Calculate logarithmic differences between consecutive values.
    """
```

### `candle_ti` - Candle Pattern Analysis (Exposes methods from the CandleTI class)

```python
class CandleTI:
  @staticmethod
  def moving_constant_envelopes_single(prices: polars.Series, constant_model_type: str, difference: float) -> polars.DataFrame:
    r"""
    Moving Constant Envelopes - Creates upper and lower bands from moving constant of price
    """
  @staticmethod
  def mcginley_dynamic_envelopes_single(prices: polars.Series, difference: float, previous_mcginley_dynamic: float) -> polars.DataFrame:
    r"""
    McGinley Dynamic Envelopes - Variation of moving constant envelopes using McGinley Dynamic
    """
  @staticmethod
  def moving_constant_bands_single(
    prices: polars.Series, constant_model_type: str, deviation_model: str, deviation_multiplier: float
  ) -> polars.DataFrame:
    r"""
    Moving Constant Bands - Extended Bollinger Bands with configurable models
    """
  @staticmethod
  def mcginley_dynamic_bands_single(
    prices: polars.Series, deviation_model: str, deviation_multiplier: float, previous_mcginley_dynamic: float
  ) -> polars.DataFrame:
    r"""
    McGinley Dynamic Bands - Variation of moving constant bands using McGinley Dynamic
    """
  @staticmethod
  def ichimoku_cloud_single(
    highs: polars.Series, lows: polars.Series, close: polars.Series, conversion_period: int, base_period: int, span_b_period: int
  ) -> polars.DataFrame:
    r"""
    Ichimoku Cloud - Calculates support and resistance levels
    """
  @staticmethod
  def donchian_channels_single(highs: polars.Series, lows: polars.Series) -> polars.DataFrame:
    r"""
    Donchian Channels - Produces bands from period highs and lows
    """
  @staticmethod
  def keltner_channel_single(
    highs: polars.Series,
    lows: polars.Series,
    close: polars.Series,
    constant_model_type: str,
    atr_constant_model_type: str,
    multiplier: float,
  ) -> polars.DataFrame:
    r"""
    Keltner Channel - Bands based on moving average and average true range
    """
  @staticmethod
  def supertrend_single(
    highs: polars.Series, lows: polars.Series, close: polars.Series, constant_model_type: str, multiplier: float
  ) -> polars.Series:
    r"""
    Supertrend - Trend indicator showing support and resistance levels
    """
  @staticmethod
  def moving_constant_envelopes_bulk(
    prices: polars.Series, constant_model_type: str, difference: float, period: int
  ) -> polars.DataFrame:
    r"""
    Moving Constant Envelopes (Bulk) - Returns envelopes over time periods
    """
  @staticmethod
  def mcginley_dynamic_envelopes_bulk(
    prices: polars.Series, difference: float, previous_mcginley_dynamic: float, period: int
  ) -> polars.DataFrame:
    r"""
    McGinley Dynamic Envelopes (Bulk)
    """
  @staticmethod
  def moving_constant_bands_bulk(
    prices: polars.Series, constant_model_type: str, deviation_model: str, deviation_multiplier: float, period: int
  ) -> polars.DataFrame:
    r"""
    Moving Constant Bands (Bulk)
    """
  @staticmethod
  def mcginley_dynamic_bands_bulk(
    prices: polars.Series, deviation_model: str, deviation_multiplier: float, previous_mcginley_dynamic: float, period: int
  ) -> polars.DataFrame:
    r"""
    McGinley Dynamic Bands (Bulk)
    """
  @staticmethod
  def ichimoku_cloud_bulk(
    highs: polars.Series, lows: polars.Series, closes: polars.Series, conversion_period: int, base_period: int, span_b_period: int
  ) -> polars.DataFrame:
    r"""
    Ichimoku Cloud (Bulk) - Returns ichimoku components over time
    """
  @staticmethod
  def donchian_channels_bulk(highs: polars.Series, lows: polars.Series, period: int) -> polars.DataFrame:
    r"""
    Donchian Channels (Bulk) - Returns donchian bands over time
    """
  @staticmethod
  def keltner_channel_bulk(
    highs: polars.Series,
    lows: polars.Series,
    closes: polars.Series,
    constant_model_type: str,
    atr_constant_model_type: str,
    multiplier: float,
    period: int,
  ) -> polars.DataFrame:
    r"""
    Keltner Channel (Bulk) - Returns keltner bands over time
    """
  @staticmethod
  def supertrend_bulk(
    highs: polars.Series, lows: polars.Series, closes: polars.Series, constant_model_type: str, multiplier: float, period: int
  ) -> polars.Series:
    r"""
    Supertrend (Bulk) - Returns supertrend values over time
    """
```

### `chart_trends_ti` - Chart Trend Analysis (Exposes methods from the ChartTrendsTI class)

```python
class ChartTrendsTI:
  @staticmethod
  def peaks(prices: polars.Series, period: int, closest_neighbor: int) -> list[tuple[float, int]]:
    r"""
    Find peaks in a price series over a given period
    """
  @staticmethod
  def valleys(prices: polars.Series, period: int, closest_neighbor: int) -> list[tuple[float, int]]:
    r"""
    Find valleys in a price series over a given period
    """
  @staticmethod
  def peak_trend(prices: polars.Series, period: int) -> tuple[float, float]:
    r"""
    Calculate peak trend (linear regression on peaks)
    """
  @staticmethod
  def valley_trend(prices: polars.Series, period: int) -> tuple[float, float]:
    r"""
    Calculate valley trend (linear regression on valleys)
    """
  @staticmethod
  def overall_trend(prices: polars.Series) -> tuple[float, float]:
    r"""
    Calculate overall trend (linear regression on all prices)
    """
  @staticmethod
  def break_down_trends(
    prices: polars.Series,
    max_outliers: int,
    soft_r_squared_minimum: float,
    soft_r_squared_maximum: float,
    hard_r_squared_minimum: float,
    hard_r_squared_maximum: float,
    soft_standard_error_multiplier: float,
    hard_standard_error_multiplier: float,
    soft_reduced_chi_squared_multiplier: float,
    hard_reduced_chi_squared_multiplier: float,
  ) -> list[tuple[int, int, float, float]]:
    r"""
    Break down trends in a price series
    """
```

### `correlation_ti` - Correlation Analysis (Exposes methods from the CorrelationTI class)

```python
class CorrelationTI:
  @staticmethod
  def correlate_asset_prices_single(
    prices_asset_a: polars.Series, prices_asset_b: polars.Series, constant_model_type: str, deviation_model: str
  ) -> float:
    r"""
    Correlation between two assets - Single value calculation
    Calculates correlation between prices of two assets using specified models
    Returns a single correlation value for the entire price series
    """
  @staticmethod
  def correlate_asset_prices_bulk(
    prices_asset_a: polars.Series, prices_asset_b: polars.Series, constant_model_type: str, deviation_model: str, period: int
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
  @staticmethod
  def moving_average_single(prices: polars.Series, moving_average_type: str) -> polars.Series:
    r"""
    Moving Average (Single) - Calculates a single moving average value for a series of prices
    """
  @staticmethod
  def moving_average_bulk(prices: polars.Series, moving_average_type: str, period: int) -> polars.Series:
    r"""
    Moving Average (Bulk) - Calculates moving averages over a rolling window
    """
  @staticmethod
  def mcginley_dynamic_single(latest_price: float, previous_mcginley_dynamic: float, period: int) -> polars.Series:
    r"""
    McGinley Dynamic (Single) - Calculates a single McGinley Dynamic value
    """
  @staticmethod
  def mcginley_dynamic_bulk(prices: polars.Series, previous_mcginley_dynamic: float, period: int) -> polars.Series:
    r"""
    McGinley Dynamic (Bulk) - Calculates McGinley Dynamic values over a series
    """
  @staticmethod
  def personalised_moving_average_single(prices: polars.Series, alpha_nominator: float, alpha_denominator: float) -> polars.Series:
    r"""
    Personalised Moving Average (Single) - Calculates a single personalised moving average
    """
  @staticmethod
  def personalised_moving_average_bulk(
    prices: polars.Series, alpha_nominator: float, alpha_denominator: float, period: int
  ) -> polars.Series:
    r"""
    Personalised Moving Average (Bulk) - Calculates personalised moving averages over a rolling window
    """
```

### `momentum_ti` - Momentum Indicators (Exposes methods from the MomentumTI class)

```python
class MomentumTI:
  r"""
  Momentum Technical Indicators - A collection of momentum analysis functions for financial data
  """
  @staticmethod
  def aroon_up_single(highs: polars.Series) -> float:
    r"""
    Aroon Up indicator
    """
  @staticmethod
  def aroon_down_single(lows: polars.Series) -> float:
    r"""
    Aroon Down indicator

    Calculates the Aroon Down indicator, which measures the time since the lowest low
    within a given period as a percentage.
    """
  @staticmethod
  def aroon_oscillator_single(aroon_up: float, aroon_down: float) -> float:
    r"""
    Aroon Oscillator

    Calculates the Aroon Oscillator by subtracting Aroon Down from Aroon Up.
    Values range from -100 to +100, indicating trend strength and direction.
    """
  @staticmethod
  def aroon_indicator_single(highs: polars.Series, lows: polars.Series) -> tuple[float, float, float]:
    r"""
    Aroon Indicator (complete calculation)

    Calculates all three Aroon components: Aroon Up, Aroon Down, and Aroon Oscillator
    in a single function call.
    """
  @staticmethod
  def long_parabolic_time_price_system_single(
    previous_sar: float, extreme_point: float, acceleration_factor: float, low: float
  ) -> float:
    r"""
    Long Parabolic Time Price System (Parabolic SAR for long positions)

    Calculates the Parabolic SAR (Stop and Reverse) for long positions, used to determine
    potential reversal points in price movement.
    """
  @staticmethod
  def short_parabolic_time_price_system_single(
    previous_sar: float, extreme_point: float, acceleration_factor: float, high: float
  ) -> float:
    r"""
    Short Parabolic Time Price System (Parabolic SAR for short positions)

    Calculates the Parabolic SAR (Stop and Reverse) for short positions, used to determine
    potential reversal points in price movement.
    """
  @staticmethod
  def volume_price_trend_single(
    current_price: float, previous_price: float, volume: float, previous_volume_price_trend: float
  ) -> float:
    r"""
    Volume Price Trend

    Calculates the Volume Price Trend indicator, which combines price and volume
    to show the relationship between volume and price changes.
    """
  @staticmethod
  def true_strength_index_single(
    prices: polars.Series, first_constant_model: str, first_period: int, second_constant_model: str
  ) -> float:
    r"""
    True Strength Index

    Calculates the True Strength Index, a momentum oscillator that uses price changes
    smoothed by two exponential moving averages.
    """
  @staticmethod
  def relative_strength_index_bulk(prices: polars.Series, constant_model_type: str, period: int) -> polars.Series:
    r"""
    Relative Strength Index (RSI) - bulk calculation

    Calculates RSI values for an entire series of prices. RSI measures the speed and change
    of price movements, oscillating between 0 and 100.
    """
  @staticmethod
  def stochastic_oscillator_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Stochastic Oscillator - bulk calculation

    Calculates the Stochastic Oscillator, which compares a security's closing price
    to its price range over a given time period.
    """
  @staticmethod
  def slow_stochastic_bulk(stochastics: polars.Series, constant_model_type: str, period: int) -> polars.Series:
    r"""
    Slow Stochastic - bulk calculation

    Calculates the Slow Stochastic by smoothing the regular Stochastic Oscillator
    to reduce noise and false signals.
    """
  @staticmethod
  def slowest_stochastic_bulk(slow_stochastics: polars.Series, constant_model_type: str, period: int) -> polars.Series:
    r"""
    Slowest Stochastic - bulk calculation

    Calculates the Slowest Stochastic by applying additional smoothing to the Slow Stochastic
    for even more noise reduction.
    """
  @staticmethod
  def williams_percent_r_bulk(high: polars.Series, low: polars.Series, close: polars.Series, period: int) -> polars.Series:
    r"""
    Williams %R - bulk calculation

    Calculates Williams %R, a momentum indicator that measures overbought and oversold levels.
    Values range from -100 to 0, where -20 and above indicates overbought, -80 and below indicates oversold.
    """
  @staticmethod
  def money_flow_index_bulk(prices: polars.Series, volume: polars.Series, period: int) -> polars.Series:
    r"""
    Money Flow Index - bulk calculation

    Calculates the Money Flow Index, a volume-weighted RSI that measures buying and selling pressure.
    Values range from 0 to 100, where >80 indicates overbought and <20 indicates oversold.
    """
  @staticmethod
  def rate_of_change_bulk(prices: polars.Series) -> polars.Series:
    r"""
    Rate of Change - bulk calculation

    Calculates the Rate of Change, which measures the percentage change in price
    from one period to the next.
    """
  @staticmethod
  def on_balance_volume_bulk(prices: polars.Series, volume: polars.Series, previous_obv: float) -> polars.Series:
    r"""
    On Balance Volume - bulk calculation

    Calculates On Balance Volume, a cumulative volume indicator that adds volume on up days
    and subtracts volume on down days to measure buying and selling pressure.
    """
  @staticmethod
  def commodity_channel_index_bulk(
    prices: polars.Series, constant_model_type: str, deviation_model: str, constant_multiplier: float, period: int
  ) -> polars.Series:
    r"""
    Commodity Channel Index - bulk calculation

    Calculates the Commodity Channel Index, which measures the variation of a security's price
    from its statistical mean. Values typically range from -100 to +100.
    """
  @staticmethod
  def mcginley_dynamic_commodity_channel_index_bulk(
    prices: polars.Series, previous_mcginley_dynamic: float, deviation_model: str, constant_multiplier: float, period: int
  ) -> tuple[polars.Series, polars.Series]:
    r"""
    McGinley Dynamic Commodity Channel Index - bulk calculation

    Calculates CCI using McGinley Dynamic as the moving average, which adapts to market conditions
    better than traditional moving averages.
    """
  @staticmethod
  def macd_line_bulk(
    prices: polars.Series, short_period: int, short_period_model: str, long_period: int, long_period_model: str
  ) -> polars.Series:
    r"""
    MACD Line - bulk calculation

    Calculates the MACD (Moving Average Convergence Divergence) line by subtracting
    the long-period moving average from the short-period moving average.
    """
  @staticmethod
  def signal_line_bulk(macds: polars.Series, constant_model_type: str, period: int) -> polars.Series:
    r"""
    Signal Line - bulk calculation

    Calculates the MACD Signal Line by applying a moving average to the MACD line.
    Used to generate buy/sell signals when MACD crosses above or below the signal line.
    """
  @staticmethod
  def mcginley_dynamic_macd_line_bulk(
    prices: polars.Series,
    short_period: int,
    previous_short_mcginley: float,
    long_period: int,
    previous_long_mcginley: float,
  ) -> polars.DataFrame:
    r"""
    McGinley Dynamic MACD Line - bulk calculation

    Calculates MACD using McGinley Dynamic moving averages instead of traditional MAs,
    providing better adaptation to market volatility and reducing lag.
    """
  @staticmethod
  def chaikin_oscillator_bulk(
    highs: polars.Series,
    lows: polars.Series,
    close: polars.Series,
    volume: polars.Series,
    short_period: int,
    long_period: int,
    previous_accumulation_distribution: float,
    short_period_model: str,
    long_period_model: str,
  ) -> tuple[polars.Series, polars.Series]:
    r"""
    Chaikin Oscillator - bulk calculation

    Calculates the Chaikin Oscillator, which applies MACD to the Accumulation/Distribution line
    to measure the momentum of the Accumulation/Distribution line.
    """
  @staticmethod
  def percentage_price_oscillator_bulk(
    prices: polars.Series, short_period: int, long_period: int, constant_model_type: str
  ) -> polars.Series:
    r"""
    Percentage Price Oscillator - bulk calculation

    Calculates the Percentage Price Oscillator, which is similar to MACD but expressed as a percentage.
    This makes it easier to compare securities with different price levels.
    """
  @staticmethod
  def chande_momentum_oscillator_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Chande Momentum Oscillator - bulk calculation

    Calculates the Chande Momentum Oscillator, which measures momentum by calculating
    the difference between the sum of gains and losses over a given period.
    Values range from -100 to +100.
    """
```

### `other_ti` - Other Technical Indicators (Exposes methods from the OtherTI class)

```python
class OtherTI:
  r"""
  Other Technical Indicators - A collection of other analysis functions for financial data
  """
  @staticmethod
  def return_on_investment_single(start_price: float, end_price: float, investment: float) -> tuple[float, float]:
    r"""
    Return on Investment - Calculates investment value and percentage change for a single period
    """
  @staticmethod
  def return_on_investment_bulk(prices: polars.Series, investment: float) -> tuple[polars.Series, polars.Series]:
    r"""
    Return on Investment Bulk - Calculates ROI for a series of consecutive price periods
    """
  @staticmethod
  def true_range_single(close: float, high: float, low: float) -> float:
    r"""
    True Range - Calculates the greatest price movement for a single period
    """
  @staticmethod
  def true_range_bulk(close: polars.Series, high: polars.Series, low: polars.Series) -> polars.Series:
    r"""
    True Range Bulk - Calculates true range for a series of OHLC data
    """
  @staticmethod
  def average_true_range_single(close: polars.Series, high: polars.Series, low: polars.Series, constant_model_type: str) -> float:
    r"""
    Average True Range - Calculates the moving average of true range values for a single result
    """
  @staticmethod
  def average_true_range_bulk(
    close: polars.Series, high: polars.Series, low: polars.Series, constant_model_type: str, period: int
  ) -> polars.Series:
    r"""
    Average True Range Bulk - Calculates rolling ATR values over specified periods
    """
  @staticmethod
  def internal_bar_strength_single(high: float, low: float, close: float) -> float:
    r"""
    Internal Bar Strength - Calculates buy/sell oscillator based on close position within high-low range
    """
  @staticmethod
  def internal_bar_strength_bulk(high: polars.Series, low: polars.Series, close: polars.Series) -> polars.Series:
    r"""
    Internal Bar Strength Bulk - Calculates IBS for a series of OHLC data
    """
  @staticmethod
  def positivity_indicator(
    open: polars.Series, previous_close: polars.Series, signal_period: int, constant_model_type: str
  ) -> tuple[polars.Series, polars.Series]:
    r"""
    Positivity Indicator - Generates trading signals based on open vs previous close comparison
    """
```

### `std_ti` - Standard Technical Indicators (Exposes methods from the StandardTI class)

```python
class StandardTI:
  @staticmethod
  def sma_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Simple Moving Average - calculates the mean over a rolling window
    """
  @staticmethod
  def smma_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Smoothed Moving Average - puts more weight on recent prices
    """
  @staticmethod
  def ema_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Exponential Moving Average - puts exponentially more weight on recent prices
    """
  @staticmethod
  def bollinger_bands_bulk(prices: polars.Series) -> polars.DataFrame:
    r"""
    Bollinger Bands - returns three series: lower band, middle (SMA), upper band
    Standard period is 20 with 2 standard deviations
    """
  @staticmethod
  def macd_bulk(prices: polars.Series) -> polars.DataFrame:
    r"""
    MACD - Moving Average Convergence Divergence
    Returns three series: MACD line, Signal line, Histogram
    Standard periods: 12, 26, 9
    """
  @staticmethod
  def rsi_bulk(prices: polars.Series) -> polars.Series:
    r"""
    RSI - Relative Strength Index
    Standard period is 14 using smoothed moving average
    """
  @staticmethod
  def sma_single(prices: polars.Series) -> float:
    r"""
    Simple Moving Average - single value calculation
    """
  @staticmethod
  def smma_single(prices: polars.Series) -> float:
    r"""
    Smoothed Moving Average - single value calculation
    """
  @staticmethod
  def ema_single(prices: polars.Series) -> float:
    r"""
    Exponential Moving Average - single value calculation
    """
  @staticmethod
  def bollinger_bands_single(prices: polars.Series) -> tuple[float, float, float]:
    r"""
    Bollinger Bands - single value calculation (requires exactly 20 periods)
    """
  @staticmethod
  def macd_single(prices: polars.Series) -> tuple[float, float, float]:
    r"""
    MACD - single value calculation (requires exactly 34 periods)
    """
  @staticmethod
  def rsi_single(prices: polars.Series) -> float:
    r"""
    RSI - single value calculation (requires exactly 14 periods)
    """
```

### `strength_ti` - Strength Indicators (Exposes methods from the StrengthTI class)

```python
class StrengthTI:
  @staticmethod
  def accumulation_distribution(
    high: polars.Series, low: polars.Series, close: polars.Series, volume: polars.Series, previous_ad: float | None
  ) -> polars.Series:
    r"""
    Accumulation Distribution - Shows whether the stock is being accumulated or distributed
    """
  @staticmethod
  def positive_volume_index(close: polars.Series, volume: polars.Series, previous_pvi: float | None) -> polars.Series:
    r"""
    Positive Volume Index - Measures volume trend strength when volume increases
    """
  @staticmethod
  def negative_volume_index(close: polars.Series, volume: polars.Series, previous_nvi: float | None) -> polars.Series:
    r"""
    Negative Volume Index - Measures volume trend strength when volume decreases
    """
  @staticmethod
  def relative_vigor_index(
    open: polars.Series, high: polars.Series, low: polars.Series, close: polars.Series, constant_model_type: str, period: int
  ) -> polars.Series:
    r"""
    Relative Vigor Index - Measures the strength of an asset by looking at previous prices
    """
  @staticmethod
  def single_accumulation_distribution(
    high: float, low: float, close: float, volume: float, previous_ad: float | None
  ) -> float:
    r"""
    Single Accumulation Distribution - Single value calculation
    """
  @staticmethod
  def single_volume_index(current_close: float, previous_close: float, previous_volume_index: float | None) -> float:
    r"""
    Single Volume Index - Generic version of PVI and NVI for single calculation
    """
  @staticmethod
  def single_relative_vigor_index(
    open: polars.Series, high: polars.Series, low: polars.Series, close: polars.Series, constant_model_type: str
  ) -> float:
    r"""
    Single Relative Vigor Index - Single value calculation
    """
```

### `trend_ti` - Trend Indicators (Exposes methods from the TrendTI class)

```python
class TrendTI:
  r"""
  Trend Technical Indicators - A collection of trend analysis functions for financial data
  """
  @staticmethod
  def aroon_up_single(highs: polars.Series) -> float:
    r"""
    Calculate Aroon Up indicator for a single value

    The Aroon Up indicator measures the strength of upward price momentum by calculating
    the percentage of time since the highest high within the given period.
    """
  @staticmethod
  def aroon_down_single(lows: polars.Series) -> float:
    r"""
    Calculate Aroon Down indicator for a single value

    The Aroon Down indicator measures the strength of downward price momentum by calculating
    the percentage of time since the lowest low within the given period.
    """
  @staticmethod
  def aroon_oscillator_single(aroon_up: float, aroon_down: float) -> float:
    r"""
    Calculate Aroon Oscillator from Aroon Up and Aroon Down values

    The Aroon Oscillator is the difference between Aroon Up and Aroon Down indicators,
    providing a single measure of trend direction and strength.
    """
  @staticmethod
  def aroon_indicator_single(highs: polars.Series, lows: polars.Series) -> tuple[float, float, float]:
    r"""
    Calculate complete Aroon Indicator (Up, Down, and Oscillator) for single values

    Computes all three Aroon components in one call: Aroon Up, Aroon Down, and Aroon Oscillator.
    """
  @staticmethod
  def long_parabolic_time_price_system_single(
    previous_sar: float, extreme_point: float, acceleration_factor: float, low: float
  ) -> float:
    r"""
    Calculate Parabolic SAR for long positions (single value)

    Computes the Stop and Reverse point for long positions in the Parabolic Time/Price System.
    """
  @staticmethod
  def short_parabolic_time_price_system_single(
    previous_sar: float, extreme_point: float, acceleration_factor: float, high: float
  ) -> float:
    r"""
    Calculate Parabolic SAR for short positions (single value)

    Computes the Stop and Reverse point for short positions in the Parabolic Time/Price System.
    """
  @staticmethod
  def volume_price_trend_single(
    current_price: float, previous_price: float, volume: float, previous_volume_price_trend: float
  ) -> float:
    r"""
    Calculate Volume Price Trend indicator (single value)

    VPT combines price and volume to show the relationship between a security's price movement and volume.
    """
  @staticmethod
  def true_strength_index_single(
    prices: polars.Series, first_constant_model: str, first_period: int, second_constant_model: str
  ) -> float:
    r"""
    Calculate True Strength Index (single value)

    TSI is a momentum oscillator that uses moving averages of price changes to filter out price noise.
    """
  @staticmethod
  def aroon_up_bulk(highs: polars.Series, period: int) -> polars.Series:
    r"""
    Calculate Aroon Up indicator for time series data

    Computes Aroon Up values for each period in the time series, measuring upward momentum strength.
    """
  @staticmethod
  def aroon_down_bulk(lows: polars.Series, period: int) -> polars.Series:
    r"""
    Calculate Aroon Down indicator for time series data

    Computes Aroon Down values for each period in the time series, measuring downward momentum strength.
    """
  @staticmethod
  def aroon_oscillator_bulk(aroon_up: polars.Series, aroon_down: polars.Series) -> polars.Series:
    r"""
    Calculate Aroon Oscillator for time series data

    Computes the difference between Aroon Up and Aroon Down for each period.
    """
  @staticmethod
  def aroon_indicator_bulk(highs: polars.Series, lows: polars.Series, period: int) -> polars.DataFrame:
    r"""
    Calculate complete Aroon Indicator system for time series data

    Computes Aroon Up, Aroon Down, and Aroon Oscillator for each period in one operation.
    """
  @staticmethod
  def parabolic_time_price_system_bulk(
    highs: polars.Series,
    lows: polars.Series,
    acceleration_factor_start: float,
    acceleration_factor_max: float,
    acceleration_factor_step: float,
    start_position: str,
    previous_sar: float,
  ) -> polars.Series:
    r"""
    Calculate Parabolic Time Price System (SAR) for time series data

    Computes Stop and Reverse points for trend-following system that provides trailing stop levels.
    """
  @staticmethod
  def directional_movement_system_bulk(
    highs: polars.Series, lows: polars.Series, closes: polars.Series, period: int, constant_model_type: str
  ) -> polars.DataFrame:
    r"""
    Calculate Directional Movement System indicators for time series data

    Computes the complete DMS including Positive Directional Indicator (+DI), Negative Directional
    Indicator (-DI), Average Directional Index (ADX), and Average Directional Rating (ADXR).
    """
  @staticmethod
  def volume_price_trend_bulk(prices: polars.Series, volumes: polars.Series, previous_volume_price_trend: float) -> polars.Series:
    r"""
    Calculate Volume Price Trend indicator for time series data

    VPT combines price and volume to show the relationship between price movement and volume flow.
    """
  @staticmethod
  def true_strength_index_bulk(
    prices: polars.Series, first_constant_model: str, first_period: int, second_constant_model: str, second_period: int
  ) -> polars.Series:
    r"""
    Calculate True Strength Index for time series data

    TSI is a momentum oscillator that uses double-smoothed price changes to filter noise
    and provide clearer signals of price momentum direction and strength.
    """
```

### `volatility_ti` - Volatility Indicators (Exposes methods from the VolatilityTI class)

```python
class VolatilityTI:
  @staticmethod
  def ulcer_index_single(prices: polars.Series) -> float:
    r"""
    Ulcer Index (Single) - Calculates how quickly the price is able to get back to its former high
    Can be used instead of standard deviation for volatility measurement
    """
  @staticmethod
  def ulcer_index_bulk(prices: polars.Series, period: int) -> polars.Series:
    r"""
    Ulcer Index (Bulk) - Calculates rolling Ulcer Index over specified period
    Returns a series of Ulcer Index values
    """
  @staticmethod
  def volatility_system(
    high: polars.Series, low: polars.Series, close: polars.Series, period: int, constant_multiplier: float, constant_model_type: str
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
