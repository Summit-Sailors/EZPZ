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
pub struct StrengthTI;

#[gen_stub_pymethods]
#[pymethods]
impl StrengthTI {
	/// Accumulation Distribution - Shows whether the stock is being accumulated or distributed
	#[staticmethod]
	fn accumulation_distribution(
		high: PySeriesStubbed,
		low: PySeriesStubbed,
		close: PySeriesStubbed,
		volume: PySeriesStubbed,
		previous_ad: Option<f64>,
	) -> PyResult<PySeriesStubbed> {
		let high_series: Series = high.0.into();
		let low_series: Series = low.0.into();
		let close_series: Series = close.0.into();
		let volume_series: Series = volume.0.into();

		let high_values: Vec<f64> = high_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let low_values: Vec<f64> = low_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let close_values: Vec<f64> = close_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let volume_values: Vec<f64> = volume_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let previous = previous_ad.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::bulk::accumulation_distribution(&high_values, &low_values, &close_values, &volume_values, &previous);

		let result_series = Series::new("accumulation_distribution".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Positive Volume Index - Measures volume trend strength when volume increases
	#[staticmethod]
	fn positive_volume_index(close: PySeriesStubbed, volume: PySeriesStubbed, previous_pvi: Option<f64>) -> PyResult<PySeriesStubbed> {
		let close_series: Series = close.0.into();
		let volume_series: Series = volume.0.into();

		let close_values: Vec<f64> = close_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let volume_values: Vec<f64> = volume_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let previous = previous_pvi.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::bulk::positive_volume_index(&close_values, &volume_values, &previous);

		let result_series = Series::new("positive_volume_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Negative Volume Index - Measures volume trend strength when volume decreases
	#[staticmethod]
	fn negative_volume_index(close: PySeriesStubbed, volume: PySeriesStubbed, previous_nvi: Option<f64>) -> PyResult<PySeriesStubbed> {
		let close_series: Series = close.0.into();
		let volume_series: Series = volume.0.into();

		let close_values: Vec<f64> = close_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let volume_values: Vec<f64> = volume_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let previous = previous_nvi.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::bulk::negative_volume_index(&close_values, &volume_values, &previous);

		let result_series = Series::new("negative_volume_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Relative Vigor Index - Measures the strength of an asset by looking at previous prices
	#[staticmethod]
	fn relative_vigor_index(
		open: PySeriesStubbed,
		high: PySeriesStubbed,
		low: PySeriesStubbed,
		close: PySeriesStubbed,
		constant_model_type: &str,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let open_series: Series = open.0.into();
		let high_series: Series = high.0.into();
		let low_series: Series = low.0.into();
		let close_series: Series = close.0.into();

		let open_values: Vec<f64> = open_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let high_values: Vec<f64> = high_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let low_values: Vec<f64> = low_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let close_values: Vec<f64> = close_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::strength_indicators::bulk::relative_vigor_index(&open_values, &high_values, &low_values, &close_values, &constant_type, &period);

		let result_series = Series::new("relative_vigor_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Single Accumulation Distribution - Single value calculation
	#[staticmethod]
	fn single_accumulation_distribution(high: f64, low: f64, close: f64, volume: f64, previous_ad: Option<f64>) -> PyResult<f64> {
		let previous = previous_ad.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::single::accumulation_distribution(&high, &low, &close, &volume, &previous);
		Ok(result)
	}

	/// Single Volume Index - Generic version of PVI and NVI for single calculation
	#[staticmethod]
	fn single_volume_index(current_close: f64, previous_close: f64, previous_volume_index: Option<f64>) -> PyResult<f64> {
		let previous = previous_volume_index.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::single::volume_index(&current_close, &previous_close, &previous);
		Ok(result)
	}

	/// Single Relative Vigor Index - Single value calculation
	#[staticmethod]
	fn single_relative_vigor_index(
		open: PySeriesStubbed,
		high: PySeriesStubbed,
		low: PySeriesStubbed,
		close: PySeriesStubbed,
		constant_model_type: &str,
	) -> PyResult<f64> {
		let open_series: Series = open.0.into();
		let high_series: Series = high.0.into();
		let low_series: Series = low.0.into();
		let close_series: Series = close.0.into();

		let open_values: Vec<f64> = open_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let high_values: Vec<f64> = high_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let low_values: Vec<f64> = low_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let close_values: Vec<f64> = close_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::strength_indicators::single::relative_vigor_index(&open_values, &high_values, &low_values, &close_values, &constant_type);

		Ok(result)
	}
}
