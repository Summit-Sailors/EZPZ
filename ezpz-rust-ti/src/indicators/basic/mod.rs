use {
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct BasicTI;

#[gen_stub_pymethods]
#[pymethods]
impl BasicTI {
	// Single value functions (return a single value from the entire series)

	#[staticmethod]
	fn mean_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::single::mean(&values);
		Ok(result)
	}

	#[staticmethod]
	fn median_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::single::median(&values);
		Ok(result)
	}

	#[staticmethod]
	fn mode_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::single::mode(&values);
		Ok(result)
	}

	#[staticmethod]
	fn variance_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::single::variance(&values);
		Ok(result)
	}

	#[staticmethod]
	fn standard_deviation_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::single::standard_deviation(&values);
		Ok(result)
	}

	#[staticmethod]
	fn max_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::single::max(&values);
		Ok(result)
	}

	#[staticmethod]
	fn min_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::single::min(&values);
		Ok(result)
	}

	#[staticmethod]
	fn absolute_deviation_single(series: PySeriesStubbed, central_point: &str) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let cp = match central_point.to_lowercase().as_str() {
			"mean" => rust_ti::CentralPoint::Mean,
			"median" => rust_ti::CentralPoint::Median,
			"mode" => rust_ti::CentralPoint::Mode,
			_ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("central_point must be 'mean', 'median', or 'mode'")),
		};

		let result = rust_ti::basic_indicators::single::absolute_deviation(&values, &cp);
		Ok(result)
	}

	#[staticmethod]
	fn log_difference_single(price_t: f64, price_t_1: f64) -> PyResult<f64> {
		let result = rust_ti::basic_indicators::single::log_difference(&price_t, &price_t_1);
		Ok(result)
	}

	// Bulk functions (return series with rolling calculations)

	#[staticmethod]
	fn mean_bulk(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::bulk::mean(&values, &period);
		let result_series = Series::new("mean".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	#[staticmethod]
	fn median_bulk(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::bulk::median(&values, &period);
		let result_series = Series::new("median".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	#[staticmethod]
	fn mode_bulk(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::bulk::mode(&values, &period);
		let result_series = Series::new("mode".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	#[staticmethod]
	fn variance_bulk(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::bulk::variance(&values, &period);
		let result_series = Series::new("variance".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	#[staticmethod]
	fn standard_deviation_bulk(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::bulk::standard_deviation(&values, &period);
		let result_series = Series::new("standard_deviation".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	#[staticmethod]
	fn absolute_deviation_bulk(series: PySeriesStubbed, period: usize, central_point: &str) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let cp = match central_point.to_lowercase().as_str() {
			"mean" => rust_ti::CentralPoint::Mean,
			"median" => rust_ti::CentralPoint::Median,
			"mode" => rust_ti::CentralPoint::Mode,
			_ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("central_point must be 'mean', 'median', or 'mode'")),
		};

		let result = rust_ti::basic_indicators::bulk::absolute_deviation(&values, &period, &cp);
		let result_series = Series::new("absolute_deviation".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	#[staticmethod]
	fn log_bulk(series: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::bulk::log(&values);
		let result_series = Series::new("log".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	#[staticmethod]
	fn log_difference_bulk(series: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let result = rust_ti::basic_indicators::bulk::log_difference(&values);
		let result_series = Series::new("log_difference".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}
