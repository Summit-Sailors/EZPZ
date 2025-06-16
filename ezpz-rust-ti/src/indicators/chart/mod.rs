use {
	crate::utils::extract_f64_values,
	ezpz_stubz::series::PySeriesStubbed,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct ChartTrendsTI;

#[gen_stub_pymethods]
#[pymethods]
impl ChartTrendsTI {
	/// Find peaks in a price series over a given period
	/// Returns a list of tuples (peak_value, peak_index)
	#[staticmethod]
	fn peaks(prices: PySeriesStubbed, period: usize, closest_neighbor: usize) -> PyResult<Vec<(f64, usize)>> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result = rust_ti::chart_trends::peaks(&values, &period, &closest_neighbor);
		Ok(result)
	}

	/// Find valleys in a price series over a given period
	/// Returns a list of tuples (valley_value, valley_index)
	#[staticmethod]
	fn valleys(prices: PySeriesStubbed, period: usize, closest_neighbor: usize) -> PyResult<Vec<(f64, usize)>> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result = rust_ti::chart_trends::valleys(&values, &period, &closest_neighbor);
		Ok(result)
	}

	/// Calculate peak trend (linear regression on peaks)
	/// Returns a tuple (slope, intercept)
	#[staticmethod]
	fn peak_trend(prices: PySeriesStubbed, period: usize) -> PyResult<(f64, f64)> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result = rust_ti::chart_trends::peak_trend(&values, &period);
		Ok(result)
	}

	/// Calculate valley trend (linear regression on valleys)
	/// Returns a tuple (slope, intercept)
	#[staticmethod]
	fn valley_trend(prices: PySeriesStubbed, period: usize) -> PyResult<(f64, f64)> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result = rust_ti::chart_trends::valley_trend(&values, &period);
		Ok(result)
	}

	/// Calculate overall trend (linear regression on all prices)
	/// Returns a tuple (slope, intercept)
	#[staticmethod]
	fn overall_trend(prices: PySeriesStubbed) -> PyResult<(f64, f64)> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result = rust_ti::chart_trends::overall_trend(&values);
		Ok(result)
	}

	/// Break down trends in a price series
	/// Returns a list of tuples (start_index, end_index, slope, intercept)
	#[staticmethod]
	#[allow(clippy::too_many_arguments)]
	fn break_down_trends(
		prices: PySeriesStubbed,
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
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result = rust_ti::chart_trends::break_down_trends(
			&values,
			&max_outliers,
			&soft_r_squared_minimum,
			&soft_r_squared_maximum,
			&hard_r_squared_minimum,
			&hard_r_squared_maximum,
			&soft_standard_error_multiplier,
			&hard_standard_error_multiplier,
			&soft_reduced_chi_squared_multiplier,
			&hard_reduced_chi_squared_multiplier,
		);
		Ok(result)
	}
}
