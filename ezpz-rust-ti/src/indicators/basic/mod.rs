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
	// Single value functions (return a single value from the entire prices)

	/// Calculate the arithmetic mean of all values.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	///
	/// # Returns
	/// f64 - The arithmetic mean
	#[staticmethod]
	fn mean_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(prices)?;
		Ok(rust_ti::basic_indicators::single::mean(&values))
	}

	/// Calculate the median of all values.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	///
	/// # Returns
	/// f64 - The median value
	#[staticmethod]
	fn median_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(prices)?;
		Ok(rust_ti::basic_indicators::single::median(&values))
	}

	/// Calculate the mode of all values.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	///
	/// # Returns
	/// f64 - The most frequently occurring value
	#[staticmethod]
	fn mode_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(prices)?;
		Ok(rust_ti::basic_indicators::single::mode(&values))
	}

	/// Calculate the variance of all values.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	///
	/// # Returns
	/// f64 - The variance
	#[staticmethod]
	fn variance_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(prices)?;
		Ok(rust_ti::basic_indicators::single::variance(&values))
	}

	/// Calculate the standard deviation of all values.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	///
	/// # Returns
	/// f64 - The standard deviation
	#[staticmethod]
	fn standard_deviation_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(prices)?;
		Ok(rust_ti::basic_indicators::single::standard_deviation(&values))
	}

	/// Find the maximum value.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	///
	/// # Returns
	/// f64 - The maximum value
	#[staticmethod]
	fn max_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(prices)?;
		Ok(rust_ti::basic_indicators::single::max(&values))
	}

	/// Find the minimum value.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	///
	/// # Returns
	/// f64 - The minimum value
	#[staticmethod]
	fn min_single(prices: PySeriesStubbed) -> PyResult<f64> {
		let values = extract_f64_values(prices)?;
		Ok(rust_ti::basic_indicators::single::min(&values))
	}

	/// Calculate the absolute deviation from a central point.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	/// - `central_point`: &str - Central point type ("mean", "median", etc.)
	///
	/// # Returns
	/// f64 - The absolute deviation
	#[staticmethod]
	fn absolute_deviation_single(prices: PySeriesStubbed, central_point: &str) -> PyResult<f64> {
		let values = extract_f64_values(prices)?;
		let cp = parse_central_point(central_point)?;
		Ok(rust_ti::basic_indicators::single::absolute_deviation(&values, &cp))
	}

	/// Calculate the logarithmic difference between two price points.
	///
	/// # Parameters
	/// - `price_t`: f64 - Current price value
	/// - `price_t_1`: f64 - Previous price value
	///
	/// # Returns
	/// f64 - The logarithmic difference
	#[staticmethod]
	fn log_difference_single(price_t: f64, price_t_1: f64) -> PyResult<f64> {
		Ok(rust_ti::basic_indicators::single::log_difference(&price_t, &price_t_1))
	}

	// Bulk functions (return prices with rolling calculations)

	/// Calculate rolling mean over a specified period.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling mean values
	#[staticmethod]
	fn mean_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(prices)?;
		let result = rust_ti::basic_indicators::bulk::mean(&values, &period);
		Ok(create_result_series("mean", result))
	}

	/// Calculate rolling median over a specified period.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling median values
	#[staticmethod]
	fn median_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(prices)?;
		let result = rust_ti::basic_indicators::bulk::median(&values, &period);
		Ok(create_result_series("median", result))
	}

	/// Calculate rolling mode over a specified period.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling mode values
	#[staticmethod]
	fn mode_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(prices)?;
		let result = rust_ti::basic_indicators::bulk::mode(&values, &period);
		Ok(create_result_series("mode", result))
	}

	/// Calculate rolling variance over a specified period.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling variance values
	#[staticmethod]
	fn variance_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(prices)?;
		let result = rust_ti::basic_indicators::bulk::variance(&values, &period);
		Ok(create_result_series("variance", result))
	}

	/// Calculate rolling standard deviation over a specified period.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling standard deviation values
	#[staticmethod]
	fn standard_deviation_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(prices)?;
		let result = rust_ti::basic_indicators::bulk::standard_deviation(&values, &period);
		Ok(create_result_series("standard_deviation", result))
	}

	/// Calculate rolling absolute deviation over a specified period.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	/// - `period`: usize - Rolling window size
	/// - `central_point`: &str - Central point type ("mean", "median", etc.)
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling absolute deviation values
	#[staticmethod]
	fn absolute_deviation_bulk(prices: PySeriesStubbed, period: usize, central_point: &str) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(prices)?;
		let cp = parse_central_point(central_point)?;
		let result = rust_ti::basic_indicators::bulk::absolute_deviation(&values, &period, &cp);
		Ok(create_result_series("absolute_deviation", result))
	}

	/// Calculate natural logarithm of all values.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	///
	/// # Returns
	/// PySeriesStubbed - Series containing natural logarithm values
	#[staticmethod]
	fn log_bulk(prices: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(prices)?;
		let result = rust_ti::basic_indicators::bulk::log(&values);
		Ok(create_result_series("log", result))
	}

	/// Calculate logarithmic differences between consecutive values.
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of numeric values
	///
	/// # Returns
	/// PySeriesStubbed - Series containing logarithmic difference values
	#[staticmethod]
	fn log_difference_bulk(prices: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(prices)?;
		let result = rust_ti::basic_indicators::bulk::log_difference(&values);
		Ok(create_result_series("log_difference", result))
	}
}
