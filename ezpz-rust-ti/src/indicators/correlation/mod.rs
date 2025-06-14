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
pub struct CorrelationTI;

#[gen_stub_pymethods]
#[pymethods]
impl CorrelationTI {
	/// Correlation between two assets - Single value calculation
	/// Calculates correlation between prices of two assets using specified models
	/// Returns a single correlation value for the entire price series
	#[staticmethod]
	fn correlate_asset_prices_single(
		prices_asset_a: PySeriesStubbed,
		prices_asset_b: PySeriesStubbed,
		constant_model_type: &str,
		deviation_model: &str,
	) -> PyResult<f64> {
		let values_a: Vec<f64> = extract_f64_values(prices_asset_a)?;
		let values_b: Vec<f64> = extract_f64_values(prices_asset_b)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;

		let result = rust_ti::correlation_indicators::single::correlate_asset_prices(&values_a, &values_b, &constant_type, &deviation_type);

		Ok(result)
	}

	/// Correlation between two assets - Rolling/Bulk calculation
	/// Calculates rolling correlation between prices of two assets using specified models
	/// Returns a series of correlation values for each period window
	#[staticmethod]
	fn correlate_asset_prices_bulk(
		prices_asset_a: PySeriesStubbed,
		prices_asset_b: PySeriesStubbed,
		constant_model_type: &str,
		deviation_model: &str,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let values_a: Vec<f64> = extract_f64_values(prices_asset_a)?;
		let values_b: Vec<f64> = extract_f64_values(prices_asset_b)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;

		let result = rust_ti::correlation_indicators::bulk::correlate_asset_prices(&values_a, &values_b, &constant_type, &deviation_type, &period);

		let correlation_series = Series::new("correlation".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(correlation_series)))
	}
}
