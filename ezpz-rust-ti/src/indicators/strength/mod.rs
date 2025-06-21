use {
	crate::utils::{extract_f64_values, parse_constant_model_type},
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct StrengthTI;

#[gen_stub_pymethods]
#[pymethods]
impl StrengthTI {
	/// Accumulation Distribution - Shows whether the stock is being accumulated or distributed
	///
	/// # Parameters
	/// - `high`: PySeriesStubbed - Series of high prices
	/// - `low`: PySeriesStubbed - Series of low prices
	/// - `close`: PySeriesStubbed - Series of closing prices
	/// - `volume`: PySeriesStubbed - Series of trading volumes
	/// - `previous_ad`: Option<f64> - Previous accumulation/distribution value (defaults to 0.0)
	///
	/// # Returns
	/// PySeriesStubbed - Series containing accumulation/distribution values
	#[staticmethod]
	fn accumulation_distribution(
		high: PySeriesStubbed,
		low: PySeriesStubbed,
		close: PySeriesStubbed,
		volume: PySeriesStubbed,
		previous_ad: Option<f64>,
	) -> PyResult<PySeriesStubbed> {
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let volume_values: Vec<f64> = extract_f64_values(volume)?;

		let previous = previous_ad.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::bulk::accumulation_distribution(&high_values, &low_values, &close_values, &volume_values, &previous);

		let result_series = Series::new("accumulation_distribution".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Positive Volume Index - Measures volume trend strength when volume increases
	///
	/// # Parameters
	/// - `close`: PySeriesStubbed - Series of closing prices
	/// - `volume`: PySeriesStubbed - Series of trading volumes
	/// - `previous_pvi`: Option<f64> - Previous positive volume index value (defaults to 0.0)
	///
	/// # Returns
	/// PySeriesStubbed - Series containing positive volume index values
	#[staticmethod]
	fn positive_volume_index(close: PySeriesStubbed, volume: PySeriesStubbed, previous_pvi: Option<f64>) -> PyResult<PySeriesStubbed> {
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let volume_values: Vec<f64> = extract_f64_values(volume)?;

		let previous = previous_pvi.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::bulk::positive_volume_index(&close_values, &volume_values, &previous);

		let result_series = Series::new("positive_volume_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Negative Volume Index - Measures volume trend strength when volume decreases
	///
	/// # Parameters
	/// - `close`: PySeriesStubbed - Series of closing prices
	/// - `volume`: PySeriesStubbed - Series of trading volumes
	/// - `previous_nvi`: Option<f64> - Previous negative volume index value (defaults to 0.0)
	///
	/// # Returns
	/// PySeriesStubbed - Series containing negative volume index values
	#[staticmethod]
	fn negative_volume_index(close: PySeriesStubbed, volume: PySeriesStubbed, previous_nvi: Option<f64>) -> PyResult<PySeriesStubbed> {
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let volume_values: Vec<f64> = extract_f64_values(volume)?;

		let previous = previous_nvi.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::bulk::negative_volume_index(&close_values, &volume_values, &previous);

		let result_series = Series::new("negative_volume_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Relative Vigor Index - Measures the strength of an asset by looking at previous prices
	///
	/// # Parameters
	/// - `open`: PySeriesStubbed - Series of opening prices
	/// - `high`: PySeriesStubbed - Series of high prices
	/// - `low`: PySeriesStubbed - Series of low prices
	/// - `close`: PySeriesStubbed - Series of closing prices
	/// - `constant_model_type`: &str - Type of constant model to use
	/// - `period`: usize - Period length for calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series containing relative vigor index values
	#[staticmethod]
	fn relative_vigor_index(
		open: PySeriesStubbed,
		high: PySeriesStubbed,
		low: PySeriesStubbed,
		close: PySeriesStubbed,
		constant_model_type: &str,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let open_values: Vec<f64> = extract_f64_values(open)?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::strength_indicators::bulk::relative_vigor_index(&open_values, &high_values, &low_values, &close_values, &constant_type, &period);

		let result_series = Series::new("relative_vigor_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Single Accumulation Distribution - Single value calculation
	///
	/// # Parameters
	/// - `high`: f64 - High price for the period
	/// - `low`: f64 - Low price for the period
	/// - `close`: f64 - Closing price for the period
	/// - `volume`: f64 - Trading volume for the period
	/// - `previous_ad`: Option<f64> - Previous accumulation/distribution value (defaults to 0.0)
	///
	/// # Returns
	/// f64 - Single accumulation/distribution value
	#[staticmethod]
	fn single_accumulation_distribution(high: f64, low: f64, close: f64, volume: f64, previous_ad: Option<f64>) -> PyResult<f64> {
		let previous = previous_ad.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::single::accumulation_distribution(&high, &low, &close, &volume, &previous);
		Ok(result)
	}

	/// Single Volume Index - Generic version of PVI and NVI for single calculation
	///
	/// # Parameters
	/// - `current_close`: f64 - Current period closing price
	/// - `previous_close`: f64 - Previous period closing price
	/// - `previous_volume_index`: Option<f64> - Previous volume index value (defaults to 0.0)
	///
	/// # Returns
	/// f64 - Single volume index value
	#[staticmethod]
	fn single_volume_index(current_close: f64, previous_close: f64, previous_volume_index: Option<f64>) -> PyResult<f64> {
		let previous = previous_volume_index.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::single::volume_index(&current_close, &previous_close, &previous);
		Ok(result)
	}

	/// Single Relative Vigor Index - Single value calculation
	///
	/// # Parameters
	/// - `open`: PySeriesStubbed - Series of opening prices
	/// - `high`: PySeriesStubbed - Series of high prices
	/// - `low`: PySeriesStubbed - Series of low prices
	/// - `close`: PySeriesStubbed - Series of closing prices
	/// - `constant_model_type`: &str - Type of constant model to use
	///
	/// # Returns
	/// f64 - Single relative vigor index value
	#[staticmethod]
	fn single_relative_vigor_index(
		open: PySeriesStubbed,
		high: PySeriesStubbed,
		low: PySeriesStubbed,
		close: PySeriesStubbed,
		constant_model_type: &str,
	) -> PyResult<f64> {
		let open_values: Vec<f64> = extract_f64_values(open)?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::strength_indicators::single::relative_vigor_index(&open_values, &high_values, &low_values, &close_values, &constant_type);

		Ok(result)
	}
}
