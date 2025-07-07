use {
	crate::utils::{extract_f64_values, parse_constant_model_type, parse_deviation_model},
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct CorrelationTI {
	pub series: PySeriesStubbed,
}

#[gen_stub_pymethods]
#[pymethods]
impl CorrelationTI {
	#[new]
	fn new(series: PySeriesStubbed) -> Self {
		Self { series }
	}

	/// Correlation between two assets - Single value calculation
	/// Calculates correlation between prices of two assets using specified models
	/// Returns a single correlation value for the entire price series
	///
	/// # Parameters
	/// - `other_asset_prices`: PySeriesStubbed - Price series for the second asset
	/// - `constant_model_type`: &str - Type of constant model to use for correlation calculation
	/// - `deviation_model`: &str - Type of deviation model to use for correlation calculation
	///
	/// # Returns
	/// f64 - Single correlation coefficient between the two asset price series
	fn correlate_asset_prices_single(&self, other_asset_prices: PySeriesStubbed, constant_model_type: &str, deviation_model: &str) -> PyResult<f64> {
		let values_a: Vec<f64> = extract_f64_values(self.series.clone())?;
		let values_b: Vec<f64> = extract_f64_values(other_asset_prices)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;
		let result = rust_ti::correlation_indicators::single::correlate_asset_prices(&values_a, &values_b, constant_type, deviation_type);
		Ok(result)
	}

	/// Correlation between two assets - Rolling/Bulk calculation
	/// Calculates rolling correlation between prices of two assets using specified models
	/// Returns a series of correlation values for each period window
	///
	/// # Parameters
	/// - `other_asset_prices`: PySeriesStubbed - Price series for the second asset
	/// - `constant_model_type`: &str - Type of constant model to use for correlation calculation
	/// - `deviation_model`: &str - Type of deviation model to use for correlation calculation
	/// - `period`: usize - Rolling window size for correlation calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling correlation coefficients for each period window with name "correlation"
	fn correlate_asset_prices_bulk(
		&self,
		other_asset_prices: PySeriesStubbed,
		constant_model_type: &str,
		deviation_model: &str,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let values_a: Vec<f64> = extract_f64_values(self.series.clone())?;
		let values_b: Vec<f64> = extract_f64_values(other_asset_prices)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;
		let result = rust_ti::correlation_indicators::bulk::correlate_asset_prices(&values_a, &values_b, constant_type, deviation_type, period);
		let correlation_series = Series::new("correlation".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(correlation_series)))
	}
}
