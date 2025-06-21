// Standard Indicators
use {
	crate::utils::{create_triple_df, extract_f64_values},
	ezpz_stubz::{frame::PyDfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct StandardTI;

#[gen_stub_pymethods]
#[pymethods]
impl StandardTI {
	/// Simple Moving Average - calculates the mean over a rolling window
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data
	/// - `period`: usize - Number of periods for the moving average window
	///
	/// # Returns
	/// PySeriesStubbed - Series containing SMA values for each period
	#[staticmethod]
	fn sma_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.len() < period {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least period ({})", values.len(), period)));
		}

		let sma_result = rust_ti::standard_indicators::bulk::simple_moving_average(&values, &period);
		let result_series = Series::new("sma".into(), sma_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Smoothed Moving Average - puts more weight on recent prices
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data
	/// - `period`: usize - Number of periods for the smoothed moving average window
	///
	/// # Returns
	/// PySeriesStubbed - Series containing SMMA values for each period
	#[staticmethod]
	fn smma_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.len() < period {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least period ({})", values.len(), period)));
		}

		let smma_result = rust_ti::standard_indicators::bulk::smoothed_moving_average(&values, &period);
		let result_series = Series::new("smma".into(), smma_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Exponential Moving Average - puts exponentially more weight on recent prices
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data
	/// - `period`: usize - Number of periods for the exponential moving average window
	///
	/// # Returns
	/// PySeriesStubbed - Series containing EMA values for each period
	#[staticmethod]
	fn ema_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.len() < period {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least period ({})", values.len(), period)));
		}

		let ema_result = rust_ti::standard_indicators::bulk::exponential_moving_average(&values, &period);
		let result_series = Series::new("ema".into(), ema_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Bollinger Bands - returns three series: lower band, middle (SMA), upper band
	/// Standard period is 20 with 2 standard deviations
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data (minimum 20 periods required)
	///
	/// # Returns
	/// PyDfStubbed - DataFrame with three columns:
	/// - `bb_lower`: Lower Bollinger Band values
	/// - `bb_middle`: Middle band (20-period SMA)
	/// - `bb_upper`: Upper Bollinger Band values
	#[staticmethod]
	fn bollinger_bands_bulk(prices: PySeriesStubbed) -> PyResult<PyDfStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.len() < 20 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least 20 for Bollinger Bands", values.len())));
		}

		let bb_result = rust_ti::standard_indicators::bulk::bollinger_bands(&values);

		let lower: Vec<f64> = bb_result.iter().map(|(l, _, _)| *l).collect();
		let middle: Vec<f64> = bb_result.iter().map(|(_, m, _)| *m).collect();
		let upper: Vec<f64> = bb_result.iter().map(|(_, _, u)| *u).collect();

		create_triple_df(lower, middle, upper, "bb_lower", "bb_middle", "bb_upper")
	}

	/// MACD - Moving Average Convergence Divergence
	/// Returns three series: MACD line, Signal line, Histogram
	/// Standard periods: 12, 26, 9
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data (minimum 34 periods required)
	///
	/// # Returns
	/// PyDfStubbed - DataFrame with three columns:
	/// - `macd`: MACD line (12-period EMA - 26-period EMA)
	/// - `macd_signal`: Signal line (9-period EMA of MACD line)
	/// - `macd_histogram`: Histogram (MACD line - Signal line)
	#[staticmethod]
	fn macd_bulk(prices: PySeriesStubbed) -> PyResult<PyDfStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.len() < 34 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least 34 for MACD", values.len())));
		}

		let macd_result = rust_ti::standard_indicators::bulk::macd(&values);

		let macd_line: Vec<f64> = macd_result.iter().map(|(m, _, _)| *m).collect();
		let signal_line: Vec<f64> = macd_result.iter().map(|(_, s, _)| *s).collect();
		let histogram: Vec<f64> = macd_result.iter().map(|(_, _, h)| *h).collect();

		create_triple_df(macd_line, signal_line, histogram, "macd", "macd_signal", "macd_histogram")
	}

	/// RSI - Relative Strength Index
	/// Standard period is 14 using smoothed moving average
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data (minimum 14 periods required)
	///
	/// # Returns
	/// PySeriesStubbed - Series containing RSI values (0-100 scale)
	#[staticmethod]
	fn rsi_bulk(prices: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.len() < 14 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least 14 for RSI", values.len())));
		}

		let rsi_result = rust_ti::standard_indicators::bulk::rsi(&values);
		let result_series = Series::new("rsi".into(), rsi_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	// Single value methods (for when you want just one calculation)

	/// Simple Moving Average - single value calculation
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data (cannot be empty)
	///
	/// # Returns
	/// f64 - Single SMA value calculated from all provided prices
	#[staticmethod]
	fn sma_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}

		let result = rust_ti::standard_indicators::single::simple_moving_average(&values);
		Ok(result)
	}

	/// Smoothed Moving Average - single value calculation
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data (cannot be empty)
	///
	/// # Returns
	/// f64 - Single SMMA value calculated from all provided prices
	#[staticmethod]
	fn smma_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}

		let result = rust_ti::standard_indicators::single::smoothed_moving_average(&values);
		Ok(result)
	}

	/// Exponential Moving Average - single value calculation
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data (cannot be empty)
	///
	/// # Returns
	/// f64 - Single EMA value calculated from all provided prices
	#[staticmethod]
	fn ema_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}

		let result = rust_ti::standard_indicators::single::exponential_moving_average(&values);
		Ok(result)
	}

	/// Bollinger Bands - single value calculation (requires exactly 20 periods)
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data (must be exactly 20 periods)
	///
	/// # Returns
	/// Tuple of (lower_band: f64, middle_band: f64, upper_band: f64)
	/// - `lower_band`: Lower Bollinger Band value
	/// - `middle_band`: Middle band (SMA) value
	/// - `upper_band`: Upper Bollinger Band value
	#[staticmethod]
	fn bollinger_bands_single(prices: PySeriesStubbed) -> PyResult<(f64, f64, f64)> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.len() != 20 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
				"Series length must be exactly 20 for single Bollinger Bands calculation, got {}",
				values.len()
			)));
		}

		let result = rust_ti::standard_indicators::single::bollinger_bands(&values);
		Ok(result)
	}

	/// MACD - single value calculation (requires exactly 34 periods)
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data (must be exactly 34 periods)
	///
	/// # Returns
	/// Tuple of (macd_line: f64, signal_line: f64, histogram: f64)
	/// - `macd_line`: MACD line value (12-period EMA - 26-period EMA)
	/// - `signal_line`: Signal line value (9-period EMA of MACD line)
	/// - `histogram`: Histogram value (MACD line - Signal line)
	#[staticmethod]
	fn macd_single(prices: PySeriesStubbed) -> PyResult<(f64, f64, f64)> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.len() != 34 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
				"Series length must be exactly 34 for single MACD calculation, got {}",
				values.len()
			)));
		}

		let result = rust_ti::standard_indicators::single::macd(&values);
		Ok(result)
	}

	/// RSI - single value calculation (requires exactly 14 periods)
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Price series data (must be exactly 14 periods)
	///
	/// # Returns
	/// f64 - Single RSI value (0-100 scale)
	#[staticmethod]
	fn rsi_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.len() != 14 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
				"Series length must be exactly 14 for single RSI calculation, got {}",
				values.len()
			)));
		}

		let result = rust_ti::standard_indicators::single::rsi(&values);
		Ok(result)
	}
}
