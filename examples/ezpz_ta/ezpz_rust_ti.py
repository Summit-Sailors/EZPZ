from datetime import date

import polars as pl

pl_series = pl.Series

_df = pl.select(
  timestamp=pl.date_range(start=date(2023, 1, 1), end=date(2023, 12, 31), interval="1d"),
).with_columns(
  [
    pl_series("open", [100 + i * 0.1 for i in range(365)]),
    pl_series("high", [101 + i * 0.1 for i in range(365)]),
    pl_series("low", [99 + i * 0.1 for i in range(365)]),
    pl_series("close", [100.5 + i * 0.1 for i in range(365)]),
    pl_series("volume", [1000 + i * 10 for i in range(365)]),
  ]
)

print(f"DataFrame shape: {_df.shape}")
print(_df.head())

# Get the close price series
close = _df["close"]

# Calculate technical indicators - it's that simple!
sma_20 = pl_series.standard_ti.sma_bulk(close, 20)  # Simple Moving Average
print(f"SMA(20) last 5 values: {sma_20}")
