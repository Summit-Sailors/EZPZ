use {
	crate::utils::{extract_f64_values, parse_constant_model_type},
	ezpz_stubz::{lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct VolatilityTI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl VolatilityTI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	/// Ulcer Index (Single) - Calculates how quickly the price is able to get back to its former high
	/// Can be used instead of standard deviation for volatility measurement
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// f64 - Single Ulcer Index value representing overall price volatility and drawdown risk
	fn ulcer_index_single(&self, price_column: &str) -> PyResult<f64> {
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
		let result = rust_ti::volatility_indicators::single::ulcer_index(&values);
		Ok(result)
	}

	/// Ulcer Index (Bulk) - Calculates rolling Ulcer Index over specified period
	/// Returns a series of Ulcer Index values
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `period`: usize - Rolling window period for calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series of rolling Ulcer Index values with name "ulcer_index"
	fn ulcer_index_bulk(&self, price_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
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
		let result = rust_ti::volatility_indicators::bulk::ulcer_index(&values, period);
		let result_series = Series::new("ulcer_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Volatility System - Calculates Welles volatility system with Stop and Reverse (SaR) points
	/// Uses trend analysis to determine long/short positions and calculate SaR levels
	/// Constant multiplier typically between 2.8-3.1 (Welles used 3.0)
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `period`: usize - Period for volatility calculation
	/// - `constant_multiplier`: f64 - Multiplier for volatility (typically 2.8-3.1)
	/// - `constant_model_type`: &str - Type of constant model to use for calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series of volatility system values with Stop and Reverse points, named "volatility_system"
	fn volatility_system(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		period: usize,
		constant_multiplier: f64,
		constant_model_type: &str,
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
		let result = rust_ti::volatility_indicators::bulk::volatility_system(&high_values, &low_values, &close_values, period, constant_multiplier, constant_type);
		let result_series = Series::new("volatility_system".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;

	fn create_test_dataframe() -> LazyFrame {
		let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
		df! {
				"high" => &data,
				"low" => vec![0.9, 1.8, 2.7, 3.6, 4.5, 5.4, 6.3, 7.2, 8.1, 9.0],
				"close" => vec![0.95, 1.9, 2.85, 3.8, 4.75, 5.7, 6.65, 7.6, 8.55, 9.5],
				"volume" => vec![100.0, 200.0, 150.0, 300.0, 250.0, 180.0, 220.0, 190.0, 280.0, 320.0]
		}
		.unwrap()
		.lazy()
	}

	fn create_volatility_ti() -> VolatilityTI {
		let lf = create_test_dataframe();
		VolatilityTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_ulcer_index_single() {
		let ti = create_volatility_ti();
		let result = ti.ulcer_index_single("close").unwrap();
		// Expected value calculated using rust_ti::volatility_indicators::single::ulcer_index
		// For the close prices [0.95, 1.9, 2.85, 3.8, 4.75, 5.7, 6.65, 7.6, 8.55, 9.5]
		// Ulcer Index involves calculating drawdowns from the highest close price
		let expected = 0.0; // No drawdowns as prices are strictly increasing
		assert_abs_diff_eq!(result, expected, epsilon = 1e-10);
	}

	#[test]
	fn test_ulcer_index_single_invalid_column() {
		let ti = create_volatility_ti();
		let result = ti.ulcer_index_single("invalid_column");
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "PyValueError: Column 'invalid_column' not found: column not found");
	}

	#[test]
	fn test_ulcer_index_bulk() {
		let ti = create_volatility_ti();
		let period = 3;
		let result = ti.ulcer_index_bulk("close", period).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_iter().map(|opt| opt.unwrap_or(f64::NAN)).collect();

		assert_eq!(values.len(), 10);
		assert!(values[0].is_nan());
		assert!(values[1].is_nan());
		// For period=3, the first non-NaN value at index 2 is the Ulcer Index of [0.95, 1.9, 2.85]
		// Since prices are increasing, drawdowns are 0, so Ulcer Index should be 0.0
		assert_abs_diff_eq!(values[2], 0.0, epsilon = 1e-10);
		assert_abs_diff_eq!(values[3], 0.0, epsilon = 1e-10);
	}

	#[test]
	fn test_ulcer_index_bulk_invalid_column() {
		let ti = create_volatility_ti();
		let result = ti.ulcer_index_bulk("invalid_column", 3);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "PyValueError: Column 'invalid_column' not found: column not found");
	}

	#[test]
	fn test_volatility_system() {
		let ti = create_volatility_ti();
		let period = 3;
		let constant_multiplier = 3.0;
		let constant_model_type = "Simple";
		let result = ti.volatility_system("high", "low", "close", period, constant_multiplier, constant_model_type).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_iter().map(|opt| opt.unwrap_or(f64::NAN)).collect();

		assert_eq!(values.len(), 10);
		assert!(values[0].is_nan());
		assert!(values[1].is_nan());
		// Expected values depend on rust_ti::volatility_indicators::bulk::volatility_system
		// For simplicity, verify that non-NaN values are finite and reasonable
		for &value in values.iter().skip(2) {
			assert!(value.is_finite());
		}
	}

	#[test]
	fn test_volatility_system_invalid_column() {
		let ti = create_volatility_ti();
		let result = ti.volatility_system("invalid_column", "low", "close", 3, 3.0, "Simple");
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "PyValueError: Column 'invalid_column' not found: column not found");
	}

	#[test]
	fn test_volatility_system_invalid_model_type() {
		let ti = create_volatility_ti();
		let result = ti.volatility_system("high", "low", "close", 3, 3.0, "InvalidModel");
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "PyValueError: Invalid constant model type: InvalidModel");
	}

	#[test]
	fn test_volatility_system_empty_dataframe() {
		let empty_lf = df! { "high" => Vec::<f64>::new(), "low" => Vec::<f64>::new(), "close" => Vec::<f64>::new() }.unwrap().lazy();
		let ti = VolatilityTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(empty_lf)));
		let result = ti.volatility_system("high", "low", "close", 3, 3.0, "Simple");
		assert!(result.is_ok());
		let values: Vec<f64> = result.unwrap().0.0.f64().unwrap().into_iter().map(|opt| opt.unwrap_or(f64::NAN)).collect();
		assert!(values.is_empty());
	}
}
