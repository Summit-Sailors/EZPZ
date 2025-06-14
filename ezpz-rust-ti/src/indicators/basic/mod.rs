use {
	crate::utils::{create_result_series, extract_f64_values, parse_central_point},
	ezpz_stubz::series::PySeriesStubbed,
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
		let values = extract_f64_values(series)?;
		Ok(rust_ti::basic_indicators::single::mean(&values))
	}

	#[staticmethod]
	fn median_single(series: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(series)?;
		Ok(rust_ti::basic_indicators::single::median(&values))
	}

	#[staticmethod]
	fn mode_single(series: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(series)?;
		Ok(rust_ti::basic_indicators::single::mode(&values))
	}

	#[staticmethod]
	fn variance_single(series: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(series)?;
		Ok(rust_ti::basic_indicators::single::variance(&values))
	}

	#[staticmethod]
	fn standard_deviation_single(series: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(series)?;
		Ok(rust_ti::basic_indicators::single::standard_deviation(&values))
	}

	#[staticmethod]
	fn max_single(series: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(series)?;
		Ok(rust_ti::basic_indicators::single::max(&values))
	}

	#[staticmethod]
	fn min_single(series: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(series)?;
		Ok(rust_ti::basic_indicators::single::min(&values))
	}

	#[staticmethod]
	fn absolute_deviation_single(series: PySeriesStubbed, central_point: &str) -> PyResult<f64> {
		let values = extract_f64_values(series)?;
		let cp = parse_central_point(central_point)?;
		Ok(rust_ti::basic_indicators::single::absolute_deviation(&values, &cp))
	}

	#[staticmethod]
	fn log_difference_single(price_t: f64, price_t_1: f64) -> PyResult<f64> {
		Ok(rust_ti::basic_indicators::single::log_difference(&price_t, &price_t_1))
	}

	// Bulk functions (return series with rolling calculations)

	#[staticmethod]
	fn mean_bulk(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(series)?;
		let result = rust_ti::basic_indicators::bulk::mean(&values, &period);
		Ok(create_result_series("mean", result))
	}

	#[staticmethod]
	fn median_bulk(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(series)?;
		let result = rust_ti::basic_indicators::bulk::median(&values, &period);
		Ok(create_result_series("median", result))
	}

	#[staticmethod]
	fn mode_bulk(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(series)?;
		let result = rust_ti::basic_indicators::bulk::mode(&values, &period);
		Ok(create_result_series("mode", result))
	}

	#[staticmethod]
	fn variance_bulk(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(series)?;
		let result = rust_ti::basic_indicators::bulk::variance(&values, &period);
		Ok(create_result_series("variance", result))
	}

	#[staticmethod]
	fn standard_deviation_bulk(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(series)?;
		let result = rust_ti::basic_indicators::bulk::standard_deviation(&values, &period);
		Ok(create_result_series("standard_deviation", result))
	}

	#[staticmethod]
	fn absolute_deviation_bulk(series: PySeriesStubbed, period: usize, central_point: &str) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(series)?;
		let cp = parse_central_point(central_point)?;
		let result = rust_ti::basic_indicators::bulk::absolute_deviation(&values, &period, &cp);
		Ok(create_result_series("absolute_deviation", result))
	}

	#[staticmethod]
	fn log_bulk(series: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(series)?;
		let result = rust_ti::basic_indicators::bulk::log(&values);
		Ok(create_result_series("log", result))
	}

	#[staticmethod]
	fn log_difference_bulk(series: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(series)?;
		let result = rust_ti::basic_indicators::bulk::log_difference(&values);
		Ok(create_result_series("log_difference", result))
	}
}
