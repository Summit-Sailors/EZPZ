use {
	crate::utils::{extract_f64_values, parse_central_point},
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct BasicTI {
	pub series: PySeriesStubbed,
}

#[gen_stub_pymethods]
#[pymethods]
impl BasicTI {
	#[new]
	fn new(series: PySeriesStubbed) -> Self {
		Self { series }
	}

	// Single value functions (return a single value from the entire prices)

	/// Calculate the arithmetic mean of all values.
	///
	/// # Returns
	/// f64 - The arithmetic mean
	fn mean_single(&self) -> PyResult<f64> {
		let values = extract_f64_values(self.series.clone())?;
		Ok(rust_ti::basic_indicators::single::mean(&values))
	}

	/// Calculate the median of all values.
	///
	/// # Returns
	/// f64 - The median value
	fn median_single(&self) -> PyResult<f64> {
		let values = extract_f64_values(self.series.clone())?;
		Ok(rust_ti::basic_indicators::single::median(&values))
	}

	/// Calculate the mode of all values.
	///
	/// # Returns
	/// f64 - The most frequently occurring value
	fn mode_single(&self) -> PyResult<f64> {
		let values = extract_f64_values(self.series.clone())?;
		Ok(rust_ti::basic_indicators::single::mode(&values))
	}

	/// Calculate the variance of all values.
	///
	/// # Returns
	/// f64 - The variance
	fn variance_single(&self) -> PyResult<f64> {
		let values = extract_f64_values(self.series.clone())?;
		Ok(rust_ti::basic_indicators::single::variance(&values))
	}

	/// Calculate the standard deviation of all values.
	///
	/// # Returns
	/// f64 - The standard deviation
	fn standard_deviation_single(&self) -> PyResult<f64> {
		let values = extract_f64_values(self.series.clone())?;
		Ok(rust_ti::basic_indicators::single::standard_deviation(&values))
	}

	/// Find the maximum value.
	///
	/// # Returns
	/// f64 - The maximum value
	fn max_single(&self) -> PyResult<f64> {
		let values = extract_f64_values(self.series.clone())?;
		Ok(rust_ti::basic_indicators::single::max(&values))
	}

	/// Find the minimum value.
	///
	/// # Returns
	/// f64 - The minimum value
	fn min_single(&self) -> PyResult<f64> {
		let values = extract_f64_values(self.series.clone())?;
		Ok(rust_ti::basic_indicators::single::min(&values))
	}

	/// Calculate the absolute deviation from a central point.
	///
	/// # Parameters
	/// - `central_point`: &str - Central point type ("mean", "median", etc.)
	///
	/// # Returns
	/// f64 - The absolute deviation
	fn absolute_deviation_single(&self, central_point: &str) -> PyResult<f64> {
		let values = extract_f64_values(self.series.clone())?;
		let cp = parse_central_point(central_point)?;
		Ok(rust_ti::basic_indicators::single::absolute_deviation(&values, cp))
	}

	/// Calculate the logarithmic difference between two price points.
	///
	/// # Parameters
	/// - `price_t`: f64 - Current price value
	/// - `price_t_1`: f64 - Previous price value
	///
	/// # Returns
	/// f64 - The logarithmic difference
	fn log_difference_single(&self, price_t: f64, price_t_1: f64) -> PyResult<f64> {
		Ok(rust_ti::basic_indicators::single::log_difference(price_t, price_t_1))
	}

	// Bulk functions (return prices with rolling calculations)

	/// Calculate rolling mean over a specified period.
	///
	/// # Parameters
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling mean values
	fn mean_bulk(&self, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(self.series.clone())?;
		let result = rust_ti::basic_indicators::bulk::mean(&values, period);
		let result_series = Series::new("mean".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate rolling median over a specified period.
	///
	/// # Parameters
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling median values
	fn median_bulk(&self, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(self.series.clone())?;
		let result = rust_ti::basic_indicators::bulk::median(&values, period);
		let result_series = Series::new("median".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate rolling mode over a specified period.
	///
	/// # Parameters
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling mode values
	fn mode_bulk(&self, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(self.series.clone())?;
		let result = rust_ti::basic_indicators::bulk::mode(&values, period);
		let result_series = Series::new("mode".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate rolling variance over a specified period.
	///
	/// # Parameters
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling variance values
	fn variance_bulk(&self, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(self.series.clone())?;
		let result = rust_ti::basic_indicators::bulk::variance(&values, period);
		let result_series = Series::new("variance".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate rolling standard deviation over a specified period.
	///
	/// # Parameters
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling standard deviation values
	fn standard_deviation_bulk(&self, period: usize) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(self.series.clone())?;
		let result = rust_ti::basic_indicators::bulk::standard_deviation(&values, period);
		let result_series = Series::new("standard_deviation".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate rolling absolute deviation over a specified period.
	///
	/// # Parameters
	/// - `period`: usize - Rolling window size
	/// - `central_point`: &str - Central point type ("mean", "median", etc.)
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling absolute deviation values
	fn absolute_deviation_bulk(&self, period: usize, central_point: &str) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(self.series.clone())?;
		let cp = parse_central_point(central_point)?;
		let result = rust_ti::basic_indicators::bulk::absolute_deviation(&values, period, cp);
		let result_series = Series::new("absolute_deviation".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate natural logarithm of all values.
	///
	/// # Returns
	/// PySeriesStubbed - Series containing natural logarithm values
	fn log_bulk(&self) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(self.series.clone())?;
		let result = rust_ti::basic_indicators::bulk::log(&values);
		let result_series = Series::new("log".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate logarithmic differences between consecutive values.
	///
	/// # Returns
	/// PySeriesStubbed - Series containing logarithmic difference values
	fn log_difference_bulk(&self) -> PyResult<PySeriesStubbed> {
		let values = extract_f64_values(self.series.clone())?;
		let result = rust_ti::basic_indicators::bulk::log_difference(&values);
		let result_series = Series::new("log_difference".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}
