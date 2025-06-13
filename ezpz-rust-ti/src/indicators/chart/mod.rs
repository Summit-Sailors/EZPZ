use {
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
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
	fn peaks(series: PySeriesStubbed, period: usize, closest_neighbor: usize) -> PyResult<Vec<(f64, usize)>> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::chart_trends::peaks(&values, &period, &closest_neighbor);
		Ok(result)
	}

	/// Find valleys in a price series over a given period
	/// Returns a list of tuples (valley_value, valley_index)
	#[staticmethod]
	fn valleys(series: PySeriesStubbed, period: usize, closest_neighbor: usize) -> PyResult<Vec<(f64, usize)>> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::chart_trends::valleys(&values, &period, &closest_neighbor);
		Ok(result)
	}

	/// Calculate peak trend (linear regression on peaks)
	/// Returns a tuple (slope, intercept)
	#[staticmethod]
	fn peak_trend(series: PySeriesStubbed, period: usize) -> PyResult<(f64, f64)> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::chart_trends::peak_trend(&values, &period);
		Ok(result)
	}

	/// Calculate valley trend (linear regression on valleys)
	/// Returns a tuple (slope, intercept)
	#[staticmethod]
	fn valley_trend(series: PySeriesStubbed, period: usize) -> PyResult<(f64, f64)> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::chart_trends::valley_trend(&values, &period);
		Ok(result)
	}

	/// Calculate overall trend (linear regression on all prices)
	/// Returns a tuple (slope, intercept)
	#[staticmethod]
	fn overall_trend(series: PySeriesStubbed) -> PyResult<(f64, f64)> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::chart_trends::overall_trend(&values);
		Ok(result)
	}

	/// Break down trends in a price series
	/// Returns a list of tuples (start_index, end_index, slope, intercept)
	#[staticmethod]
	#[allow(clippy::too_many_arguments)]
	fn break_down_trends(
		series: PySeriesStubbed,
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
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

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
