use {
	crate::utils::{extract_f64_values, parse_constant_model_type},
	ezpz_stubz::{lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct StrengthTI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl StrengthTI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	/// Accumulation Distribution (Single) - Shows whether the stock is being accumulated or distributed
	/// Single value calculation using the last available values
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `volume_column`: &str - Name of the volume column
	/// - `previous_ad`: Option<f64> - Previous accumulation/distribution value (defaults to 0.0)
	///
	/// # Returns
	/// f64 - Single accumulation/distribution value
	fn accumulation_distribution_single(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		volume_column: &str,
		previous_ad: Option<f64>,
	) -> PyResult<f64> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column), col(close_column), col(volume_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let high_series = df
			.column(high_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' could not be converted to Series")))?
			.clone();

		let low_series = df
			.column(low_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' could not be converted to Series")))?
			.clone();

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let volume_series = df
			.column(volume_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' could not be converted to Series")))?
			.clone();

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;
		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;
		let volume_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(volume_series)))?;

		let high = high_values.last().ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("High series is empty"))?;
		let low = low_values.last().ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Low series is empty"))?;
		let close = close_values.last().ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Close series is empty"))?;
		let volume = volume_values.last().ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Volume series is empty"))?;

		let previous = previous_ad.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::single::accumulation_distribution(*high, *low, *close, *volume, previous);
		Ok(result)
	}

	/// Accumulation Distribution (Bulk) - Shows whether the stock is being accumulated or distributed
	/// Returns a series of accumulation/distribution values
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `volume_column`: &str - Name of the volume column
	/// - `previous_ad`: Option<f64> - Previous accumulation/distribution value (defaults to 0.0)
	///
	/// # Returns
	/// PySeriesStubbed - Series containing accumulation/distribution values with name "accumulation_distribution"
	fn accumulation_distribution_bulk(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		volume_column: &str,
		previous_ad: Option<f64>,
	) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column), col(close_column), col(volume_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let high_series = df
			.column(high_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' could not be converted to Series")))?
			.clone();

		let low_series = df
			.column(low_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' could not be converted to Series")))?
			.clone();

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let volume_series = df
			.column(volume_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' could not be converted to Series")))?
			.clone();

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;
		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;
		let volume_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(volume_series)))?;

		let previous = previous_ad.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::bulk::accumulation_distribution(&high_values, &low_values, &close_values, &volume_values, previous);

		let result_series = Series::new("accumulation_distribution".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Positive Volume Index (Single) - Measures volume trend strength when volume increases
	/// Single value calculation using the last available values
	///
	/// # Parameters
	/// - `close_column`: &str - Name of the close price column
	/// - `volume_column`: &str - Name of the volume column
	/// - `previous_pvi`: Option<f64> - Previous positive volume index value (defaults to 0.0)
	///
	/// # Returns
	/// f64 - Single positive volume index value
	fn positive_volume_index_single(&self, close_column: &str, volume_column: &str, previous_pvi: Option<f64>) -> PyResult<f64> {
		let df = self
			.lf
			.clone()
			.select([col(close_column), col(volume_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let volume_series = df
			.column(volume_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' could not be converted to Series")))?
			.clone();

		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;
		let volume_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(volume_series)))?;

		// Need at least 2 values for comparison
		if close_values.len() < 2 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Need at least 2 values for volume index calculation"));
		}

		let current_close = close_values.last().unwrap();
		let previous_close = close_values[close_values.len() - 2];
		let current_volume = volume_values.last().ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Volume series is empty"))?;
		let previous_volume =
			volume_values.get(volume_values.len() - 2).ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Need at least 2 volume values"))?;

		let previous = previous_pvi.unwrap_or(0.0);

		// Calculate PVI: only update when volume increases
		let result = if current_volume > previous_volume { previous + ((*current_close - previous_close) / previous_close) * previous } else { previous };

		Ok(result)
	}

	/// Positive Volume Index (Bulk) - Measures volume trend strength when volume increases
	/// Returns a series of positive volume index values
	///
	/// # Parameters
	/// - `close_column`: &str - Name of the close price column
	/// - `volume_column`: &str - Name of the volume column
	/// - `previous_pvi`: Option<f64> - Previous positive volume index value (defaults to 0.0)
	///
	/// # Returns
	/// PySeriesStubbed - Series containing positive volume index values with name "positive_volume_index"
	fn positive_volume_index_bulk(&self, close_column: &str, volume_column: &str, previous_pvi: Option<f64>) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(close_column), col(volume_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let volume_series = df
			.column(volume_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' could not be converted to Series")))?
			.clone();

		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;
		let volume_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(volume_series)))?;

		let previous = previous_pvi.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::bulk::positive_volume_index(&close_values, &volume_values, previous);

		let result_series = Series::new("positive_volume_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Negative Volume Index (Single) - Measures volume trend strength when volume decreases
	/// Single value calculation using the last available values
	///
	/// # Parameters
	/// - `close_column`: &str - Name of the close price column
	/// - `volume_column`: &str - Name of the volume column
	/// - `previous_nvi`: Option<f64> - Previous negative volume index value (defaults to 0.0)
	///
	/// # Returns
	/// f64 - Single negative volume index value
	fn negative_volume_index_single(&self, close_column: &str, volume_column: &str, previous_nvi: Option<f64>) -> PyResult<f64> {
		let df = self
			.lf
			.clone()
			.select([col(close_column), col(volume_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let volume_series = df
			.column(volume_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' could not be converted to Series")))?
			.clone();

		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;
		let volume_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(volume_series)))?;

		// Need at least 2 values for comparison
		if close_values.len() < 2 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Need at least 2 values for volume index calculation"));
		}

		let current_close = close_values.last().unwrap();
		let previous_close = close_values[close_values.len() - 2];
		let current_volume = volume_values.last().ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Volume series is empty"))?;
		let previous_volume =
			volume_values.get(volume_values.len() - 2).ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>("Need at least 2 volume values"))?;

		let previous = previous_nvi.unwrap_or(0.0);

		// Calculate NVI: only update when volume decreases
		let result = if current_volume < previous_volume { previous + ((*current_close - previous_close) / previous_close) * previous } else { previous };

		Ok(result)
	}

	/// Negative Volume Index (Bulk) - Measures volume trend strength when volume decreases
	/// Returns a series of negative volume index values
	///
	/// # Parameters
	/// - `close_column`: &str - Name of the close price column
	/// - `volume_column`: &str - Name of the volume column
	/// - `previous_nvi`: Option<f64> - Previous negative volume index value (defaults to 0.0)
	///
	/// # Returns
	/// PySeriesStubbed - Series containing negative volume index values with name "negative_volume_index"
	fn negative_volume_index_bulk(&self, close_column: &str, volume_column: &str, previous_nvi: Option<f64>) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(close_column), col(volume_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let volume_series = df
			.column(volume_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' could not be converted to Series")))?
			.clone();

		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;
		let volume_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(volume_series)))?;

		let previous = previous_nvi.unwrap_or(0.0);
		let result = rust_ti::strength_indicators::bulk::negative_volume_index(&close_values, &volume_values, previous);

		let result_series = Series::new("negative_volume_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Relative Vigor Index (Single) - Measures the strength of an asset by looking at previous prices
	/// Single value calculation using all available values
	///
	/// # Parameters
	/// - `open_column`: &str - Name of the opening price column
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `constant_model_type`: &str - Type of constant model to use
	///
	/// # Returns
	/// f64 - Single relative vigor index value
	fn relative_vigor_index_single(
		&self,
		open_column: &str,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		constant_model_type: &str,
	) -> PyResult<f64> {
		let df = self
			.lf
			.clone()
			.select([col(open_column), col(high_column), col(low_column), col(close_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let open_series = df
			.column(open_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{open_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{open_column}' could not be converted to Series")))?
			.clone();

		let high_series = df
			.column(high_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' could not be converted to Series")))?
			.clone();

		let low_series = df
			.column(low_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' could not be converted to Series")))?
			.clone();

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let open_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(open_series)))?;
		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;
		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::strength_indicators::single::relative_vigor_index(&open_values, &high_values, &low_values, &close_values, constant_type);

		Ok(result)
	}

	/// Relative Vigor Index (Bulk) - Measures the strength of an asset by looking at previous prices
	/// Returns a series of relative vigor index values
	///
	/// # Parameters
	/// - `open_column`: &str - Name of the opening price column
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `constant_model_type`: &str - Type of constant model to use
	/// - `period`: usize - Period length for calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series containing relative vigor index values with name "relative_vigor_index"
	fn relative_vigor_index_bulk(
		&self,
		open_column: &str,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		constant_model_type: &str,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(open_column), col(high_column), col(low_column), col(close_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let open_series = df
			.column(open_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{open_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{open_column}' could not be converted to Series")))?
			.clone();

		let high_series = df
			.column(high_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' could not be converted to Series")))?
			.clone();

		let low_series = df
			.column(low_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' could not be converted to Series")))?
			.clone();

		let close_series = df
			.column(close_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{close_column}' could not be converted to Series")))?
			.clone();

		let open_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(open_series)))?;
		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;
		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::strength_indicators::bulk::relative_vigor_index(&open_values, &high_values, &low_values, &close_values, constant_type, period);

		let result_series = Series::new("relative_vigor_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;

	fn create_test_dataframe() -> LazyFrame {
		let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
		df! {
				"open" => data.clone(),
				"high" => data.iter().map(|x| x + 1.0).collect::<Vec<f64>>(),
				"low" => data.iter().map(|x| x - 0.5).collect::<Vec<f64>>(),
				"close" => data,
				"volume" => vec![100.0, 200.0, 150.0, 300.0, 250.0, 180.0, 220.0, 190.0, 280.0, 320.0]
		}
		.unwrap()
		.lazy()
	}

	fn create_strength_ti() -> StrengthTI {
		let lf = create_test_dataframe();
		StrengthTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_accumulation_distribution_single() {
		let ti = create_strength_ti();
		let result = ti.accumulation_distribution_single("high", "low", "close", "volume", None).unwrap();
		// Formula: ((close - low) - (high - close)) / (high - low) * volume + previous_ad
		// For last row: high=11.0, low=9.5, close=10.0, volume=320.0, previous_ad=0.0
		// ((10.0 - 9.5) - (11.0 - 10.0)) / (11.0 - 9.5) * 320.0 = (0.5 - 1.0) / 1.5 * 320.0 = -106.66666666666667
		assert_abs_diff_eq!(result, -106.66666666666667, epsilon = 1e-10);
	}

	#[test]
	fn test_accumulation_distribution_single_invalid_column() {
		let ti = create_strength_ti();
		let result = ti.accumulation_distribution_single("invalid", "low", "close", "volume", None);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "PyValueError: Column 'invalid' not found: ColumnNotFound");
	}

	#[test]
	fn test_accumulation_distribution_bulk() {
		let ti = create_strength_ti();
		let result = ti.accumulation_distribution_bulk("high", "low", "close", "volume", None).unwrap();
		let values: Vec<f64> = extract_f64_values(result).unwrap();

		assert_eq!(values.len(), 10);
		// First value: high=2.0, low=0.5, close=1.0, volume=100.0
		// ((1.0 - 0.5) - (2.0 - 1.0)) / (2.0 - 0.5) * 100.0 = (0.5 - 1.0) / 1.5 * 100.0 = -33.333333333333336
		assert_abs_diff_eq!(values[0], -33.333333333333336, epsilon = 1e-10);
		// Last value: high=11.0, low=9.5, close=10.0, volume=320.0
		assert_abs_diff_eq!(values[9], -106.66666666666667, epsilon = 1e-10);
	}

	#[test]
	fn test_positive_volume_index_single() {
		let ti = create_strength_ti();
		let result = ti.positive_volume_index_single("close", "volume", None).unwrap();
		// Last volume (320.0) > previous volume (280.0)
		// Formula: previous_pvi + ((current_close - previous_close) / previous_close) * previous_pvi
		// previous_pvi=0.0, so result=0.0 (since initial previous_pvi is 0)
		assert_abs_diff_eq!(result, 0.0, epsilon = 1e-10);
	}

	#[test]
	fn test_positive_volume_index_single_insufficient_data() {
		let mut ti = create_strength_ti();
		// Create a dataframe with only one row
		let lf = df! {
				"close" => vec![10.0],
				"volume" => vec![320.0]
		}
		.unwrap()
		.lazy();
		ti.lf = lf;
		let result = ti.positive_volume_index_single("close", "volume", None);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err().to_string(), "PyValueError: Need at least 2 values for volume index calculation");
	}

	#[test]
	fn test_positive_volume_index_bulk() {
		let ti = create_strength_ti();
		let result = ti.positive_volume_index_bulk("close", "volume", None).unwrap();
		let values: Vec<f64> = extract_f64_values(result).unwrap();

		assert_eq!(values.len(), 10);
		// First value is 0.0 (initial value)
		assert_abs_diff_eq!(values[0], 0.0, epsilon = 1e-10);
		// Check a case where volume increases (index 1: volume 200.0 > 100.0)
		// PVI = previous_pvi + ((current_close - previous_close) / previous_close) * previous_pvi
		// close[1]=2.0, close[0]=1.0, previous_pvi=0.0 -> 0.0 + ((2.0 - 1.0) / 1.0) * 0.0 = 0.0
		assert_abs_diff_eq!(values[1], 0.0, epsilon = 1e-10);
	}

	#[test]
	fn test_negative_volume_index_single() {
		let ti = create_strength_ti();
		let result = ti.negative_volume_index_single("close", "volume", None).unwrap();
		// Last volume (320.0) > previous volume (280.0), so NVI doesn't update
		// Result is previous_nvi (0.0)
		assert_abs_diff_eq!(result, 0.0, epsilon = 1e-10);
	}

	#[test]
	fn test_negative_volume_index_single_decreasing_volume() {
		let mut ti = create_strength_ti();
		// Modify volume so last volume < previous volume
		let lf = df! {
				"close" => vec![1.0, 2.0, 3.0],
				"volume" => vec![200.0, 150.0, 100.0]
		}
		.unwrap()
		.lazy();
		ti.lf = lf;
		let result = ti.negative_volume_index_single("close", "volume", Some(1000.0)).unwrap();
		// Volume decreases (100.0 < 150.0)
		// NVI = previous_nvi + ((current_close - previous_close) / previous_close) * previous_nvi
		// close[2]=3.0, close[1]=2.0, previous_nvi=1000.0
		// NVI = 1000.0 + ((3.0 - 2.0) / 2.0) * 1000.0 = 1000.0 + 0.5 * 1000.0 = 1500.0
		assert_abs_diff_eq!(result, 1500.0, epsilon = 1e-10);
	}

	#[test]
	fn test_negative_volume_index_bulk() {
		let ti = create_strength_ti();
		let result = ti.negative_volume_index_bulk("close", "volume", None).unwrap();
		let values: Vec<f64> = extract_f64_values(result).unwrap();

		assert_eq!(values.len(), 10);
		// First value is 0.0 (initial value)
		assert_abs_diff_eq!(values[0], 0.0, epsilon = 1e-10);
		// Check a case where volume decreases (index 2: volume 150.0 < 200.0)
		// NVI = previous_nvi + ((current_close - previous_close) / previous_close) * previous_nvi
		// close[2]=3.0, close[1]=2.0, previous_nvi=0.0 -> 0.0 + ((3.0 - 2.0) / 2.0) * 0.0 = 0.0
		assert_abs_diff_eq!(values[2], 0.0, epsilon = 1e-10);
	}

	#[test]
	fn test_relative_vigor_index_single() {
		let ti = create_strength_ti();
		let result = ti.relative_vigor_index_single("open", "high", "low", "close", "mean").unwrap();
		// RVI = ((close - open) / (high - low)) for the period, normalized by constant model (mean)
		// Using all values, calculate average (close - open) / (high - low)
		// For each row: close - open = 0.0, high - low = 1.5, so (0.0 / 1.5) = 0.0
		// Mean of these values over 10 rows = 0.0
		assert_abs_diff_eq!(result, 0.0, epsilon = 1e-10);
	}

	#[test]
	fn test_relative_vigor_index_single_invalid_constant_model() {
		let ti = create_strength_ti();
		let result = ti.relative_vigor_index_single("open", "high", "low", "close", "invalid");
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("Invalid constant model type"));
	}

	#[test]
	fn test_relative_vigor_index_bulk() {
		let ti = create_strength_ti();
		let result = ti.relative_vigor_index_bulk("open", "high", "low", "close", "mean", 3).unwrap();
		let values: Vec<f64> = extract_f64_values(result).unwrap();

		assert_eq!(values.len(), 10);
		// First two values are NaN due to period=3
		assert!(values[0].is_nan());
		assert!(values[1].is_nan());
		// For index 2: take rows 0-2
		// Each row: (close - open) / (high - low) = 0.0 / 1.5 = 0.0
		// Mean over 3 rows = 0.0
		assert_abs_diff_eq!(values[2], 0.0, epsilon = 1e-10);
	}

	#[test]
	fn test_relative_vigor_index_bulk_empty_data() {
		let mut ti = create_strength_ti();
		let lf = df! {
				"open" => Vec::<f64>::new(),
				"high" => Vec::<f64>::new(),
				"low" => Vec::<f64>::new(),
				"close" => Vec::<f64>::new()
		}
		.unwrap()
		.lazy();
		ti.lf = lf;
		let result = ti.relative_vigor_index_bulk("open", "high", "low", "close", "mean", 3);
		assert!(result.is_ok());
		let values: Vec<f64> = extract_f64_values(result.unwrap()).unwrap();
		assert_eq!(values.len(), 0);
	}
}
