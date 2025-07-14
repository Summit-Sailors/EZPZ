use {
	crate::utils::{create_triple_df, extract_f64_values},
	ezpz_stubz::{frame::PyDfStubbed, lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct StandardTI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl StandardTI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	/// Simple Moving Average (Single) - calculates the mean of all values in the column
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// f64 - Single SMA value calculated from all provided prices
	fn sma_single(&self, price_column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}

		let result = rust_ti::standard_indicators::single::simple_moving_average(&values);
		Ok(result)
	}

	/// Simple Moving Average (Bulk) - calculates the mean over a rolling window
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `period`: usize - Number of periods for the moving average window
	///
	/// # Returns
	/// PySeriesStubbed - Series containing SMA values for each period
	fn sma_bulk(&self, price_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.len() < period {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series length must be at least the specified period"));
		}

		let sma_result = rust_ti::standard_indicators::bulk::simple_moving_average(&values, period);
		let result_series = Series::new("sma".into(), sma_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Smoothed Moving Average (Single) - single value calculation
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// f64 - Single SMMA value calculated from all provided prices
	fn smma_single(&self, price_column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}

		let result = rust_ti::standard_indicators::single::smoothed_moving_average(&values);
		Ok(result)
	}

	/// Smoothed Moving Average (Bulk) - puts more weight on recent prices
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `period`: usize - Number of periods for the smoothed moving average window
	///
	/// # Returns
	/// PySeriesStubbed - Series containing SMMA values for each period
	fn smma_bulk(&self, price_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.len() < period {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series length must be at least the specified period"));
		}

		let smma_result = rust_ti::standard_indicators::bulk::smoothed_moving_average(&values, period);
		let result_series = Series::new("smma".into(), smma_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Exponential Moving Average (Single) - single value calculation
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// f64 - Single EMA value calculated from all provided prices
	fn ema_single(&self, price_column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}

		let result = rust_ti::standard_indicators::single::exponential_moving_average(&values);
		Ok(result)
	}

	/// Exponential Moving Average (Bulk) - puts exponentially more weight on recent prices
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `period`: usize - Number of periods for the exponential moving average window
	///
	/// # Returns
	/// PySeriesStubbed - Series containing EMA values for each period
	fn ema_bulk(&self, price_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.len() < period {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series length must be at least the specified period"));
		}

		let ema_result = rust_ti::standard_indicators::bulk::exponential_moving_average(&values, period);
		let result_series = Series::new("ema".into(), ema_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Bollinger Bands (Single) - single value calculation (requires exactly 20 periods)
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// Tuple of (lower_band: f64, middle_band: f64, upper_band: f64)
	/// - `lower_band`: Lower Bollinger Band value
	/// - `middle_band`: Middle band (SMA) value
	/// - `upper_band`: Upper Bollinger Band value
	fn bollinger_bands_single(&self, price_column: &str) -> PyResult<(f64, f64, f64)> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.len() != 20 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series length must be exactly 20 for single Bollinger Bands calculation"));
		}

		let result = rust_ti::standard_indicators::single::bollinger_bands(&values);
		Ok(result)
	}

	/// Bollinger Bands (Bulk) - returns three series: lower band, middle (SMA), upper band
	/// Standard period is 20 with 2 standard deviations
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// PyDfStubbed - DataFrame with three columns:
	/// - `bb_lower`: Lower Bollinger Band values
	/// - `bb_middle`: Middle band (20-period SMA)
	/// - `bb_upper`: Upper Bollinger Band values
	fn bollinger_bands_bulk(&self, price_column: &str) -> PyResult<PyDfStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.len() < 20 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series length must be at least 20 for Bollinger Bands"));
		}

		let bb_result = rust_ti::standard_indicators::bulk::bollinger_bands(&values);

		let lower: Vec<f64> = bb_result.iter().map(|(l, _, _)| *l).collect();
		let middle: Vec<f64> = bb_result.iter().map(|(_, m, _)| *m).collect();
		let upper: Vec<f64> = bb_result.iter().map(|(_, _, u)| *u).collect();

		create_triple_df(lower, middle, upper, "bb_lower", "bb_middle", "bb_upper")
	}

	/// MACD (Single) - single value calculation (requires exactly 34 periods)
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// Tuple of (macd_line: f64, signal_line: f64, histogram: f64)
	/// - `macd_line`: MACD line value (12-period EMA - 26-period EMA)
	/// - `signal_line`: Signal line value (9-period EMA of MACD line)
	/// - `histogram`: Histogram value (MACD line - Signal line)
	fn macd_single(&self, price_column: &str) -> PyResult<(f64, f64, f64)> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.len() != 34 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series length must be exactly 34 for single MACD calculation"));
		}

		let result = rust_ti::standard_indicators::single::macd(&values);
		Ok(result)
	}

	/// MACD (Bulk) - Moving Average Convergence Divergence
	/// Returns three series: MACD line, Signal line, Histogram
	/// Standard periods: 12, 26, 9
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// PyDfStubbed - DataFrame with three columns:
	/// - `macd`: MACD line (12-period EMA - 26-period EMA)
	/// - `macd_signal`: Signal line (9-period EMA of MACD line)
	/// - `macd_histogram`: Histogram (MACD line - Signal line)
	fn macd_bulk(&self, price_column: &str) -> PyResult<PyDfStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.len() < 34 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series length must be at least 34 for MACD"));
		}

		let macd_result = rust_ti::standard_indicators::bulk::macd(&values);

		let macd_line: Vec<f64> = macd_result.iter().map(|(m, _, _)| *m).collect();
		let signal_line: Vec<f64> = macd_result.iter().map(|(_, s, _)| *s).collect();
		let histogram: Vec<f64> = macd_result.iter().map(|(_, _, h)| *h).collect();

		create_triple_df(macd_line, signal_line, histogram, "macd", "macd_signal", "macd_histogram")
	}

	/// RSI (Single) - single value calculation (requires exactly 14 periods)
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// f64 - Single RSI value (0-100 scale)
	fn rsi_single(&self, price_column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.len() != 14 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series length must be exactly 14 for single RSI calculation"));
		}

		let result = rust_ti::standard_indicators::single::rsi(&values);
		Ok(result)
	}

	/// RSI (Bulk) - Relative Strength Index
	/// Standard period is 14 using smoothed moving average
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// PySeriesStubbed - Series containing RSI values (0-100 scale)
	fn rsi_bulk(&self, price_column: &str) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;

		if values.len() < 14 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series length must be at least 14 for RSI"));
		}

		let rsi_result = rust_ti::standard_indicators::bulk::rsi(&values);
		let result_series = Series::new("rsi".into(), rsi_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;

	fn create_test_dataframe() -> LazyFrame {
		let data = vec![
			1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0, 21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0,
			28.0, 29.0, 30.0, 31.0, 32.0, 33.0, 34.0, 35.0,
		];
		df! {
			"price" => data,
			"volume" => vec![100.0; 35]
		}
		.unwrap()
		.lazy()
	}

	fn create_standard_ti() -> StandardTI {
		let lf = create_test_dataframe();
		StandardTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	fn create_small_dataframe() -> LazyFrame {
		let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
		df! {
			"price" => data
		}
		.unwrap()
		.lazy()
	}

	fn create_small_ti() -> StandardTI {
		let lf = create_small_dataframe();
		StandardTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_sma_single() {
		let ti = create_standard_ti();
		let result = ti.sma_single("price").unwrap();
		assert_abs_diff_eq!(result, 18.0, epsilon = 1e-10);
	}

	#[test]
	fn test_sma_bulk() {
		let ti = create_standard_ti();
		let result = ti.sma_bulk("price", 3).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 33);
		assert_abs_diff_eq!(values[0], 2.0, epsilon = 1e-10);
		assert_abs_diff_eq!(values[1], 3.0, epsilon = 1e-10);
		assert_abs_diff_eq!(values[2], 4.0, epsilon = 1e-10);
	}

	#[test]
	fn test_sma_bulk_insufficient_data() {
		let ti = create_small_ti();
		let result = ti.sma_bulk("price", 10);
		assert!(result.is_err());
	}

	#[test]
	fn test_smma_single() {
		let ti = create_standard_ti();
		let result = ti.smma_single("price").unwrap();
		assert!(result > 0.0);
	}

	#[test]
	fn test_smma_bulk() {
		let ti = create_standard_ti();
		let result = ti.smma_bulk("price", 5).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 31);
		assert!(values.iter().all(|&x| x > 0.0));
	}

	#[test]
	fn test_smma_bulk_insufficient_data() {
		let ti = create_small_ti();
		let result = ti.smma_bulk("price", 10);
		assert!(result.is_err());
	}

	#[test]
	fn test_ema_single() {
		let ti = create_standard_ti();
		let result = ti.ema_single("price").unwrap();
		assert!(result > 0.0);
	}

	#[test]
	fn test_ema_bulk() {
		let ti = create_standard_ti();
		let result = ti.ema_bulk("price", 5).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 31);
		assert!(values.iter().all(|&x| x > 0.0));
	}

	#[test]
	fn test_ema_bulk_insufficient_data() {
		let ti = create_small_ti();
		let result = ti.ema_bulk("price", 10);
		assert!(result.is_err());
	}

	#[test]
	fn test_bollinger_bands_single() {
		let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0];
		let df = df! {
			"price" => data
		}
		.unwrap()
		.lazy();

		let ti = StandardTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(df)));
		let result = ti.bollinger_bands_single("price").unwrap();

		assert!(result.0 < result.1);
		assert!(result.1 < result.2);
		assert_abs_diff_eq!(result.1, 10.5, epsilon = 1e-10);
	}

	#[test]
	fn test_bollinger_bands_single_wrong_length() {
		let ti = create_small_ti();
		let result = ti.bollinger_bands_single("price");
		assert!(result.is_err());
	}

	#[test]
	fn test_bollinger_bands_bulk() {
		let ti = create_standard_ti();
		let result = ti.bollinger_bands_bulk("price").unwrap();
		let df = result.0.0;

		assert_eq!(df.height(), 16);
		assert_eq!(df.width(), 3);
		assert!(df.column("bb_lower").is_ok());
		assert!(df.column("bb_middle").is_ok());
		assert!(df.column("bb_upper").is_ok());
	}

	#[test]
	fn test_bollinger_bands_bulk_insufficient_data() {
		let ti = create_small_ti();
		let result = ti.bollinger_bands_bulk("price");
		assert!(result.is_err());
	}

	#[test]
	fn test_macd_single() {
		let data: Vec<f64> = (1..=34).map(|x| x as f64).collect();
		let df = df! {
			"price" => data
		}
		.unwrap()
		.lazy();

		let ti = StandardTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(df)));
		let result = ti.macd_single("price").unwrap();

		assert!(result.0.is_finite());
		assert!(result.1.is_finite());
		assert!(result.2.is_finite());
	}

	#[test]
	fn test_macd_single_wrong_length() {
		let ti = create_small_ti();
		let result = ti.macd_single("price");
		assert!(result.is_err());
	}

	#[test]
	fn test_macd_bulk() {
		let ti = create_standard_ti();
		let result = ti.macd_bulk("price").unwrap();
		let df = result.0.0;

		assert_eq!(df.height(), 2);
		assert_eq!(df.width(), 3);
		assert!(df.column("macd").is_ok());
		assert!(df.column("macd_signal").is_ok());
		assert!(df.column("macd_histogram").is_ok());
	}

	#[test]
	fn test_macd_bulk_insufficient_data() {
		let ti = create_small_ti();
		let result = ti.macd_bulk("price");
		assert!(result.is_err());
	}

	#[test]
	fn test_rsi_single() {
		let data: Vec<f64> = (1..=14).map(|x| x as f64).collect();
		let df = df! {
			"price" => data
		}
		.unwrap()
		.lazy();

		let ti = StandardTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(df)));
		let result = ti.rsi_single("price").unwrap();

		assert!((0.0..=100.0).contains(&result));
	}

	#[test]
	fn test_rsi_single_wrong_length() {
		let ti = create_small_ti();
		let result = ti.rsi_single("price");
		assert!(result.is_err());
	}

	#[test]
	fn test_rsi_bulk() {
		let ti = create_standard_ti();
		let result = ti.rsi_bulk("price").unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 22);
		assert!(values.iter().all(|&x| (0.0..=100.0).contains(&x)));
	}

	#[test]
	fn test_rsi_bulk_insufficient_data() {
		let ti = create_small_ti();
		let result = ti.rsi_bulk("price");
		assert!(result.is_err());
	}

	#[test]
	fn test_invalid_column_error() {
		let ti = create_standard_ti();
		let result = ti.sma_single("nonexistent_column");
		assert!(result.is_err());
	}

	#[test]
	fn test_empty_series_error() {
		let empty_df = df! {
			"price" => Vec::<f64>::new()
		}
		.unwrap()
		.lazy();

		let ti = StandardTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(empty_df)));
		let result = ti.sma_single("price");
		assert!(result.is_err());
	}

	#[test]
	fn test_single_value_dataset() {
		let single_data = df! {
			"price" => vec![5.0]
		}
		.unwrap()
		.lazy();

		let ti = StandardTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(single_data)));

		assert_abs_diff_eq!(ti.sma_single("price").unwrap(), 5.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.smma_single("price").unwrap(), 5.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.ema_single("price").unwrap(), 5.0, epsilon = 1e-10);
	}

	#[test]
	fn test_duplicate_values_dataset() {
		let duplicate_data = df! {
			"price" => vec![3.0; 35]
		}
		.unwrap()
		.lazy();

		let ti = StandardTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(duplicate_data)));

		assert_abs_diff_eq!(ti.sma_single("price").unwrap(), 3.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.smma_single("price").unwrap(), 3.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.ema_single("price").unwrap(), 3.0, epsilon = 1e-10);
	}

	#[test]
	fn test_sma_bulk_period_one() {
		let ti = create_standard_ti();
		let result = ti.sma_bulk("price", 1).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 35);
		for (i, &value) in values.iter().enumerate() {
			assert_abs_diff_eq!(value, (i + 1) as f64, epsilon = 1e-10);
		}
	}

	#[test]
	fn test_bollinger_bands_bulk_band_ordering() {
		let ti = create_standard_ti();
		let result = ti.bollinger_bands_bulk("price").unwrap();
		let df = result.0.0;

		let lower: Vec<f64> = df.column("bb_lower").unwrap().f64().unwrap().into_no_null_iter().collect();
		let middle: Vec<f64> = df.column("bb_middle").unwrap().f64().unwrap().into_no_null_iter().collect();
		let upper: Vec<f64> = df.column("bb_upper").unwrap().f64().unwrap().into_no_null_iter().collect();

		for i in 0..lower.len() {
			assert!(lower[i] < middle[i]);
			assert!(middle[i] < upper[i]);
		}
	}

	#[test]
	fn test_macd_bulk_histogram_calculation() {
		let ti = create_standard_ti();
		let result = ti.macd_bulk("price").unwrap();
		let df = result.0.0;

		let macd: Vec<f64> = df.column("macd").unwrap().f64().unwrap().into_no_null_iter().collect();
		let signal: Vec<f64> = df.column("macd_signal").unwrap().f64().unwrap().into_no_null_iter().collect();
		let histogram: Vec<f64> = df.column("macd_histogram").unwrap().f64().unwrap().into_no_null_iter().collect();

		for i in 0..macd.len() {
			assert_abs_diff_eq!(histogram[i], macd[i] - signal[i], epsilon = 1e-10);
		}
	}
}
