use {
	crate::utils::extract_f64_values,
	ezpz_stubz::{lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

fn parse_moving_average_type(ma_type: &str) -> PyResult<rust_ti::MovingAverageType> {
	match ma_type.to_lowercase().as_str() {
		"simple" => Ok(rust_ti::MovingAverageType::Simple),
		"exponential" => Ok(rust_ti::MovingAverageType::Exponential),
		"smoothed" => Ok(rust_ti::MovingAverageType::Smoothed),
		_ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Unsupported moving average type")),
	}
}

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct MATI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl MATI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	/// Moving Average (Single) - Calculates a single moving average value for a series of prices
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `moving_average_type`: &str - Type of moving average ("simple", "exponential", "smoothed")
	///
	/// # Returns
	/// f64 - Single moving average value
	fn moving_average_single(&self, price_column: &str, moving_average_type: &str) -> PyResult<f64> {
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
		let ma_type = parse_moving_average_type(moving_average_type)?;
		let result = rust_ti::moving_average::single::moving_average(&values, ma_type);
		Ok(result)
	}

	/// Moving Average (Bulk) - Calculates moving averages over a rolling window
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `moving_average_type`: &str - Type of moving average ("simple", "exponential", "smoothed")
	/// - `period`: usize - Period over which to calculate the moving average
	///
	/// # Returns
	/// PySeriesStubbed - Series of moving average values with name "moving_average"
	fn moving_average_bulk(&self, price_column: &str, moving_average_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
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
		let ma_type = parse_moving_average_type(moving_average_type)?;
		let result = rust_ti::moving_average::bulk::moving_average(&values, ma_type, period);
		let result_series = Series::new("moving_average".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// McGinley Dynamic (Single) - Calculates a single McGinley Dynamic value
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `previous_mcginley_dynamic`: f64 - Previous McGinley Dynamic value (use 0.0 if none)
	/// - `period`: usize - Period for calculation
	///
	/// # Returns
	/// f64 - Single McGinley Dynamic value
	fn mcginley_dynamic_single(&self, price_column: &str, previous_mcginley_dynamic: f64, period: usize) -> PyResult<f64> {
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
		// Use the last price value as the latest price
		let latest_price = values.last().ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Empty series"))?;
		let result = rust_ti::moving_average::single::mcginley_dynamic(*latest_price, previous_mcginley_dynamic, period);
		Ok(result)
	}

	/// McGinley Dynamic (Bulk) - Calculates McGinley Dynamic values over a series
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `previous_mcginley_dynamic`: f64 - Previous McGinley Dynamic value (use 0.0 if none)
	/// - `period`: usize - Period for calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series of McGinley Dynamic values with name "mcginley_dynamic"
	fn mcginley_dynamic_bulk(&self, price_column: &str, previous_mcginley_dynamic: f64, period: usize) -> PyResult<PySeriesStubbed> {
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
		let result = rust_ti::moving_average::bulk::mcginley_dynamic(&values, previous_mcginley_dynamic, period);
		let result_series = Series::new("mcginley_dynamic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Personalised Moving Average (Single) - Calculates a single personalised moving average
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `alpha_nominator`: f64 - Alpha nominator value
	/// - `alpha_denominator`: f64 - Alpha denominator value
	///
	/// # Returns
	/// f64 - Single personalised moving average value
	fn personalised_moving_average_single(&self, price_column: &str, alpha_nominator: f64, alpha_denominator: f64) -> PyResult<f64> {
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
		let ma_type = rust_ti::MovingAverageType::Personalised { alpha_num: alpha_nominator, alpha_den: alpha_denominator };
		let result = rust_ti::moving_average::single::moving_average(&values, ma_type);
		Ok(result)
	}

	/// Personalised Moving Average (Bulk) - Calculates personalised moving averages over a rolling window
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `alpha_nominator`: f64 - Alpha nominator value
	/// - `alpha_denominator`: f64 - Alpha denominator value
	/// - `period`: usize - Period over which to calculate the moving average
	///
	/// # Returns
	/// PySeriesStubbed - Series of personalised moving average values with name "personalised_moving_average"
	fn personalised_moving_average_bulk(&self, price_column: &str, alpha_nominator: f64, alpha_denominator: f64, period: usize) -> PyResult<PySeriesStubbed> {
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
		let ma_type = rust_ti::MovingAverageType::Personalised { alpha_num: alpha_nominator, alpha_den: alpha_denominator };
		let result = rust_ti::moving_average::bulk::moving_average(&values, ma_type, period);
		let result_series = Series::new("personalised_moving_average".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;
	use polars::lazy::frame::LazyFrame;

	fn create_test_dataframe() -> LazyFrame {
		let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
		df! {
			"price" => data,
		}
		.unwrap()
		.lazy()
	}

	fn create_mati() -> MATI {
		let lf = create_test_dataframe();
		MATI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_moving_average_single_simple() {
		let mati = create_mati();
		let result = mati.moving_average_single("price", "simple").unwrap();
		assert_abs_diff_eq!(result, 5.5, epsilon = 1e-10);
	}

	#[test]
	fn test_moving_average_bulk_exponential() {
		let mati = create_mati();
		let result = mati.moving_average_bulk("price", "exponential", 3).unwrap();
		let result_vec = result.0.0.f64().unwrap().into_no_null_iter().collect::<Vec<_>>();
		assert_eq!(result_vec.len(), 10);
		assert!(result_vec.iter().any(|&v| v != 0.0));
	}

	#[test]
	fn test_mcginley_dynamic_single() {
		let mati = create_mati();
		let result = mati.mcginley_dynamic_single("price", 0.0, 3).unwrap();
		assert!(result > 0.0);
	}

	#[test]
	fn test_mcginley_dynamic_bulk() {
		let mati = create_mati();
		let result = mati.mcginley_dynamic_bulk("price", 0.0, 3).unwrap();
		let result_vec = result.0.0.f64().unwrap().into_no_null_iter().collect::<Vec<_>>();
		assert_eq!(result_vec.len(), 10);
		assert!(result_vec.iter().any(|&v| v != 0.0));
	}

	#[test]
	fn test_personalised_moving_average_single() {
		let mati = create_mati();
		let result = mati.personalised_moving_average_single("price", 2.0, 3.0).unwrap();
		assert!(result > 0.0);
	}

	#[test]
	fn test_personalised_moving_average_bulk() {
		let mati = create_mati();
		let result = mati.personalised_moving_average_bulk("price", 2.0, 3.0, 3).unwrap();
		let result_vec = result.0.0.f64().unwrap().into_no_null_iter().collect::<Vec<_>>();
		assert_eq!(result_vec.len(), 10);
		assert!(result_vec.iter().any(|&v| v != 0.0));
	}
}
