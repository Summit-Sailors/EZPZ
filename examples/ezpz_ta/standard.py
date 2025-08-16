import time
import logging
import statistics
from typing import Unpack, Callable
from datetime import date, timedelta

import polars as pl

logger = logging.getLogger(__name__)

# Thresholds for numerical accuracy comparison
TOLERANCE_MACHINE_PRECISION = 1e-10
TOLERANCE_HIGH_ACCURACY = 1e-6
TOLERANCE_MINOR_DIFFERENCE = 1e-3


class InsufficientDataError(ValueError): ...


class BenchmarkResult:
  def __init__(self, avg_time: float, min_time: float, max_time: float, std_dev: float) -> None:
    self.avg_time = avg_time
    self.min_time = min_time
    self.max_time = max_time
    self.std_dev = std_dev

  @property
  def avg_time_ms(self) -> float:
    return self.avg_time * 1000.0


def sma_pure_python(prices: list[float], period: int) -> list[float]:
  length = len(prices)
  if period > length:
    raise InsufficientDataError()

  result: list[float] = []

  loop_max = length - period + 1
  for i in range(loop_max):
    # The slice now starts from 'i' and goes for 'period' elements
    window_sum = sum(prices[i : i + period])
    result.append(window_sum / period)

  return result


def sma_pure_python_optimized(prices: list[float], period: int) -> list[float]:
  length = len(prices)
  if period > length:
    raise InsufficientDataError()

  result: list[float] = []

  # the first SMA value
  # The first window is from index 0 to period-1
  current_sum: float = sum(prices[0:period])
  result.append(current_sum / period)

  # Slide the window for subsequent values, starting from the next element after the first window
  # The loop goes from 'period' up to 'length'
  for i in range(period, length):
    current_sum += prices[i] - prices[i - period]  # Add new, subtract old
    result.append(current_sum / period)

  return result


def benchmark_python_function(
  func: Callable[[list[float], int], list[float]], *args: Unpack[tuple[list[float], int]], num_runs: int = 1000
) -> tuple[BenchmarkResult, list[float] | None]:
  times: list[float] = []
  result: list[float] | None = None

  for _ in range(num_runs):
    start_time = time.perf_counter()
    result = func(*args)
    end_time = time.perf_counter()
    times.append(end_time - start_time)

  benchmark_result = BenchmarkResult(
    avg_time=statistics.mean(times), min_time=min(times), max_time=max(times), std_dev=statistics.stdev(times) if len(times) > 1 else 0.0
  )

  return benchmark_result, result


def benchmark_rust_function(
  func: Callable[[pl.LazyFrame, str, int], pl.Series], *args: Unpack[tuple[pl.LazyFrame, str, int]], num_runs: int = 1000
) -> tuple[BenchmarkResult, pl.Series | None]:
  times: list[float] = []
  result = None

  for _ in range(num_runs):
    start_time = time.perf_counter()
    result = func(*args)
    end_time = time.perf_counter()
    times.append(end_time - start_time)

  benchmark_result = BenchmarkResult(
    avg_time=statistics.mean(times), min_time=min(times), max_time=max(times), std_dev=statistics.stdev(times) if len(times) > 1 else 0.0
  )

  return benchmark_result, result


def create_test_data(num_points: int = 365) -> tuple[pl.LazyFrame, list[float]]:
  start_date = date(2023, 1, 1)
  end_date = start_date + timedelta(days=num_points - 1)

  # Create price data
  close_prices = [100.5 + i * 0.1 for i in range(num_points)]

  _df = pl.select(
    timestamp=pl.date_range(start=start_date, end=end_date, interval="1d", eager=True),
  ).with_columns(
    [
      pl.Series("open", [100 + i * 0.1 for i in range(num_points)]),
      pl.Series("high", [101 + i * 0.1 for i in range(num_points)]),
      pl.Series("low", [99 + i * 0.1 for i in range(num_points)]),
      pl.Series("close", close_prices),
      pl.Series("volume", [1000 + i * 10 for i in range(num_points)]),
    ]
  )

  return _df.lazy(), close_prices


def compare_results_accuracy(first_result: list[float] | None, second_result: pl.Series | list[float] | None, title: str = "ACCURACY COMPARISON") -> None:
  """Compare accuracy between Python and Rust implementations."""
  logger.info("=" * 50)
  logger.info(title)
  logger.info("=" * 50)

  second_result_list = list[float]()

  if isinstance(second_result, pl.Series):
    second_result_list = second_result.to_list()
  elif isinstance(second_result, list):
    second_result_list = second_result
  else:
    raise TypeError("PANIC!")

  if first_result is not None and len(first_result) != len(second_result_list):
    logger.error(f"Length mismatch: Python={len(first_result)}, Other={len(second_result_list)}")
    return

  # Compare values
  differences: list[float] = []
  max_diff = 0.0
  first_valid_idx = None

  if first_result is None:
    raise ValueError("first_result_is_None")

  for i, (py_val, other_val) in enumerate(zip(first_result, second_result_list, strict=True)):
    if first_valid_idx is None:
      first_valid_idx = i
    diff = abs(py_val - other_val)
    differences.append(diff)
    max_diff = max(max_diff, diff)

  if not differences:
    logger.warning("No valid values to compare")
    return

  avg_diff = statistics.mean(differences)

  logger.info(f"Values compared: {len(differences)}")
  logger.info(f"Average difference: {avg_diff:.2e}")
  logger.info(f"Maximum difference: {max_diff:.2e}")

  logger.info("\nSample value comparisons (last 5 valid values):")
  sample_size = min(5, len(differences))
  start_idx_for_display = len(first_result) - sample_size
  start_idx_for_display = max(start_idx_for_display, 0)

  for i in range(start_idx_for_display, len(first_result)):
    py_val = first_result[i]
    other_val = second_result_list[i]
    if other_val is not None:
      diff = abs(py_val - other_val)
      logger.info(f"Index {i}: Python={py_val:.8f}, Other={other_val:.8f}, Diff={diff:.2e}")

  # Accuracy assessment
  if max_diff < TOLERANCE_MACHINE_PRECISION:
    logger.info("✓ Results are numerically identical (within machine precision)")
  elif max_diff < TOLERANCE_HIGH_ACCURACY:
    logger.info("✓ Results are highly accurate (sub-microsecond differences)")
  elif max_diff < TOLERANCE_MINOR_DIFFERENCE:
    logger.info("⚠️ Results have minor differences (sub-millisecond)")
  else:
    logger.error("✗ Results have significant differences")

  logger.info("")


