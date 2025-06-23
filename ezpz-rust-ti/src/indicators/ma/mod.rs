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
pub struct MATI;

fn parse_moving_average_type(ma_type: &str) -> PyResult<rust_ti::MovingAverageType<'_>> {
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
	/// Moving Average (Single) - Calculates a single moving average value for a series of prices
	///
	/// # Arguments
	/// * `prices` - Series of price values
	/// * `moving_average_type` - Type of moving average ("simple", "exponential", "smoothed")
	///
	/// # Returns
	/// Single moving average value as a Series
	#[staticmethod]
	fn moving_average_single(prices: PySeriesStubbed, moving_average_type: &str) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let ma_type = parse_moving_average_type(moving_average_type)?;
		let result = rust_ti::moving_average::single::moving_average(&values, &ma_type);

		let result_series = Series::new("moving_average".into(), vec![result]);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Moving Average (Bulk) - Calculates moving averages over a rolling window
	///
	/// # Arguments
	/// * `prices` - Series of price values
	/// * `moving_average_type` - Type of moving average ("simple", "exponential", "smoothed")
	/// * `period` - Period over which to calculate the moving average
	///
	/// # Returns
	/// Series of moving average values
	#[staticmethod]
	fn moving_average_bulk(prices: PySeriesStubbed, moving_average_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let ma_type = parse_moving_average_type(moving_average_type)?;
		let result = rust_ti::moving_average::bulk::moving_average(&values, &ma_type, &period);

		let result_series = Series::new("moving_average".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// McGinley Dynamic (Single) - Calculates a single McGinley Dynamic value
	///
	/// # Arguments
	/// * `latest_price` - Latest price value
	/// * `previous_mcginley_dynamic` - Previous McGinley Dynamic value (use 0.0 if none)
	/// * `period` - Period for calculation
	///
	/// # Returns
	/// Single McGinley Dynamic value as a Series
	#[staticmethod]
	fn mcginley_dynamic_single(latest_price: f64, previous_mcginley_dynamic: f64, period: usize) -> PyResult<PySeriesStubbed> {
		let result = rust_ti::moving_average::single::mcginley_dynamic(&latest_price, &previous_mcginley_dynamic, &period);

		let result_series = Series::new("mcginley_dynamic".into(), vec![result]);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// McGinley Dynamic (Bulk) - Calculates McGinley Dynamic values over a series
	///
	/// # Arguments
	/// * `prices` - Series of price values
	/// * `previous_mcginley_dynamic` - Previous McGinley Dynamic value (use 0.0 if none)
	/// * `period` - Period for calculation
	///
	/// # Returns
	/// Series of McGinley Dynamic values
	#[staticmethod]
	fn mcginley_dynamic_bulk(prices: PySeriesStubbed, previous_mcginley_dynamic: f64, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result = rust_ti::moving_average::bulk::mcginley_dynamic(&values, &previous_mcginley_dynamic, &period);

		let result_series = Series::new("mcginley_dynamic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Personalised Moving Average (Single) - Calculates a single personalised moving average
	///
	/// # Arguments
	/// * `prices` - Series of price values
	/// * `alpha_nominator` - Alpha nominator value
	/// * `alpha_denominator` - Alpha denominator value
	///
	/// # Returns
	/// Single personalised moving average value as a Series
	#[staticmethod]
	fn personalised_moving_average_single(prices: PySeriesStubbed, alpha_nominator: f64, alpha_denominator: f64) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let ma_type = rust_ti::MovingAverageType::Personalised(&alpha_nominator, &alpha_denominator);
		let result = rust_ti::moving_average::single::moving_average(&values, &ma_type);

		let result_series = Series::new("personalised_moving_average".into(), vec![result]);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Personalised Moving Average (Bulk) - Calculates personalised moving averages over a rolling window
	///
	/// # Arguments
	/// * `prices` - Series of price values
	/// * `alpha_nominator` - Alpha nominator value
	/// * `alpha_denominator` - Alpha denominator value
	/// * `period` - Period over which to calculate the moving average
	///
	/// # Returns
	/// Series of personalised moving average values
	#[staticmethod]
	fn personalised_moving_average_bulk(prices: PySeriesStubbed, alpha_nominator: f64, alpha_denominator: f64, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let ma_type = rust_ti::MovingAverageType::Personalised(&alpha_nominator, &alpha_denominator);
		let result = rust_ti::moving_average::bulk::moving_average(&values, &ma_type, &period);

		let result_series = Series::new("personalised_moving_average".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}
