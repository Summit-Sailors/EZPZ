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
