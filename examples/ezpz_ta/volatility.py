# ruff: noqa: NPY002, T201
import numpy as np
import polars as pl


def test_volatility_ti_plugin() -> None:
  """Test the VolatilityTI plugin via polars LazyFrame.volatility_ti"""

  # sample OHLC data
  np.random.seed(42)
  n_periods = 100

  base_price = 100.0
  returns = np.random.normal(0, 0.02, n_periods)  # 2% daily volatility
  prices = [base_price]

  for ret in returns:
    prices.append(prices[-1] * (1 + ret))

  high_prices = [p * (1 + abs(np.random.normal(0, 0.01))) for p in prices[1:]]
  low_prices = [p * (1 - abs(np.random.normal(0, 0.01))) for p in prices[1:]]
  close_prices = prices[1:]

  _df = pl.DataFrame({"high": high_prices, "low": low_prices, "close": close_prices, "volume": np.random.randint(1000, 10000, n_periods)})

  print("Sample data:")
  print(_df.head())
  print(f"\nData shape: {_df.shape}")

  lf = _df.lazy()

  print("\n=== Testing Ulcer Index (Single) via Plugin ===")
  try:
    ulcer_single = lf.volatility_ti.ulcer_index_single("close")
    print(f"Single Ulcer Index: {ulcer_single:.6f}")
  except Exception as e:
    print(f"Error in ulcer_index_single: {e}")

  print("\n=== Testing Ulcer Index (Bulk) via Plugin ===")
  try:
    ulcer_bulk_series = lf.volatility_ti.ulcer_index_bulk("close", period=14)
    print(f"Ulcer Index Bulk Series type: {type(ulcer_bulk_series)}")
    print(f"Series name: {ulcer_bulk_series.name}")
    print(f"Series length: {len(ulcer_bulk_series)}")
    print(f"First 10 values: {ulcer_bulk_series.head(10).to_list()}")
    print(f"Last 10 values: {ulcer_bulk_series.tail(10).to_list()}")
  except Exception as e:
    print(f"Error in ulcer_index_bulk: {e}")

  print("\n=== Testing Volatility System via Plugin ===")
  try:
    volatility_system_series = lf.volatility_ti.volatility_system(
      high_column="high",
      low_column="low",
      close_column="close",
      period=14,
      constant_multiplier=3.0,
      constant_model_type="sma",
    )
    print(f"Volatility System Series type: {type(volatility_system_series)}")
    print(f"Series name: {volatility_system_series.name}")
    print(f"Series length: {len(volatility_system_series)}")
    print(f"First 10 values: {volatility_system_series.head(10).to_list()}")
    print(f"Last 10 values: {volatility_system_series.tail(10).to_list()}")
  except Exception as e:
    print(f"Error in volatility_system: {e}")

  print("\n=== Testing Integration with Polars Operations ===")
  try:
    result_df = lf.with_columns(
      [
        lf.volatility_ti.ulcer_index_bulk("close", period=14).alias("ulcer_index_14"),
        lf.volatility_ti.volatility_system("high", "low", "close", 14, 3.0, "sma").alias("volatility_system"),
      ]
    ).collect()

    print("DataFrame with volatility indicators:")
    print(result_df.head())
    print(f"\nFinal DataFrame shape: {result_df.shape}")
    print(f"Columns: {result_df.columns}")
  except Exception as e:
    print(f"Error in integration test: {e}")

  print("\n=== Testing Error Handling ===")
  try:
    lf.volatility_ti.ulcer_index_single("invalid_column")
  except Exception as e:
    print(f"Expected error for invalid column: {e}")

  print("\n=== Performance Test ===")
  large_n = 10000
  large_prices = [base_price]
  large_returns = np.random.normal(0, 0.02, large_n)

  for ret in large_returns:
    large_prices.append(large_prices[-1] * (1 + ret))

  large_df = pl.DataFrame(
    {
      "high": [p * (1 + abs(np.random.normal(0, 0.01))) for p in large_prices[1:]],
      "low": [p * (1 - abs(np.random.normal(0, 0.01))) for p in large_prices[1:]],
      "close": large_prices[1:],
    }
  )

  large_lf = large_df.lazy()

  import time

  start_time = time.time()

  try:
    large_ulcer = large_lf.volatility_ti.ulcer_index_single("close")
    end_time = time.time()
    print(f"Large dataset ({large_n} rows) Ulcer Index: {large_ulcer:.6f}")
    print(f"Processing time: {end_time - start_time:.4f} seconds")
  except Exception as e:
    print(f"Error with large dataset: {e}")


