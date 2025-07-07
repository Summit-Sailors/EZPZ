use {
	crate::utils::extract_f64_values,
	ezpz_stubz::series::PySeriesStubbed,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct ChartTrendsTI {
	pub series: PySeriesStubbed,
}

#[gen_stub_pymethods]
#[pymethods]
impl ChartTrendsTI {
	#[new]
	fn new(series: PySeriesStubbed) -> Self {
		Self { series }
	}

	/// Find peaks in a price series over a given period
	///
	/// # Parameters
	/// - `period`: usize - Period length for peak detection
	/// - `closest_neighbor`: usize - Minimum distance between peaks
	///
	/// # Returns
	/// Vec<(f64, usize)> - List of tuples containing:
	/// - `peak_value`: The price value at the peak
	/// - `peak_index`: The index position of the peak in the series
	fn peaks(&self, period: usize, closest_neighbor: usize) -> PyResult<Vec<(f64, usize)>> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let result = rust_ti::chart_trends::peaks(&values, period, closest_neighbor);
		Ok(result)
	}

	/// Find valleys in a price series over a given period
	///
	/// # Parameters
	/// - `period`: usize - Period length for valley detection
	/// - `closest_neighbor`: usize - Minimum distance between valleys
	///
	/// # Returns
	/// Vec<(f64, usize)> - List of tuples containing:
	/// - `valley_value`: The price value at the valley
	/// - `valley_index`: The index position of the valley in the series
	fn valleys(&self, period: usize, closest_neighbor: usize) -> PyResult<Vec<(f64, usize)>> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let result = rust_ti::chart_trends::valleys(&values, period, closest_neighbor);
		Ok(result)
	}

	/// Calculate peak trend (linear regression on peaks)
	///
	/// # Parameters
	/// - `period`: usize - Period length for peak detection
	///
	/// # Returns
	/// Tuple of (slope: f64, intercept: f64)
	/// - `slope`: The slope of the linear regression line through peaks
	/// - `intercept`: The y-intercept of the linear regression line
	fn peak_trend(&self, period: usize) -> PyResult<(f64, f64)> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let result = rust_ti::chart_trends::peak_trend(&values, period);
		Ok(result)
	}

	/// Calculate valley trend (linear regression on valleys)
	///
	/// # Parameters
	/// - `period`: usize - Period length for valley detection
	///
	/// # Returns
	/// Tuple of (slope: f64, intercept: f64)
	/// - `slope`: The slope of the linear regression line through valleys
	/// - `intercept`: The y-intercept of the linear regression line
	fn valley_trend(&self, period: usize) -> PyResult<(f64, f64)> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let result = rust_ti::chart_trends::valley_trend(&values, period);
		Ok(result)
	}

	/// Calculate overall trend (linear regression on all prices)
	///
	/// # Returns
	/// Tuple of (slope: f64, intercept: f64)
	/// - `slope`: The slope of the linear regression line through all price points
	/// - `intercept`: The y-intercept of the linear regression line
	fn overall_trend(&self) -> PyResult<(f64, f64)> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let result = rust_ti::chart_trends::overall_trend(&values);
		Ok(result)
	}

	/// Break down trends in a price series
	///
	/// # Parameters
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
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
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
