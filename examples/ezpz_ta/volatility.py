# ruff: noqa: NPY002, T201
import numpy as np
import polars as pl

DAYS_IN_JANUARY = 31
DAYS_IN_JAN_FEB = 59


def test_volatility_ti_plugin() -> None:  # noqa: PLR0915
  np.random.seed(42)
  n_periods = 100

  base_price = 100.0
  returns = np.random.normal(0, 0.02, n_periods)
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
  print("Skipping volatility_system test - no supported constant model types found")

  print("\n=== Testing Integration with Polars Operations ===")
  try:
    ulcer_series = lf.volatility_ti.ulcer_index_bulk("close", period=14)
    original_df = lf.collect()
    padding_length = len(original_df) - len(ulcer_series)
    padded_ulcer = [None] * padding_length + ulcer_series.to_list()
    result_df = original_df.with_columns(pl.Series("ulcer_index_14", padded_ulcer))

    print("DataFrame with ulcer index:")
    print(result_df.head())
    print(f"\nFinal DataFrame shape: {result_df.shape}")
    print(f"Columns: {result_df.columns}")

    non_null_ulcer = result_df.filter(pl.col("ulcer_index_14").is_not_null())
    print(f"\nNon-null ulcer index values: {len(non_null_ulcer)}")
    print(non_null_ulcer.head())

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

  try:
    ulcer = large_lf.volatility_ti.ulcer_index_single("close")
    print(f"Large dataset ({large_n} rows) Ulcer Index: {ulcer:.6f}")
  except Exception as e:
    print(f"Error with large dataset: {e}")


def test_chaining_operations() -> None:
  print("\n=== Testing Method Chaining ===")

  np.random.seed(123)
  n = 200
  base_price = 100.0
  returns = np.random.normal(0, 0.015, n)
  prices = [base_price]

  for ret in returns:
    prices.append(prices[-1] * (1 + ret))

  timestamps = [
    f"2024-01-{i + 1:02d}" if i < DAYS_IN_JANUARY else f"2024-02-{i - 30:02d}" if i < DAYS_IN_JAN_FEB else f"2024-03-{i - 58:02d}" for i in range(n)
  ]

  _df = pl.DataFrame(
    {
      "timestamp": timestamps,
      "high": [p * (1 + abs(np.random.normal(0, 0.008))) for p in prices[1:]],
      "low": [p * (1 - abs(np.random.normal(0, 0.008))) for p in prices[1:]],
      "close": prices[1:],
      "volume": np.random.randint(5000, 50000, n),
    }
  )

  lf = _df.lazy()

  try:
    ulcer_series = lf.volatility_ti.ulcer_index_bulk("close", period=20)
    base_df = lf.collect()
    padding_length = len(base_df) - len(ulcer_series)
    padded_ulcer = [None] * padding_length + ulcer_series.to_list()

    result = (
      base_df.with_columns(
        [
          pl.Series("ulcer_20", padded_ulcer),
          pl.col("close").rolling_mean(window_size=20).alias("sma_20"),
          pl.col("close").rolling_std(window_size=20).alias("std_20"),
          (pl.col("close") / pl.col("close").shift(1) - 1).alias("returns"),
        ]
      )
      .filter(pl.col("ulcer_20").is_not_null())
      .select(["timestamp", "close", "ulcer_20", "sma_20", "std_20", "returns"])
    )

    print("Chained operations result:")
    print(result.head(10))
    print(f"\nResult shape: {result.shape}")

    print("\nUlcer Index 20 stats:")
    print(f"  Mean: {result['ulcer_20'].mean():.6f}")
    print(f"  Std:  {result['ulcer_20'].std():.6f}")
    print(f"  Min:  {result['ulcer_20'].min():.6f}")
    print(f"  Max:  {result['ulcer_20'].max():.6f}")

  except Exception as e:
    print(f"Error in chaining test: {e}")


if __name__ == "__main__":
  test_volatility_ti_plugin()
  test_chaining_operations()