def main() -> None:  # noqa: PLR0915
  logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")

  period = 20
  num_runs = 1000

  dataset_sizes = [365, 10_000, 100_000, 1_000_000]

  for size in dataset_sizes:
    logger.info(f"--- Benchmarks for {size:,} data points ---")
    logger.info("=" * 50)

    lf, prices = create_test_data(size)

    logger.info(f"Data points: {size:,}")
    logger.info(f"SMA period: {period}")
    logger.info(f"Benchmark runs: {num_runs}")
    logger.info("")

    logger.info("Benchmarking Original Pure Python SMA...")
    python_orig_benchmark, python_orig_result = benchmark_python_function(sma_pure_python, prices, period, num_runs=num_runs)
    logger.info(f"Original Python avg: {python_orig_benchmark.avg_time_ms:.4f} ms")

    logger.info("Benchmarking Optimized Pure Python SMA...")
    python_opt_benchmark, python_opt_result = benchmark_python_function(sma_pure_python_optimized, prices, period, num_runs=num_runs)
    logger.info(f"Optimized Python avg: {python_opt_benchmark.avg_time_ms:.4f} ms")

    # Original Python vs Optimized Python (Accuracy Check)
    compare_results_accuracy(python_orig_result, python_opt_result, title="ORIGINAL VS OPTIMIZED PYTHON ACCURACY")

    logger.info("Benchmarking Rust SMA...")

    def rust_sma_wrapper(lf: pl.LazyFrame, price_column: str, period: int) -> pl.Series:
      return lf.standard_ti.sma_bulk(price_column, period)

    try:
      rust_benchmark, rust_result = benchmark_rust_function(
        rust_sma_wrapper,
        lf,
        "close",
        period,
        num_runs=num_runs,
      )
      logger.info(f"Rust avg: {rust_benchmark.avg_time_ms:.4f} ms")

      # Python Results against Rust results (Accuracy Check)
      compare_results_accuracy(python_opt_result, rust_result, title="OPTIMIZED PYTHON VS RUST ACCURACY")

      # --- Final Performance Comparison ---
      logger.info("")
      logger.info("=" * 50)
      logger.info("PERFORMANCE RESULTS SUMMARY")
      logger.info("=" * 50)
      logger.info(f"Original Python:   {python_orig_benchmark.avg_time_ms:.4f} ms")
      logger.info(f"Optimized Python:  {python_opt_benchmark.avg_time_ms:.4f} ms")
      logger.info(f"Rust:              {rust_benchmark.avg_time_ms:.4f} ms")

      logger.info("\n--- Speedup (vs. Original Python) ---")
      if python_opt_benchmark.avg_time < python_orig_benchmark.avg_time:
        speedup_opt = python_orig_benchmark.avg_time / python_opt_benchmark.avg_time
        logger.info(f"✓ Optimized Python is {speedup_opt:.1f}x FASTER than Original Python")
      else:
        logger.info("⚠️ Optimized Python is not faster than Original Python (unlikely)")

      if rust_benchmark.avg_time < python_orig_benchmark.avg_time:
        speedup_rust_vs_orig = python_orig_benchmark.avg_time / rust_benchmark.avg_time
        logger.info(f"✓ Rust is {speedup_rust_vs_orig:.1f}x FASTER than Original Python")
      else:
        slowdown_rust_vs_orig = rust_benchmark.avg_time / python_orig_benchmark.avg_time
        logger.info(f"⚠️ Rust is {slowdown_rust_vs_orig:.1f}x SLOWER than Original Python")

      logger.info("\n--- Speedup (vs. Optimized Python) ---")
      if rust_benchmark.avg_time < python_opt_benchmark.avg_time:
        speedup_rust_vs_opt = python_opt_benchmark.avg_time / rust_benchmark.avg_time
        logger.info(f"✓ Rust is {speedup_rust_vs_opt:.1f}x FASTER than Optimized Python")
      else:
        slowdown_rust_vs_opt = rust_benchmark.avg_time / python_opt_benchmark.avg_time
        logger.info(f"⚠️ Rust is {slowdown_rust_vs_opt:.1f}x SLOWER than Optimized Python")
        logger.info("  (This suggests overhead in the Rust binding or small dataset size)")

      logger.info("")

    except AttributeError:
      logger.exception("rust_ti extension not available - cannot benchmark Rust implementation")
      logger.info("Install the rust_ti extension to compare with Rust performance")
      break
    except Exception:
      logger.exception("Error benchmarking Rust implementation")
      break


if __name__ == "__main__":
  main()
