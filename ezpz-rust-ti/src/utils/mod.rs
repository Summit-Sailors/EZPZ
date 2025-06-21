use {
	ezpz_stubz::{frame::PyDfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
};

pub(crate) fn parse_constant_model_type(constant_model_type: &str) -> PyResult<rust_ti::ConstantModelType<'_>> {
	match constant_model_type.to_lowercase().as_str() {
		"simple_moving_average" => Ok(rust_ti::ConstantModelType::SimpleMovingAverage),
		"smoothed_moving_average" => Ok(rust_ti::ConstantModelType::SmoothedMovingAverage),
		"exponential_moving_average" => Ok(rust_ti::ConstantModelType::ExponentialMovingAverage),
		"simple_moving_median" => Ok(rust_ti::ConstantModelType::SimpleMovingMedian),
		"simple_moving_mode" => Ok(rust_ti::ConstantModelType::SimpleMovingMode),
		_ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Unsupported constant model type: {constant_model_type}"))),
	}
}

pub(crate) fn parse_deviation_model(model_type: &str) -> PyResult<rust_ti::DeviationModel> {
	match model_type {
		"standard_deviation" => Ok(rust_ti::DeviationModel::StandardDeviation),
		"mean_absolute_deviation" => Ok(rust_ti::DeviationModel::MeanAbsoluteDeviation),
		"median_absolute_deviation" => Ok(rust_ti::DeviationModel::MedianAbsoluteDeviation),
		"mode_absolute_deviation" => Ok(rust_ti::DeviationModel::ModeAbsoluteDeviation),
		"ulcer_index" => Ok(rust_ti::DeviationModel::UlcerIndex),
		_ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Unsupported deviation model: {model_type}"))),
	}
}

// extract f64 values from PySeriesStubbed
pub(crate) fn extract_f64_values(series: PySeriesStubbed) -> PyResult<Vec<f64>> {
	let polars_series: Series = series.0.into();
	let values = polars_series
		.cast(&DataType::Float64)
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
		.f64()
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
		.into_no_null_iter()
		.collect::<Vec<f64>>();
	Ok(values)
}

pub(crate) fn parse_central_point(central_point: &str) -> PyResult<rust_ti::CentralPoint> {
	match central_point.to_lowercase().as_str() {
		"mean" => Ok(rust_ti::CentralPoint::Mean),
		"median" => Ok(rust_ti::CentralPoint::Median),
		"mode" => Ok(rust_ti::CentralPoint::Mode),
		_ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("central_point must be 'mean', 'median', or 'mode'")),
	}
}

pub(crate) fn create_result_series(name: &str, values: Vec<f64>) -> PySeriesStubbed {
	let result_series = Series::new(name.into(), values);
	PySeriesStubbed(pyo3_polars::PySeries(result_series))
}

#[inline]
pub(crate) fn unzip_triple<T: Clone>(data: Vec<(T, T, T)>) -> (Vec<T>, Vec<T>, Vec<T>) {
	let capacity = data.len();
	let mut vec1 = Vec::with_capacity(capacity);
	let mut vec2 = Vec::with_capacity(capacity);
	let mut vec3 = Vec::with_capacity(capacity);

	for (a, b, c) in data {
		vec1.push(a);
		vec2.push(b);
		vec3.push(c);
	}

	(vec1, vec2, vec3)
}

#[inline]
pub(crate) fn create_triple_df(
	lower: Vec<f64>,
	middle: Vec<f64>,
	upper: Vec<f64>,
	lower_name: &str,
	middle_name: &str,
	upper_name: &str,
) -> PyResult<PyDfStubbed> {
	let df = df! {
		lower_name => lower,
		middle_name => middle,
		upper_name => upper,
	}
	.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("DataFrame creation failed: {e}")))?;

	Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
}
