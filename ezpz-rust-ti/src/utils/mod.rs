use pyo3::prelude::*;
pub(crate) fn parse_constant_model_type(constant_model_type: &str) -> PyResult<rust_ti::ConstantModelType> {
	match constant_model_type.to_lowercase().as_str() {
		"simplemovingaverage" => Ok(rust_ti::ConstantModelType::SimpleMovingAverage),
		"smoothedmovingaverage" => Ok(rust_ti::ConstantModelType::SmoothedMovingAverage),
		"exponentialmovingaverage" => Ok(rust_ti::ConstantModelType::ExponentialMovingAverage),
		"simplemovingmedian" => Ok(rust_ti::ConstantModelType::SimpleMovingMedian),
		"simplemovingmode" => Ok(rust_ti::ConstantModelType::SimpleMovingMode),
		_ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Unsupported constant model type: {constant_model_type}"))),
	}
}

pub(crate) fn parse_deviation_model(model_type: &str) -> PyResult<rust_ti::DeviationModel> {
	match model_type {
		"StandardDeviation" => Ok(rust_ti::DeviationModel::StandardDeviation),
		"MeanAbsoluteDeviation" => Ok(rust_ti::DeviationModel::MeanAbsoluteDeviation),
		"MedianAbsoluteDeviation" => Ok(rust_ti::DeviationModel::MedianAbsoluteDeviation),
		"ModeAbsoluteDeviation" => Ok(rust_ti::DeviationModel::ModeAbsoluteDeviation),
		"UlcerIndex" => Ok(rust_ti::DeviationModel::UlcerIndex),
		_ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Unsupported deviation model: {model_type}"))),
	}
}
