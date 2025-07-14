use {
	crate::utils::{extract_f64_values, parse_constant_model_type, parse_deviation_model},
	ezpz_stubz::{lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct CorrelationTI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl CorrelationTI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	/// Correlation between two assets - Single value calculation
	/// Calculates correlation between prices of two assets using specified models
	/// Returns a single correlation value for the entire price series
	///
	/// # Parameters
	/// - `price_column_a`: &str - Name of the first asset's price column
	/// - `price_column_b`: &str - Name of the second asset's price column
	/// - `constant_model_type`: &str - Type of constant model to use for correlation calculation
	/// - `deviation_model`: &str - Type of deviation model to use for correlation calculation
	///
	/// # Returns
	/// f64 - Single correlation coefficient between the two asset price series
	fn correlate_asset_prices_single(&self, price_column_a: &str, price_column_b: &str, constant_model_type: &str, deviation_model: &str) -> PyResult<f64> {
		let df = self
			.lf
			.clone()
			.select([col(price_column_a), col(price_column_b)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let series_a = df
			.column(price_column_a)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column_a}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column_a}' could not be converted to Series")))?
			.clone();

		let series_b = df
			.column(price_column_b)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column_b}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column_b}' could not be converted to Series")))?
			.clone();

		let values_a: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series_a)))?;
		let values_b: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series_b)))?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;
		let result = rust_ti::correlation_indicators::single::correlate_asset_prices(&values_a, &values_b, constant_type, deviation_type);
		Ok(result)
	}

	/// Correlation between two assets - Rolling/Bulk calculation
	/// Calculates rolling correlation between prices of two assets using specified models
	/// Returns a series of correlation values for each period window
	///
	/// # Parameters
	/// - `price_column_a`: &str - Name of the first asset's price column
	/// - `price_column_b`: &str - Name of the second asset's price column
	/// - `constant_model_type`: &str - Type of constant model to use for correlation calculation
	/// - `deviation_model`: &str - Type of deviation model to use for correlation calculation
	/// - `period`: usize - Rolling window size for correlation calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series containing rolling correlation coefficients for each period window with name "correlation"
	fn correlate_asset_prices_bulk(
		&self,
		price_column_a: &str,
		price_column_b: &str,
		constant_model_type: &str,
		deviation_model: &str,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(price_column_a), col(price_column_b)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let series_a = df
			.column(price_column_a)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column_a}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column_a}' could not be converted to Series")))?
			.clone();

		let series_b = df
			.column(price_column_b)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column_b}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column_b}' could not be converted to Series")))?
			.clone();

		let values_a: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series_a)))?;
		let values_b: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series_b)))?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;
		let result = rust_ti::correlation_indicators::bulk::correlate_asset_prices(&values_a, &values_b, constant_type, deviation_type, period);
		let correlation_series = Series::new("correlation".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(correlation_series)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;

	fn create_test_dataframe() -> LazyFrame {
		let price_a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
		let price_b = vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0, 20.0];
		df! {
			"price_a" => price_a,
			"price_b" => price_b,
			"volume" => vec![100.0, 200.0, 150.0, 300.0, 250.0, 180.0, 220.0, 190.0, 280.0, 320.0]
		}
		.unwrap()
		.lazy()
	}

	fn create_uncorrelated_dataframe() -> LazyFrame {
		let price_a = vec![1.0, 3.0, 2.0, 5.0, 4.0, 7.0, 6.0, 9.0, 8.0, 10.0];
		let price_b = vec![10.0, 8.0, 9.0, 6.0, 7.0, 4.0, 5.0, 2.0, 3.0, 1.0];
		df! {
			"price_a" => price_a,
			"price_b" => price_b
		}
		.unwrap()
		.lazy()
	}

	fn create_negative_correlation_dataframe() -> LazyFrame {
		let price_a = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
		let price_b = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
		df! {
			"price_a" => price_a,
			"price_b" => price_b
		}
		.unwrap()
		.lazy()
	}

	fn create_correlation_ti() -> CorrelationTI {
		let lf = create_test_dataframe();
		CorrelationTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	fn create_uncorrelated_ti() -> CorrelationTI {
		let lf = create_uncorrelated_dataframe();
		CorrelationTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	fn create_negative_correlation_ti() -> CorrelationTI {
		let lf = create_negative_correlation_dataframe();
		CorrelationTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_correlate_asset_prices_single_positive() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_single("price_a", "price_b", "arithmetic", "population").unwrap();
		assert!(result > 0.9);
		assert!(result <= 1.0);
	}

	#[test]
	fn test_correlate_asset_prices_single_negative() {
		let ti = create_negative_correlation_ti();
		let result = ti.correlate_asset_prices_single("price_a", "price_b", "arithmetic", "population").unwrap();
		assert!(result < -0.9);
		assert!(result >= -1.0);
	}

	#[test]
	fn test_correlate_asset_prices_single_uncorrelated() {
		let ti = create_uncorrelated_ti();
		let result = ti.correlate_asset_prices_single("price_a", "price_b", "arithmetic", "population").unwrap();
		assert!(result.abs() < 0.5);
	}

	#[test]
	fn test_correlate_asset_prices_single_arithmetic_sample() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_single("price_a", "price_b", "arithmetic", "sample").unwrap();
		assert!(result > 0.9);
		assert!(result <= 1.0);
	}

	#[test]
	fn test_correlate_asset_prices_single_geometric_population() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_single("price_a", "price_b", "geometric", "population").unwrap();
		assert!(result.is_finite());
		assert!((-1.0..=1.0).contains(&result));
	}

	#[test]
	fn test_correlate_asset_prices_single_harmonic_sample() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_single("price_a", "price_b", "harmonic", "sample").unwrap();
		assert!(result.is_finite());
		assert!((-1.0..=1.0).contains(&result));
	}

	#[test]
	fn test_correlate_asset_prices_bulk_basic() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_bulk("price_a", "price_b", "arithmetic", "population", 3).unwrap();
		let series = result.0.0;
		assert!(!series.is_empty());
		assert_eq!(series.name(), "correlation");
	}

	#[test]
	fn test_correlate_asset_prices_bulk_window_size() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_bulk("price_a", "price_b", "arithmetic", "population", 5).unwrap();
		let series = result.0.0;
		assert!(!series.is_empty());

		if let Ok(values) = series.f64() {
			for value in values.into_iter().flatten() {
				assert!((-1.0..=1.0).contains(&value));
			}
		}
	}

	#[test]
	fn test_correlate_asset_prices_bulk_different_models() {
		let ti = create_correlation_ti();
		let result1 = ti.correlate_asset_prices_bulk("price_a", "price_b", "arithmetic", "population", 4).unwrap();
		let result2 = ti.correlate_asset_prices_bulk("price_a", "price_b", "geometric", "sample", 4).unwrap();

		let series1 = result1.0.0;
		let series2 = result2.0.0;

		assert!(!series1.is_empty());
		assert!(!series2.is_empty());
		assert_eq!(series1.name(), "correlation");
		assert_eq!(series2.name(), "correlation");
	}

	#[test]
	fn test_correlate_asset_prices_bulk_large_window() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_bulk("price_a", "price_b", "arithmetic", "population", 8).unwrap();
		let series = result.0.0;
		assert!(!series.is_empty());
	}

	#[test]
	fn test_correlate_asset_prices_bulk_small_window() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_bulk("price_a", "price_b", "arithmetic", "population", 2).unwrap();
		let series = result.0.0;
		assert!(!series.is_empty());
	}

	#[test]
	fn test_correlate_asset_prices_single_invalid_column() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_single("nonexistent", "price_b", "arithmetic", "population");
		assert!(result.is_err());
	}

	#[test]
	fn test_correlate_asset_prices_bulk_invalid_column() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_bulk("price_a", "nonexistent", "arithmetic", "population", 3);
		assert!(result.is_err());
	}

	#[test]
	fn test_correlate_asset_prices_single_invalid_constant_model() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_single("price_a", "price_b", "invalid_model", "population");
		assert!(result.is_err());
	}

	#[test]
	fn test_correlate_asset_prices_single_invalid_deviation_model() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_single("price_a", "price_b", "arithmetic", "invalid_model");
		assert!(result.is_err());
	}

	#[test]
	fn test_correlate_asset_prices_bulk_invalid_constant_model() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_bulk("price_a", "price_b", "invalid_model", "population", 3);
		assert!(result.is_err());
	}

	#[test]
	fn test_correlate_asset_prices_bulk_invalid_deviation_model() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_bulk("price_a", "price_b", "arithmetic", "invalid_model", 3);
		assert!(result.is_err());
	}

	#[test]
	fn test_correlate_asset_prices_single_same_column() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_single("price_a", "price_a", "arithmetic", "population").unwrap();
		assert_abs_diff_eq!(result, 1.0, epsilon = 1e-10);
	}

	#[test]
	fn test_correlate_asset_prices_bulk_same_column() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_bulk("price_a", "price_a", "arithmetic", "population", 3).unwrap();
		let series = result.0.0;

		if let Ok(values) = series.f64() {
			for value in values.into_iter().flatten() {
				assert_abs_diff_eq!(value, 1.0, epsilon = 1e-10);
			}
		}
	}

	#[test]
	fn test_correlate_asset_prices_single_all_model_combinations() {
		let ti = create_correlation_ti();
		let constant_models = vec!["arithmetic", "geometric", "harmonic"];
		let deviation_models = vec!["population", "sample"];

		for constant_model in &constant_models {
			for deviation_model in &deviation_models {
				let result = ti.correlate_asset_prices_single("price_a", "price_b", constant_model, deviation_model).unwrap();
				assert!(result.is_finite());
				assert!((-1.0..=1.0).contains(&result));
			}
		}
	}

	#[test]
	fn test_correlate_asset_prices_bulk_all_model_combinations() {
		let ti = create_correlation_ti();
		let constant_models = vec!["arithmetic", "geometric", "harmonic"];
		let deviation_models = vec!["population", "sample"];

		for constant_model in &constant_models {
			for deviation_model in &deviation_models {
				let result = ti.correlate_asset_prices_bulk("price_a", "price_b", constant_model, deviation_model, 3).unwrap();
				let series = result.0.0;
				assert!(!series.is_empty());
				assert_eq!(series.name(), "correlation");
			}
		}
	}

	#[test]
	fn test_correlate_asset_prices_bulk_correlation_bounds() {
		let ti = create_correlation_ti();
		let result = ti.correlate_asset_prices_bulk("price_a", "price_b", "arithmetic", "population", 4).unwrap();
		let series = result.0.0;
		if let Ok(values) = series.f64() {
			for value in values.into_iter().flatten() {
				assert!((-1.0..=1.0).contains(&value), "Correlation value {value} is out of bounds");
			}
		}
	}
}
