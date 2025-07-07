use {
	crate::utils::extract_f64_values,
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct MATI {
	pub series: PySeriesStubbed,
}

fn parse_moving_average_type(ma_type: &str) -> PyResult<rust_ti::MovingAverageType> {
	match ma_type.to_lowercase().as_str() {
		"simple" => Ok(rust_ti::MovingAverageType::Simple),
		"exponential" => Ok(rust_ti::MovingAverageType::Exponential),
		"smoothed" => Ok(rust_ti::MovingAverageType::Smoothed),
		_ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Unsupported moving average type")),
	}
}

#[gen_stub_pymethods]
#[pymethods]
impl MATI {
	#[new]
	fn new(series: PySeriesStubbed) -> Self {
		Self { series }
	}

	/// Moving Average (Single) - Calculates a single moving average value for a series of prices
	///
	/// # Parameters
	/// - `moving_average_type`: &str - Type of moving average ("simple", "exponential", "smoothed")
	///
	/// # Returns
	/// f64 - Single moving average value
	fn moving_average_single(&self, moving_average_type: &str) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let ma_type = parse_moving_average_type(moving_average_type)?;
		let result = rust_ti::moving_average::single::moving_average(&values, ma_type);
		Ok(result)
	}

	/// Moving Average (Bulk) - Calculates moving averages over a rolling window
	///
	/// # Parameters
	/// - `moving_average_type`: &str - Type of moving average ("simple", "exponential", "smoothed")
	/// - `period`: usize - Period over which to calculate the moving average
	///
	/// # Returns
	/// PySeriesStubbed - Series of moving average values with name "moving_average"
	fn moving_average_bulk(&self, moving_average_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let ma_type = parse_moving_average_type(moving_average_type)?;
		let result = rust_ti::moving_average::bulk::moving_average(&values, ma_type, period);
		let result_series = Series::new("moving_average".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// McGinley Dynamic (Single) - Calculates a single McGinley Dynamic value
	///
	/// # Parameters
	/// - `previous_mcginley_dynamic`: f64 - Previous McGinley Dynamic value (use 0.0 if none)
	/// - `period`: usize - Period for calculation
	///
	/// # Returns
	/// f64 - Single McGinley Dynamic value
	fn mcginley_dynamic_single(&self, previous_mcginley_dynamic: f64, period: usize) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		// Use the last price value as the latest price
		let latest_price = values.last().ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Empty series"))?;
		let result = rust_ti::moving_average::single::mcginley_dynamic(*latest_price, previous_mcginley_dynamic, period);
		Ok(result)
	}

	/// McGinley Dynamic (Bulk) - Calculates McGinley Dynamic values over a series
	///
	/// # Parameters
	/// - `previous_mcginley_dynamic`: f64 - Previous McGinley Dynamic value (use 0.0 if none)
	/// - `period`: usize - Period for calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series of McGinley Dynamic values with name "mcginley_dynamic"
	fn mcginley_dynamic_bulk(&self, previous_mcginley_dynamic: f64, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let result = rust_ti::moving_average::bulk::mcginley_dynamic(&values, previous_mcginley_dynamic, period);
		let result_series = Series::new("mcginley_dynamic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Personalised Moving Average (Single) - Calculates a single personalised moving average
	///
	/// # Parameters
	/// - `alpha_nominator`: f64 - Alpha nominator value
	/// - `alpha_denominator`: f64 - Alpha denominator value
	///
	/// # Returns
	/// f64 - Single personalised moving average value
	fn personalised_moving_average_single(&self, alpha_nominator: f64, alpha_denominator: f64) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let ma_type = rust_ti::MovingAverageType::Personalised { alpha_num: alpha_nominator, alpha_den: alpha_denominator };
		let result = rust_ti::moving_average::single::moving_average(&values, ma_type);
		Ok(result)
	}

	/// Personalised Moving Average (Bulk) - Calculates personalised moving averages over a rolling window
	///
	/// # Parameters
	/// - `alpha_nominator`: f64 - Alpha nominator value
	/// - `alpha_denominator`: f64 - Alpha denominator value
	/// - `period`: usize - Period over which to calculate the moving average
	///
	/// # Returns
	/// PySeriesStubbed - Series of personalised moving average values with name "personalised_moving_average"
	fn personalised_moving_average_bulk(&self, alpha_nominator: f64, alpha_denominator: f64, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let ma_type = rust_ti::MovingAverageType::Personalised { alpha_num: alpha_nominator, alpha_den: alpha_denominator };
		let result = rust_ti::moving_average::bulk::moving_average(&values, ma_type, period);
		let result_series = Series::new("personalised_moving_average".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}
