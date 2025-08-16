use {
	crate::utils::{extract_f64_values, parse_constant_model_type},
	ezpz_stubz::{lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

/// Other Technical Indicators - A collection of other analysis functions for financial data

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct OtherTI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl OtherTI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	/// Return on Investment - Calculates investment value and percentage change for a single period
	/// Uses the first and last values from the price column as start and end prices
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `investment`: f64 - Initial investment amount
	///
	/// # Returns
	/// Tuple of (final_investment_value: f64, percent_return: f64)
	/// - `final_investment_value`: The absolute value of the investment at the end
	/// - `percent_return`: The percentage return on the investment
	fn return_on_investment_single(&self, price_column: &str, investment: f64) -> PyResult<(f64, f64)> {
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
		if values.len() < 2 {
			return Err(pyo3::exceptions::PyValueError::new_err("Series must have at least 2 values"));
		}
		let start_price = values[0];
		let end_price = values[values.len() - 1];
		let result = rust_ti::other_indicators::single::return_on_investment(start_price, end_price, investment);
		Ok(result)
	}

	/// Return on Investment Bulk - Calculates ROI for a series of consecutive price periods
	/// Uses the price column as price values for consecutive period calculations
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `investment`: f64 - Initial investment amount
	///
	/// # Returns
	/// Tuple of (final_investment_values: PySeriesStubbed, percent_returns: PySeriesStubbed)
	/// - `final_investment_values`: Series of absolute investment values for each period
	/// - `percent_returns`: Series of percentage returns for each period
	fn return_on_investment_bulk(&self, price_column: &str, investment: f64) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
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
		let results = rust_ti::other_indicators::bulk::return_on_investment(&values, investment);

		let final_values: Vec<f64> = results.iter().map(|(final_val, _)| *final_val).collect();
		let percent_returns: Vec<f64> = results.iter().map(|(_, percent)| *percent).collect();

		let final_series = Series::new("final_investment_value".into(), final_values);
		let percent_series = Series::new("percent_return".into(), percent_returns);

		Ok((PySeriesStubbed(pyo3_polars::PySeries(final_series)), PySeriesStubbed(pyo3_polars::PySeries(percent_series))))
	}

	/// True Range - Calculates the greatest price movement for a single period
	/// Uses the provided high/low/close columns to calculate true range
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	///
	/// # Returns
	/// PySeriesStubbed - Series of true range values for each period
	fn true_range(&self, high_column: &str, low_column: &str, close_column: &str) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column), col(close_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let high_series = df
			.column(high_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' could not be converted to Series")))?
			.clone();

		let low_series = df
			.column(low_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' could not be converted to Series")))?
			.clone();

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;
		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;

		let results = rust_ti::other_indicators::bulk::true_range(&close_values, &high_values, &low_values);
		let result_series = Series::new("true_range".into(), results);

		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Average True Range - Calculates the moving average of true range values for a single result
	/// Uses the provided high/low/close columns to calculate ATR from the entire price series
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `constant_model_type`: &str - Type of moving average ("sma", "ema", "wma", etc.)
	///
	/// # Returns
	/// f64 - Single ATR value calculated from the entire price series
	fn average_true_range_single(&self, high_column: &str, low_column: &str, close_column: &str, constant_model_type: &str) -> PyResult<f64> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column), col(close_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let high_series = df
			.column(high_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' could not be converted to Series")))?
			.clone();

		let low_series = df
			.column(low_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' could not be converted to Series")))?
			.clone();

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;
		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::other_indicators::single::average_true_range(&close_values, &high_values, &low_values, constant_type);

		Ok(result)
	}

	/// Average True Range Bulk - Calculates rolling ATR values over specified periods
	/// Uses the provided high/low/close columns for rolling ATR calculations
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `constant_model_type`: &str - Type of moving average ("sma", "ema", "wma", etc.)
	/// - `period`: usize - Number of periods for the moving average calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series of ATR values for each period
	fn average_true_range_bulk(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		constant_model_type: &str,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column), col(close_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let high_series = df
			.column(high_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' could not be converted to Series")))?
			.clone();

		let low_series = df
			.column(low_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' could not be converted to Series")))?
			.clone();

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;
		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let results = rust_ti::other_indicators::bulk::average_true_range(&close_values, &high_values, &low_values, constant_type, period);

		let result_series = Series::new("average_true_range".into(), results);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Internal Bar Strength - Calculates buy/sell oscillator based on close position within high-low range
	/// Uses the provided high/low/close columns to calculate IBS values
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	///
	/// # Returns
	/// PySeriesStubbed - Series of IBS values (0-1 range) for each period, where values closer to 1
	///                   indicate closes near the high, and values closer to 0 indicate closes near the low
	fn internal_bar_strength(&self, high_column: &str, low_column: &str, close_column: &str) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column), col(close_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let high_series = df
			.column(high_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' could not be converted to Series")))?
			.clone();

		let low_series = df
			.column(low_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' could not be converted to Series")))?
			.clone();

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;
		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;

		let results = rust_ti::other_indicators::bulk::internal_bar_strength(&high_values, &low_values, &close_values);
		let result_series = Series::new("internal_bar_strength".into(), results);

		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Positivity Indicator - Generates trading signals based on open vs previous close comparison
	/// Uses the provided open/close columns for signal generation
	///
	/// # Parameters
	/// - `open_column`: &str - Name of the opening price column
	/// - `close_column`: &str - Name of the close price column
	/// - `signal_period`: usize - Number of periods for signal line smoothing
	/// - `constant_model_type`: &str - Type of moving average for signal line ("sma", "ema", "wma", etc.)
	///
	/// # Returns
	/// Tuple of (positivity_indicator: PySeriesStubbed, signal_line: PySeriesStubbed)
	/// - `positivity_indicator`: Series of raw positivity values based on open/close comparison
	/// - `signal_line`: Series of smoothed signal values using specified moving average
	fn positivity_indicator(
		&self,
		open_column: &str,
		close_column: &str,
		signal_period: usize,
		constant_model_type: &str,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
		let df = self
			.lf
			.clone()
			.select([col(open_column), col(close_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let open_series = df
			.column(open_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{open_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{open_column}' could not be converted to Series")))?
			.clone();

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let open_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(open_series)))?;
		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let results = rust_ti::other_indicators::bulk::positivity_indicator(&open_values, &close_values, signal_period, constant_type);

		let positivity_values: Vec<f64> = results.iter().map(|(pos, _)| *pos).collect();
		let signal_values: Vec<f64> = results.iter().map(|(_, signal)| *signal).collect();

		let positivity_series = Series::new("positivity_indicator".into(), positivity_values);
		let signal_series = Series::new("signal_line".into(), signal_values);

		Ok((PySeriesStubbed(pyo3_polars::PySeries(positivity_series)), PySeriesStubbed(pyo3_polars::PySeries(signal_series))))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;

	fn create_test_dataframe() -> LazyFrame {
		df! {
			"open" => vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0],
			"high" => vec![105.0, 106.0, 104.0, 107.0, 109.0, 108.0, 110.0, 112.0, 111.0, 113.0],
			"low" => vec![99.0, 101.0, 100.0, 102.0, 104.0, 103.0, 105.0, 107.0, 106.0, 108.0],
			"close" => vec![104.0, 103.0, 102.0, 106.0, 107.0, 106.0, 109.0, 110.0, 108.0, 112.0],
			"price" => vec![100.0, 102.0, 101.0, 103.0, 105.0, 104.0, 106.0, 108.0, 107.0, 109.0]
		}
		.unwrap()
		.lazy()
	}

	fn create_basic_other_ti() -> OtherTI {
		let lf = create_test_dataframe();
		OtherTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_return_on_investment_single() {
		let ti = create_basic_other_ti();
		let result = ti.return_on_investment_single("price", 1000.0).unwrap();

		// First price: 100.0, Last price: 109.0
		// Expected final value: 1000.0 * (109.0 / 100.0) = 1090.0
		// Expected return: (109.0 - 100.0) / 100.0 * 100.0 = 9.0%
		assert_abs_diff_eq!(result.0, 1090.0, epsilon = 1e-10);
		assert_abs_diff_eq!(result.1, 9.0, epsilon = 1e-10);
	}

	#[test]
	fn test_return_on_investment_single_insufficient_data() {
		let single_data = df! {
			"price" => vec![100.0]
		}
		.unwrap()
		.lazy();

		let ti = OtherTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(single_data)));
		let result = ti.return_on_investment_single("price", 1000.0);
		assert!(result.is_err());
	}

	#[test]
	fn test_return_on_investment_bulk() {
		let ti = create_basic_other_ti();
		let result = ti.return_on_investment_bulk("price", 1000.0).unwrap();

		let final_values: Vec<f64> = result.0.0.0.f64().unwrap().into_no_null_iter().collect();
		let percent_returns: Vec<f64> = result.1.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(final_values.len(), 9);
		assert_eq!(percent_returns.len(), 9);

		// First transition: 100.0 -> 102.0
		assert_abs_diff_eq!(final_values[0], 1020.0, epsilon = 1e-10);
		assert_abs_diff_eq!(percent_returns[0], 2.0, epsilon = 1e-10);
	}

	#[test]
	fn test_true_range() {
		let ti = create_basic_other_ti();
		let result = ti.true_range("high", "low", "close").unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);

		// First period: max(105.0 - 99.0, |105.0 - 104.0|, |99.0 - 104.0|) = 6.0
		assert_abs_diff_eq!(values[0], 6.0, epsilon = 1e-10);
	}

	#[test]
	fn test_average_true_range_single() {
		let ti = create_basic_other_ti();
		let result = ti.average_true_range_single("high", "low", "close", "sma").unwrap();

		// Result should be a single ATR value
		assert!(result > 0.0);
	}

	#[test]
	fn test_average_true_range_bulk() {
		let ti = create_basic_other_ti();
		let result = ti.average_true_range_bulk("high", "low", "close", "sma", 3).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);

		// First few values should be NaN due to period requirement
		assert!(values[0].is_nan());
		assert!(values[1].is_nan());
		assert!(!values[2].is_nan());
	}

	#[test]
	fn test_internal_bar_strength() {
		let ti = create_basic_other_ti();
		let result = ti.internal_bar_strength("high", "low", "close").unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);

		// All IBS values should be between 0 and 1
		for &value in &values {
			assert!((0.0..=1.0).contains(&value));
		}

		// First period: (104.0 - 99.0) / (105.0 - 99.0) = 5.0 / 6.0 ≈ 0.833
		assert_abs_diff_eq!(values[0], 5.0 / 6.0, epsilon = 1e-10);
	}

	#[test]
	fn test_positivity_indicator() {
		let ti = create_basic_other_ti();
		let result = ti.positivity_indicator("open", "close", 3, "sma").unwrap();

		let positivity_values: Vec<f64> = result.0.0.0.f64().unwrap().into_no_null_iter().collect();
		let signal_values: Vec<f64> = result.1.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(positivity_values.len(), 10);
		assert_eq!(signal_values.len(), 10);

		// First few signal values should be NaN due to period requirement
		assert!(signal_values[0].is_nan());
		assert!(signal_values[1].is_nan());
		assert!(!signal_values[2].is_nan());
	}

	#[test]
	fn test_invalid_column_error() {
		let ti = create_basic_other_ti();

		let result = ti.return_on_investment_single("nonexistent_column", 1000.0);
		assert!(result.is_err());

		let result = ti.true_range("nonexistent_high", "low", "close");
		assert!(result.is_err());
	}

	#[test]
	fn test_invalid_constant_model_type() {
		let ti = create_basic_other_ti();

		let result = ti.average_true_range_single("high", "low", "close", "invalid_type");
		assert!(result.is_err());

		let result = ti.positivity_indicator("open", "close", 3, "invalid_type");
		assert!(result.is_err());
	}

	#[test]
	fn test_zero_investment() {
		let ti = create_basic_other_ti();
		let result = ti.return_on_investment_single("price", 0.0).unwrap();

		assert_abs_diff_eq!(result.0, 0.0, epsilon = 1e-10);
		assert_abs_diff_eq!(result.1, 9.0, epsilon = 1e-10); // Percentage should still be calculated
	}

	#[test]
	fn test_negative_investment() {
		let ti = create_basic_other_ti();
		let result = ti.return_on_investment_single("price", -1000.0).unwrap();

		// Negative investment should work mathematically
		assert_abs_diff_eq!(result.0, -1090.0, epsilon = 1e-10);
		assert_abs_diff_eq!(result.1, 9.0, epsilon = 1e-10);
	}

	#[test]
	fn test_zero_period_bulk() {
		let ti = create_basic_other_ti();
		let result = ti.average_true_range_bulk("high", "low", "close", "sma", 0);

		// Should handle zero period gracefully
		assert!(
			result.is_err() || {
				let values: Vec<f64> = result.unwrap().0.0.f64().unwrap().into_no_null_iter().collect();
				values.iter().all(|&x| x.is_nan())
			}
		);
	}

	#[test]
	fn test_large_period_bulk() {
		let ti = create_basic_other_ti();
		let result = ti.average_true_range_bulk("high", "low", "close", "sma", 20).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);

		// All values should be NaN since period > data length
		assert!(values.iter().take(9).all(|&x| x.is_nan()));
	}

	#[test]
	fn test_single_value_dataset() {
		let single_data = df! {
			"open" => vec![100.0],
			"high" => vec![105.0],
			"low" => vec![99.0],
			"close" => vec![104.0],
			"price" => vec![100.0]
		}
		.unwrap()
		.lazy();

		let ti = OtherTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(single_data)));

		let tr_result = ti.true_range("high", "low", "close").unwrap();
		let tr_values: Vec<f64> = tr_result.0.0.f64().unwrap().into_no_null_iter().collect();
		assert_eq!(tr_values.len(), 1);
		assert_abs_diff_eq!(tr_values[0], 6.0, epsilon = 1e-10); // 105.0 - 99.0

		let ibs_result = ti.internal_bar_strength("high", "low", "close").unwrap();
		let ibs_values: Vec<f64> = ibs_result.0.0.f64().unwrap().into_no_null_iter().collect();
		assert_eq!(ibs_values.len(), 1);
		assert_abs_diff_eq!(ibs_values[0], 5.0 / 6.0, epsilon = 1e-10);
	}

	#[test]
	fn test_identical_high_low_close() {
		let identical_data = df! {
			"open" => vec![100.0, 100.0, 100.0],
			"high" => vec![100.0, 100.0, 100.0],
			"low" => vec![100.0, 100.0, 100.0],
			"close" => vec![100.0, 100.0, 100.0],
			"price" => vec![100.0, 100.0, 100.0]
		}
		.unwrap()
		.lazy();

		let ti = OtherTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(identical_data)));

		let tr_result = ti.true_range("high", "low", "close").unwrap();
		let tr_values: Vec<f64> = tr_result.0.0.f64().unwrap().into_no_null_iter().collect();

		// All true range values should be 0 when high = low = close
		for &value in &tr_values {
			assert_abs_diff_eq!(value, 0.0, epsilon = 1e-10);
		}

		let ibs_result = ti.internal_bar_strength("high", "low", "close").unwrap();
		let ibs_values: Vec<f64> = ibs_result.0.0.f64().unwrap().into_no_null_iter().collect();

		// IBS should handle division by zero gracefully
		for &value in &ibs_values {
			assert!(value.is_nan() || value == 0.0 || value == 1.0);
		}
	}

	#[test]
	fn test_floating_point_precision() {
		let precision_data = df! {
			"open" => vec![100.000000001, 100.000000002, 100.000000003],
			"high" => vec![100.000000011, 100.000000012, 100.000000013],
			"low" => vec![99.999999991, 99.999999992, 99.999999993],
			"close" => vec![100.000000001, 100.000000002, 100.000000003],
			"price" => vec![100.000000001, 100.000000002, 100.000000003]
		}
		.unwrap()
		.lazy();

		let ti = OtherTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(precision_data)));

		let roi_result = ti.return_on_investment_single("price", 1000.0).unwrap();

		// Should handle small precision differences
		assert!(roi_result.0 > 999.0 && roi_result.0 < 1001.0);
		assert!(roi_result.1.abs() < 1.0);
	}

	#[test]
	fn test_different_moving_average_types() {
		let ti = create_basic_other_ti();

		let sma_result = ti.average_true_range_single("high", "low", "close", "sma").unwrap();
		let ema_result = ti.average_true_range_single("high", "low", "close", "ema").unwrap();

		// Results should be different for different MA types
		assert_ne!(sma_result, ema_result);

		// Both should be positive
		assert!(sma_result > 0.0);
		assert!(ema_result > 0.0);
	}

	#[test]
	fn test_cross_column_consistency() {
		let ti = create_basic_other_ti();

		let tr_result = ti.true_range("high", "low", "close").unwrap();
		let tr_values: Vec<f64> = tr_result.0.0.f64().unwrap().into_no_null_iter().collect();

		// True range should always be non-negative
		for &value in &tr_values {
			assert!(value >= 0.0);
		}

		let ibs_result = ti.internal_bar_strength("high", "low", "close").unwrap();
		let ibs_values: Vec<f64> = ibs_result.0.0.f64().unwrap().into_no_null_iter().collect();

		// IBS should always be between 0 and 1 (or NaN)
		for &value in &ibs_values {
			assert!(value.is_nan() || (0.0..=1.0).contains(&value));
		}
	}
}