def test_chaining_operations() -> None:
  """Test chaining volatility operations with other polars operations"""
  print("\n=== Testing Method Chaining ===")

  # sample data
  np.random.seed(123)
  n = 200
  base_price = 100.0
  returns = np.random.normal(0, 0.015, n)
  prices = [base_price]

  for ret in returns:
    prices.append(prices[-1] * (1 + ret))

  _df = pl.DataFrame(
    {
      "timestamp": pl.date_range(start="2024-01-01", end="2024-07-19", interval="1d").head(n),
      "high": [p * (1 + abs(np.random.normal(0, 0.008))) for p in prices[1:]],
      "low": [p * (1 - abs(np.random.normal(0, 0.008))) for p in prices[1:]],
      "close": prices[1:],
      "volume": np.random.randint(5000, 50000, n),
    }
  )

  lf = _df.lazy()

  try:
    result = (
      lf.with_columns(
        [
          lf.volatility_ti.ulcer_index_bulk("close", period=20).alias("ulcer_20"),
          lf.volatility_ti.volatility_system("high", "low", "close", 14, 2.8, "sma").alias("vol_system"),
          pl.col("close").rolling_mean(window_size=20).alias("sma_20"),
          pl.col("close").rolling_std(window_size=20).alias("std_20"),
          (pl.col("close") / pl.col("close").shift(1) - 1).alias("returns"),
        ]
      )
      .filter(pl.col("timestamp") > pl.date(2024, 1, 20))
      .select(["timestamp", "close", "ulcer_20", "vol_system", "sma_20", "std_20", "returns"])
      .collect()
    )

    print("Chained operations result:")
    print(result.head(10))
    print(f"\nResult shape: {result.shape}")

    # some statistics
    print("\nUlcer Index 20 stats:")
    print(f"  Mean: {result['ulcer_20'].mean():.6f}")
    print(f"  Std:  {result['ulcer_20'].std():.6f}")
    print(f"  Min:  {result['ulcer_20'].min():.6f}")
    print(f"  Max:  {result['ulcer_20'].max():.6f}")

  except Exception as e:
    print(f"Error in chaining test: {e}")


def benchmark_memory_usage() -> None:
  """Benchmark memory usage of the plugin"""
  import os

  import psutil

  process = psutil.Process(os.getpid())

  print("\n=== Memory Usage Benchmark ===")
  initial_memory = process.memory_info().rss / 1024 / 1024  # MB
  print(f"Initial memory usage: {initial_memory:.2f} MB")

  # with increasingly large datasets
  sizes = [1000, 5000, 10000, 50000]

  for size in sizes:
    base_price = 100.0
    returns = np.random.normal(0, 0.02, size)
    prices = [base_price]

    for ret in returns:
      prices.append(prices[-1] * (1 + ret))

    _df = pl.DataFrame(
      {
        "high": [p * 1.01 for p in prices[1:]],
        "low": [p * 0.99 for p in prices[1:]],
        "close": prices[1:],
      }
    )

    lf = _df.lazy()

    # Measure memory before operation
    before_memory = process.memory_info().rss / 1024 / 1024

    # Perform operation via plugin
    import time

    start_time = time.time()
    ulcer = lf.volatility_ti.ulcer_index_single("close")
    end_time = time.time()

    after_memory = process.memory_info().rss / 1024 / 1024

    print(f"Size: {size:6d} | Time: {end_time - start_time:.4f}s | Memory: {before_memory:.1f}MB -> {after_memory:.1f}MB | Ulcer: {ulcer:.6f}")

    del _df, lf


if __name__ == "__main__":
  test_volatility_ti_plugin()
  test_chaining_operations()
  benchmark_memory_usage()
