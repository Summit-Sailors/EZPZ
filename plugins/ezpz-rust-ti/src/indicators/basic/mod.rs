use {
	crate::utils::{extract_f64_values, parse_central_point},
	ezpz_stubz::{lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

/// Basic Technical Indicators - A collection of basic analysis functions for financial data

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct BasicTI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl BasicTI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	// Single value functions (return a single value from the entire prices)

	/// Calculate the arithmetic mean of all values.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	///
	/// # Returns
	/// f64 - The arithmetic mean
	fn mean_single(&self, column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		Ok(rust_ti::basic_indicators::single::mean(&values))
	}

	/// Calculate the median of all values.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	///
	/// # Returns
	/// f64 - The median value
	fn median_single(&self, column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		Ok(rust_ti::basic_indicators::single::median(&values))
	}

	/// Calculate the mode of all values.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	///
	/// # Returns
	/// f64 - The most frequently occurring value
	fn mode_single(&self, column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		Ok(rust_ti::basic_indicators::single::mode(&values))
	}

	/// Calculate the variance of all values.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	///
	/// # Returns
	/// f64 - The variance
	fn variance_single(&self, column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		Ok(rust_ti::basic_indicators::single::variance(&values))
	}

	/// Calculate the standard deviation of all values.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	///
	/// # Returns
	/// f64 - The standard deviation
	fn standard_deviation_single(&self, column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		Ok(rust_ti::basic_indicators::single::standard_deviation(&values))
	}

	/// Find the maximum value.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	///
	/// # Returns
	/// f64 - The maximum value
	fn max_single(&self, column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		Ok(rust_ti::basic_indicators::single::max(&values))
	}

	/// Find the minimum value.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	///
	/// # Returns
	/// f64 - The minimum value
	fn min_single(&self, column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		Ok(rust_ti::basic_indicators::single::min(&values))
	}

	/// Calculate the absolute deviation from a central point.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	/// - `central_point`: &str - Central point type ("mean", "median", etc.)
	///
	/// # Returns
	/// f64 - The absolute deviation
	fn absolute_deviation_single(&self, column: &str, central_point: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
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
	/// - `column`: &str - Name of the column to analyze
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling mean values
	fn mean_bulk(&self, column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::basic_indicators::bulk::mean(&values, period);
		let result_series = Series::new("mean".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate rolling median over a specified period.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling median values
	fn median_bulk(&self, column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::basic_indicators::bulk::median(&values, period);
		let result_series = Series::new("median".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate rolling mode over a specified period.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling mode values
	fn mode_bulk(&self, column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::basic_indicators::bulk::mode(&values, period);
		let result_series = Series::new("mode".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate rolling variance over a specified period.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling variance values
	fn variance_bulk(&self, column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::basic_indicators::bulk::variance(&values, period);
		let result_series = Series::new("variance".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate rolling standard deviation over a specified period.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	/// - `period`: usize - Rolling window size
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling standard deviation values
	fn standard_deviation_bulk(&self, column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::basic_indicators::bulk::standard_deviation(&values, period);
		let result_series = Series::new("standard_deviation".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate rolling absolute deviation over a specified period.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	/// - `period`: usize - Rolling window size
	/// - `central_point`: &str - Central point type ("mean", "median", etc.)
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling absolute deviation values
	fn absolute_deviation_bulk(&self, column: &str, period: usize, central_point: &str) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let cp = parse_central_point(central_point)?;
		let result = rust_ti::basic_indicators::bulk::absolute_deviation(&values, period, cp);
		let result_series = Series::new("absolute_deviation".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate natural logarithm of all values.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	///
	/// # Returns
	/// PySeriesStubbed - Series containing natural logarithm values
	fn log_bulk(&self, column: &str) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::basic_indicators::bulk::log(&values);
		let result_series = Series::new("log".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate logarithmic differences between consecutive values.
	///
	/// # Parameters
	/// - `column`: &str - Name of the column to analyze
	///
	/// # Returns
	/// PySeriesStubbed - Series containing logarithmic difference values
	fn log_difference_bulk(&self, column: &str) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{column}': {e}")))?
			.column(column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{column}' could not be converted to Series")))?
			.clone();

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::basic_indicators::bulk::log_difference(&values);
		let result_series = Series::new("log_difference".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;

	fn create_test_dataframe() -> LazyFrame {
		let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
		df! {
				"price" => data,
				"volume" => vec![100.0, 200.0, 150.0, 300.0, 250.0, 180.0, 220.0, 190.0, 280.0, 320.0]
		}
		.unwrap()
		.lazy()
	}

	fn create_basic_ti() -> BasicTI {
		let lf = create_test_dataframe();
		BasicTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_mean_single() {
		let ti = create_basic_ti();
		let result = ti.mean_single("price").unwrap();
		assert_abs_diff_eq!(result, 5.5, epsilon = 1e-10);
	}

	#[test]
	fn test_median_single() {
		let ti = create_basic_ti();
		let result = ti.median_single("price").unwrap();
		assert_abs_diff_eq!(result, 5.5, epsilon = 1e-10);
	}

	#[test]
	fn test_mode_single() {
		let ti = create_basic_ti();
		let result = ti.mode_single("price").unwrap();
		assert_abs_diff_eq!(result, 1.0, epsilon = 1e-10);
	}

	#[test]
	fn test_variance_single() {
		let ti = create_basic_ti();
		let result = ti.variance_single("price").unwrap();
		assert_abs_diff_eq!(result, 8.25, epsilon = 1e-10);
	}

	#[test]
	fn test_standard_deviation_single() {
		let ti = create_basic_ti();
		let result = ti.standard_deviation_single("price").unwrap();
		assert_abs_diff_eq!(result, 2.8722813232690143, epsilon = 1e-10);
	}

	#[test]
	fn test_max_single() {
		let ti = create_basic_ti();
		let result = ti.max_single("price").unwrap();
		assert_abs_diff_eq!(result, 10.0, epsilon = 1e-10);
	}

	#[test]
	fn test_min_single() {
		let ti = create_basic_ti();
		let result = ti.min_single("price").unwrap();
		assert_abs_diff_eq!(result, 1.0, epsilon = 1e-10);
	}

	#[test]
	fn test_absolute_deviation_single_mean() {
		let ti = create_basic_ti();
		let result = ti.absolute_deviation_single("price", "mean").unwrap();
		assert_abs_diff_eq!(result, 2.5, epsilon = 1e-10);
	}

	#[test]
	fn test_absolute_deviation_single_median() {
		let ti = create_basic_ti();
		let result = ti.absolute_deviation_single("price", "median").unwrap();
		assert_abs_diff_eq!(result, 2.5, epsilon = 1e-10);
	}

	#[test]
	fn test_log_difference_single() {
		let ti = create_basic_ti();
		let result = ti.log_difference_single(2.0, 1.0).unwrap();
		assert_abs_diff_eq!(result, (2.0_f64).ln() - (1.0_f64).ln(), epsilon = 1e-10);
	}

	#[test]
	fn test_mean_bulk() {
		let ti = create_basic_ti();
		let result = ti.mean_bulk("price", 3).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);
		assert!(values[0].is_nan());
		assert!(values[1].is_nan());
		assert_abs_diff_eq!(values[2], 2.0, epsilon = 1e-10);
		assert_abs_diff_eq!(values[3], 3.0, epsilon = 1e-10);
	}

	#[test]
	fn test_median_bulk() {
		let ti = create_basic_ti();
		let result = ti.median_bulk("price", 3).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);
		assert!(values[0].is_nan());
		assert!(values[1].is_nan());
		assert_abs_diff_eq!(values[2], 2.0, epsilon = 1e-10);
	}

	#[test]
	fn test_mode_bulk() {
		let ti = create_basic_ti();
		let result = ti.mode_bulk("price", 3).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);
		assert!(values[0].is_nan());
		assert!(values[1].is_nan());
	}

	#[test]
	fn test_variance_bulk() {
		let ti = create_basic_ti();
		let result = ti.variance_bulk("price", 3).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);
		assert!(values[0].is_nan());
		assert!(values[1].is_nan());
		assert_abs_diff_eq!(values[2], 0.6666666666666666, epsilon = 1e-10);
	}

	#[test]
	fn test_standard_deviation_bulk() {
		let ti = create_basic_ti();
		let result = ti.standard_deviation_bulk("price", 3).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);
		assert!(values[0].is_nan());
		assert!(values[1].is_nan());
		assert_abs_diff_eq!(values[2], 0.8164965809277261, epsilon = 1e-10);
	}

	#[test]
	fn test_absolute_deviation_bulk() {
		let ti = create_basic_ti();
		let result = ti.absolute_deviation_bulk("price", 3, "mean").unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);
		assert!(values[0].is_nan());
		assert!(values[1].is_nan());
		assert_abs_diff_eq!(values[2], 0.6666666666666666, epsilon = 1e-10);
	}

	#[test]
	fn test_log_bulk() {
		let ti = create_basic_ti();
		let result = ti.log_bulk("price").unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);
		assert_abs_diff_eq!(values[0], (1.0_f64).ln(), epsilon = 1e-10);
		assert_abs_diff_eq!(values[1], (2.0_f64).ln(), epsilon = 1e-10);
		assert_abs_diff_eq!(values[9], (10.0_f64).ln(), epsilon = 1e-10);
	}

	#[test]
	fn test_log_difference_bulk() {
		let ti = create_basic_ti();
		let result = ti.log_difference_bulk("price").unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);
		assert!(values[0].is_nan());
		assert_abs_diff_eq!(values[1], (2.0_f64).ln() - (1.0_f64).ln(), epsilon = 1e-10);
		assert_abs_diff_eq!(values[2], (3.0_f64).ln() - (2.0_f64).ln(), epsilon = 1e-10);
	}

	#[test]
	fn test_invalid_column_error() {
		let ti = create_basic_ti();
		let result = ti.mean_single("nonexistent_column");
		assert!(result.is_err());
	}

	#[test]
	fn test_invalid_central_point_error() {
		let ti = create_basic_ti();
		let result = ti.absolute_deviation_single("price", "invalid_central_point");
		assert!(result.is_err());
	}

	#[test]
	fn test_zero_period_bulk() {
		let ti = create_basic_ti();
		let result = ti.mean_bulk("price", 0);
		assert!(
			result.is_err() || {
				let values: Vec<f64> = result.unwrap().0.0.f64().unwrap().into_no_null_iter().collect();
				values.iter().all(|&x| x.is_nan())
			}
		);
	}

	#[test]
	fn test_large_period_bulk() {
		let ti = create_basic_ti();
		let result = ti.mean_bulk("price", 20).unwrap();
		let values: Vec<f64> = result.0.0.f64().unwrap().into_no_null_iter().collect();

		assert_eq!(values.len(), 10);
		assert!(values.iter().take(9).all(|&x| x.is_nan()));
		assert_abs_diff_eq!(values[9], 5.5, epsilon = 1e-10);
	}

	#[test]
	fn test_single_value_dataset() {
		let single_data = df! {
				"price" => vec![5.0]
		}
		.unwrap()
		.lazy();

		let ti = BasicTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(single_data)));

		assert_abs_diff_eq!(ti.mean_single("price").unwrap(), 5.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.median_single("price").unwrap(), 5.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.max_single("price").unwrap(), 5.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.min_single("price").unwrap(), 5.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.variance_single("price").unwrap(), 0.0, epsilon = 1e-10);
	}

	#[test]
	fn test_duplicate_values_dataset() {
		let duplicate_data = df! {
				"price" => vec![3.0, 3.0, 3.0, 3.0, 3.0]
		}
		.unwrap()
		.lazy();

		let ti = BasicTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(duplicate_data)));

		assert_abs_diff_eq!(ti.mean_single("price").unwrap(), 3.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.mode_single("price").unwrap(), 3.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.variance_single("price").unwrap(), 0.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.standard_deviation_single("price").unwrap(), 0.0, epsilon = 1e-10);
	}

	#[test]
	fn test_negative_values() {
		let negative_data = df! {
				"price" => vec![-5.0, -3.0, -1.0, 1.0, 3.0, 5.0]
		}
		.unwrap()
		.lazy();

		let ti = BasicTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(negative_data)));

		assert_abs_diff_eq!(ti.mean_single("price").unwrap(), 0.0, epsilon = 1e-10);
		assert_abs_diff_eq!(ti.median_single("price").unwrap(), 0.0, epsilon = 1e-10);
	}

	#[test]
	fn test_floating_point_precision() {
		let precision_data = df! {
				"price" => vec![1.000000001, 1.000000002, 1.000000003]
		}
		.unwrap()
		.lazy();

		let ti = BasicTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(precision_data)));

		let mean = ti.mean_single("price").unwrap();
		assert_abs_diff_eq!(mean, 1.000000002, epsilon = 1e-9);
	}

	#[test]
	fn test_different_columns() {
		let ti = create_basic_ti();

		let price_mean = ti.mean_single("price").unwrap();
		let volume_mean = ti.mean_single("volume").unwrap();

		assert_abs_diff_eq!(price_mean, 5.5, epsilon = 1e-10);
		assert_abs_diff_eq!(volume_mean, 219.0, epsilon = 1e-10);
	}
}
