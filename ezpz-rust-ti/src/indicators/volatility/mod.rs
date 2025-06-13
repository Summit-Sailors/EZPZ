use {
	crate::utils::parse_constant_model_type,
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
	#[staticmethod]
	fn ulcer_index_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::volatility_indicators::single::ulcer_index(&values);
		Ok(result)
	}

	/// Ulcer Index (Bulk) - Calculates rolling Ulcer Index over specified period
	/// Returns a series of Ulcer Index values
	#[staticmethod]
	fn ulcer_index_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::volatility_indicators::bulk::ulcer_index(&values, &period);
		let result_series = Series::new("ulcer_index".into(), result);

		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Volatility System - Calculates Welles volatility system with Stop and Reverse (SaR) points
	/// Uses trend analysis to determine long/short positions and calculate SaR levels
	/// Constant multiplier typically between 2.8-3.1 (Welles used 3.0)
	#[staticmethod]
	fn volatility_system(
		high: PySeriesStubbed,
		low: PySeriesStubbed,
		close: PySeriesStubbed,
		period: usize,
		constant_multiplier: f64,
		constant_model_type: &str,
	) -> PyResult<PySeriesStubbed> {
		let high_series: Series = high.0.into();
		let high_values: Vec<f64> = high_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let low_series: Series = low.0.into();
		let low_values: Vec<f64> = low_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let close_series: Series = close.0.into();
		let close_values: Vec<f64> = close_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let constant_type = parse_constant_model_type(constant_model_type)?;

		let result =
			rust_ti::volatility_indicators::bulk::volatility_system(&high_values, &low_values, &close_values, &period, &constant_multiplier, &constant_type);

		let result_series = Series::new("volatility_system".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}
