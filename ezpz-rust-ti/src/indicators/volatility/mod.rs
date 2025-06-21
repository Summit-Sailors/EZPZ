use {
	crate::utils::{extract_f64_values, parse_constant_model_type},
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct VolatilityTI;

#[gen_stub_pymethods]
#[pymethods]
impl VolatilityTI {
	/// Ulcer Index (Single) - Calculates how quickly the price is able to get back to its former high
	/// Can be used instead of standard deviation for volatility measurement
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of price values to analyze
	///
	/// # Returns
	/// f64 - Single Ulcer Index value representing overall price volatility and drawdown risk
	#[staticmethod]
	fn ulcer_index_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(prices)?;
		let result = rust_ti::volatility_indicators::single::ulcer_index(&values);
		Ok(result)
	}

	/// Ulcer Index (Bulk) - Calculates rolling Ulcer Index over specified period
	/// Returns a series of Ulcer Index values
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of price values to analyze
	/// - `period`: usize - Rolling window period for calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series of rolling Ulcer Index values with name "ulcer_index"
	#[staticmethod]
	fn ulcer_index_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;
		let result = rust_ti::volatility_indicators::bulk::ulcer_index(&values, &period);
		let result_series = Series::new("ulcer_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Volatility System - Calculates Welles volatility system with Stop and Reverse (SaR) points
	/// Uses trend analysis to determine long/short positions and calculate SaR levels
	/// Constant multiplier typically between 2.8-3.1 (Welles used 3.0)
	///
	/// # Parameters
	/// - `high`: PySeriesStubbed - Series of high price values
	/// - `low`: PySeriesStubbed - Series of low price values
	/// - `close`: PySeriesStubbed - Series of closing price values
	/// - `period`: usize - Period for volatility calculation
	/// - `constant_multiplier`: f64 - Multiplier for volatility (typically 2.8-3.1)
	/// - `constant_model_type`: &str - Type of constant model to use for calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series of volatility system values with Stop and Reverse points, named "volatility_system"
	#[staticmethod]
	fn volatility_system(
		high: PySeriesStubbed,
		low: PySeriesStubbed,
		close: PySeriesStubbed,
		period: usize,
		constant_multiplier: f64,
		constant_model_type: &str,
	) -> PyResult<PySeriesStubbed> {
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result =
			rust_ti::volatility_indicators::bulk::volatility_system(&high_values, &low_values, &close_values, &period, &constant_multiplier, &constant_type);
		let result_series = Series::new("volatility_system".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}
