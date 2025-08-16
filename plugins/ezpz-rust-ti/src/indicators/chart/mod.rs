use {
	crate::utils::extract_f64_values,
	ezpz_stubz::{lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

/// Chart Trends Technical Indicators - A collection of chart trend analysis functions for financial data

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct ChartTrendsTI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl ChartTrendsTI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	/// Find peaks in a price series over a given period
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `period`: usize - Period length for peak detection
	/// - `closest_neighbor`: usize - Minimum distance between peaks
	///
	/// # Returns
	/// Vec<(f64, usize)> - List of tuples containing:
	/// - `peak_value`: The price value at the peak
	/// - `peak_index`: The index position of the peak in the series
	fn peaks(&self, price_column: &str, period: usize, closest_neighbor: usize) -> PyResult<Vec<(f64, usize)>> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::chart_trends::peaks(&values, period, closest_neighbor);
		Ok(result)
	}

	/// Find valleys in a price series over a given period
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `period`: usize - Period length for valley detection
	/// - `closest_neighbor`: usize - Minimum distance between valleys
	///
	/// # Returns
	/// Vec<(f64, usize)> - List of tuples containing:
	/// - `valley_value`: The price value at the valley
	/// - `valley_index`: The index position of the valley in the series
	fn valleys(&self, price_column: &str, period: usize, closest_neighbor: usize) -> PyResult<Vec<(f64, usize)>> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::chart_trends::valleys(&values, period, closest_neighbor);
		Ok(result)
	}

	/// Calculate peak trend (linear regression on peaks)
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `period`: usize - Period length for peak detection
	///
	/// # Returns
	/// Tuple of (slope: f64, intercept: f64)
	/// - `slope`: The slope of the linear regression line through peaks
	/// - `intercept`: The y-intercept of the linear regression line
	fn peak_trend(&self, price_column: &str, period: usize) -> PyResult<(f64, f64)> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::chart_trends::peak_trend(&values, period);
		Ok(result)
	}

	/// Calculate valley trend (linear regression on valleys)
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `period`: usize - Period length for valley detection
	///
	/// # Returns
	/// Tuple of (slope: f64, intercept: f64)
	/// - `slope`: The slope of the linear regression line through valleys
	/// - `intercept`: The y-intercept of the linear regression line
	fn valley_trend(&self, price_column: &str, period: usize) -> PyResult<(f64, f64)> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::chart_trends::valley_trend(&values, period);
		Ok(result)
	}

	/// Calculate overall trend (linear regression on all prices)
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	///
	/// # Returns
	/// Tuple of (slope: f64, intercept: f64)
	/// - `slope`: The slope of the linear regression line through all price points
	/// - `intercept`: The y-intercept of the linear regression line
	fn overall_trend(&self, price_column: &str) -> PyResult<(f64, f64)> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::chart_trends::overall_trend(&values);
		Ok(result)
	}

	/// Break down trends in a price series
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `max_outliers`: usize - Maximum number of outliers allowed
	/// - `soft_r_squared_minimum`: f64 - Soft minimum threshold for R-squared value
	/// - `soft_r_squared_maximum`: f64 - Soft maximum threshold for R-squared value
	/// - `hard_r_squared_minimum`: f64 - Hard minimum threshold for R-squared value
	/// - `hard_r_squared_maximum`: f64 - Hard maximum threshold for R-squared value
	/// - `soft_standard_error_multiplier`: f64 - Soft multiplier for standard error threshold
	/// - `hard_standard_error_multiplier`: f64 - Hard multiplier for standard error threshold
	/// - `soft_reduced_chi_squared_multiplier`: f64 - Soft multiplier for reduced chi-squared threshold
	/// - `hard_reduced_chi_squared_multiplier`: f64 - Hard multiplier for reduced chi-squared threshold
	///
	/// # Returns
	/// Vec<(usize, usize, f64, f64)> - List of tuples containing:
	/// - `start_index`: Starting index of the trend segment
	/// - `end_index`: Ending index of the trend segment
	/// - `slope`: The slope of the linear regression for this trend segment
	/// - `intercept`: The y-intercept of the linear regression for this trend segment
	#[allow(clippy::too_many_arguments)]
	fn break_down_trends(
		&self,
		price_column: &str,
		max_outliers: usize,
		soft_r_squared_minimum: f64,
		soft_r_squared_maximum: f64,
		hard_r_squared_minimum: f64,
		hard_r_squared_maximum: f64,
		soft_standard_error_multiplier: f64,
		hard_standard_error_multiplier: f64,
		soft_reduced_chi_squared_multiplier: f64,
		hard_reduced_chi_squared_multiplier: f64,
	) -> PyResult<Vec<(usize, usize, f64, f64)>> {
		let series = self
			.lf
			.clone()
			.select([col(price_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{price_column}': {e}")))?
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::chart_trends::break_down_trends(
			&values,
			max_outliers,
			soft_r_squared_minimum,
			soft_r_squared_maximum,
			hard_r_squared_minimum,
			hard_r_squared_maximum,
			soft_standard_error_multiplier,
			hard_standard_error_multiplier,
			soft_reduced_chi_squared_multiplier,
			hard_reduced_chi_squared_multiplier,
		);
		Ok(result)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;

	fn create_test_dataframe() -> LazyFrame {
		let data = vec![1.0, 3.0, 2.0, 5.0, 4.0, 7.0, 6.0, 9.0, 8.0, 10.0];
		df! {
			"price" => data,
			"volume" => vec![100.0, 200.0, 150.0, 300.0, 250.0, 180.0, 220.0, 190.0, 280.0, 320.0]
		}
		.unwrap()
		.lazy()
	}

	fn create_peak_valley_dataframe() -> LazyFrame {
		let data = vec![1.0, 5.0, 2.0, 8.0, 3.0, 9.0, 4.0, 6.0, 7.0, 10.0];
		df! {
			"price" => data
		}
		.unwrap()
		.lazy()
	}

	fn create_trending_dataframe() -> LazyFrame {
		let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
		df! {
			"price" => data
		}
		.unwrap()
		.lazy()
	}

	fn create_chart_trends_ti() -> ChartTrendsTI {
		let lf = create_test_dataframe();
		ChartTrendsTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	fn create_peak_valley_ti() -> ChartTrendsTI {
		let lf = create_peak_valley_dataframe();
		ChartTrendsTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	fn create_trending_ti() -> ChartTrendsTI {
		let lf = create_trending_dataframe();
		ChartTrendsTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_peaks_basic() {
		let ti = create_peak_valley_ti();
		let result = ti.peaks("price", 2, 1).unwrap();
		assert!(!result.is_empty());
		for (value, index) in result {
			assert!(value > 0.0);
			assert!(index < 10);
		}
	}

	#[test]
	fn test_peaks_with_period() {
		let ti = create_chart_trends_ti();
		let result = ti.peaks("price", 3, 2).unwrap();
		for (value, index) in result {
			assert!((1.0..=10.0).contains(&value));
			assert!(index < 10);
		}
	}

	#[test]
	fn test_valleys_basic() {
		let ti = create_peak_valley_ti();
		let result = ti.valleys("price", 2, 1).unwrap();
		assert!(!result.is_empty());
		for (value, index) in result {
			assert!(value > 0.0);
			assert!(index < 10);
		}
	}

	#[test]
	fn test_valleys_with_period() {
		let ti = create_chart_trends_ti();
		let result = ti.valleys("price", 3, 2).unwrap();
		for (value, index) in result {
			assert!((1.0..=10.0).contains(&value));
			assert!(index < 10);
		}
	}

	#[test]
	fn test_peak_trend() {
		let ti = create_trending_ti();
		let result = ti.peak_trend("price", 2).unwrap();
		let (slope, intercept) = result;
		assert!(slope.is_finite());
		assert!(intercept.is_finite());
	}

	#[test]
	fn test_valley_trend() {
		let ti = create_trending_ti();
		let result = ti.valley_trend("price", 2).unwrap();
		let (slope, intercept) = result;
		assert!(slope.is_finite());
		assert!(intercept.is_finite());
	}

	#[test]
	fn test_overall_trend_upward() {
		let ti = create_trending_ti();
		let result = ti.overall_trend("price").unwrap();
		let (slope, intercept) = result;
		assert!(slope > 0.0);
		assert!(intercept.is_finite());
		assert_abs_diff_eq!(slope, 1.0, epsilon = 1e-10);
	}

	#[test]
	fn test_overall_trend_calculation() {
		let ti = create_chart_trends_ti();
		let result = ti.overall_trend("price").unwrap();
		let (slope, intercept) = result;
		assert!(slope.is_finite());
		assert!(intercept.is_finite());
	}

	#[test]
	fn test_break_down_trends_basic() {
		let ti = create_trending_ti();
		let result = ti.break_down_trends("price", 2, 0.5, 0.95, 0.3, 0.98, 2.0, 3.0, 2.0, 3.0).unwrap();

		assert!(!result.is_empty());
		for (start, end, slope, intercept) in result {
			assert!(start < end);
			assert!(end <= 10);
			assert!(slope.is_finite());
			assert!(intercept.is_finite());
		}
	}

	#[test]
	fn test_break_down_trends_with_outliers() {
		let ti = create_chart_trends_ti();
		let result = ti.break_down_trends("price", 3, 0.4, 0.9, 0.2, 0.95, 1.5, 2.5, 1.5, 2.5).unwrap();

		for (start, end, slope, intercept) in result {
			assert!(start < end);
			assert!(slope.is_finite());
			assert!(intercept.is_finite());
		}
	}

	#[test]
	fn test_invalid_column_name() {
		let ti = create_chart_trends_ti();
		let result = ti.peaks("nonexistent", 2, 1);
		assert!(result.is_err());
	}

	#[test]
	fn test_peak_trend_with_different_periods() {
		let ti = create_chart_trends_ti();
		let result1 = ti.peak_trend("price", 1).unwrap();
		let result2 = ti.peak_trend("price", 3).unwrap();

		assert!(result1.0.is_finite() && result1.1.is_finite());
		assert!(result2.0.is_finite() && result2.1.is_finite());
	}

	#[test]
	fn test_valley_trend_with_different_periods() {
		let ti = create_chart_trends_ti();
		let result1 = ti.valley_trend("price", 1).unwrap();
		let result2 = ti.valley_trend("price", 3).unwrap();

		assert!(result1.0.is_finite() && result1.1.is_finite());
		assert!(result2.0.is_finite() && result2.1.is_finite());
	}

	#[test]
	fn test_peaks_empty_result_handling() {
		let ti = create_chart_trends_ti();
		let result = ti.peaks("price", 10, 5).unwrap();
		// Should not panic even if no peaks found
		for (value, index) in result {
			assert!(value > 0.0);
			assert!(index < 10);
		}
	}

	#[test]
	fn test_valleys_empty_result_handling() {
		let ti = create_chart_trends_ti();
		let result = ti.valleys("price", 10, 5).unwrap();
		// Should not panic even if no valleys found
		for (value, index) in result {
			assert!(value > 0.0);
			assert!(index < 10);
		}
	}
}
