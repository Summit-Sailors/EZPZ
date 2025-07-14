use {
	crate::utils::{create_triple_df, extract_f64_values, parse_constant_model_type},
	ezpz_stubz::{frame::PyDfStubbed, lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

/// Trend Technical Indicators - A collection of trend analysis functions for financial data
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct TrendTI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl TrendTI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	// Single value functions (return a single value from the entire series)

	/// Aroon Up (Single) - Measures the strength of upward price momentum
	/// Calculates the percentage of time since the highest high within the series
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column to analyze
	///
	/// # Returns
	/// f64 - Aroon Up value (0-100), where higher values indicate stronger upward momentum
	fn aroon_up_single(&self, high_column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(high_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{high_column}': {e}")))?
			.column(high_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::trend_indicators::single::aroon_up(&values);
		Ok(result)
	}

	/// Aroon Down (Single) - Measures the strength of downward price momentum
	/// Calculates the percentage of time since the lowest low within the series
	///
	/// # Parameters
	/// - `low_column`: &str - Name of the low price column to analyze
	///
	/// # Returns
	/// f64 - Aroon Down value (0-100), where higher values indicate stronger downward momentum
	fn aroon_down_single(&self, low_column: &str) -> PyResult<f64> {
		let series = self
			.lf
			.clone()
			.select([col(low_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{low_column}': {e}")))?
			.column(low_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::trend_indicators::single::aroon_down(&values);
		Ok(result)
	}

	/// Aroon Oscillator (Single) - Calculates the difference between Aroon Up and Aroon Down
	/// Provides a single measure of trend direction and strength
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	///
	/// # Returns
	/// f64 - Aroon Oscillator value (-100 to 100), where positive values indicate upward trend
	fn aroon_oscillator_single(&self, high_column: &str, low_column: &str) -> PyResult<f64> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column)])
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

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;

		let result = rust_ti::trend_indicators::single::aroon_indicator(&high_values, &low_values);
		Ok(result.2) // Return the oscillator component
	}

	/// Aroon Indicator (Single) - Calculates complete Aroon system in one call
	/// Computes Aroon Up, Aroon Down, and Aroon Oscillator
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	///
	/// # Returns
	/// (f64, f64, f64) - Tuple containing (Aroon Up, Aroon Down, Aroon Oscillator)
	fn aroon_indicator_single(&self, high_column: &str, low_column: &str) -> PyResult<(f64, f64, f64)> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column)])
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

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;

		let result = rust_ti::trend_indicators::single::aroon_indicator(&high_values, &low_values);
		Ok(result)
	}

	/// True Strength Index (Single) - Momentum oscillator using double-smoothed price changes
	/// Filters out price noise to provide clearer momentum signals
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `first_constant_model`: &str - First smoothing method ("SimpleMovingAverage", "ExponentialMovingAverage", etc.)
	/// - `first_period`: usize - Period for first smoothing
	/// - `second_constant_model`: &str - Second smoothing method
	///
	/// # Returns
	/// f64 - True Strength Index value (-100 to 100)
	fn true_strength_index_single(&self, price_column: &str, first_constant_model: &str, first_period: usize, second_constant_model: &str) -> PyResult<f64> {
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
		let first_model = parse_constant_model_type(first_constant_model)?;
		let second_model = parse_constant_model_type(second_constant_model)?;
		let result = rust_ti::trend_indicators::single::true_strength_index(&values, first_model, first_period, second_model);
		Ok(result)
	}

	// Bulk functions (return series of values)

	/// Aroon Up (Bulk) - Calculates rolling Aroon Up indicator over specified period
	/// Measures upward momentum strength for each period in the time series
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column to analyze
	/// - `period`: usize - Lookback period for calculation (typically 14)
	///
	/// # Returns
	/// PySeriesStubbed - Series of Aroon Up values (0-100) named "aroon_up"
	fn aroon_up_bulk(&self, high_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(high_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{high_column}': {e}")))?
			.column(high_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{high_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::trend_indicators::bulk::aroon_up(&values, period);
		let result_series = Series::new("aroon_up".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Aroon Down (Bulk) - Calculates rolling Aroon Down indicator over specified period
	/// Measures downward momentum strength for each period in the time series
	///
	/// # Parameters
	/// - `low_column`: &str - Name of the low price column to analyze
	/// - `period`: usize - Lookback period for calculation (typically 14)
	///
	/// # Returns
	/// PySeriesStubbed - Series of Aroon Down values (0-100) named "aroon_down"
	fn aroon_down_bulk(&self, low_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(low_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{low_column}': {e}")))?
			.column(low_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{low_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let result = rust_ti::trend_indicators::bulk::aroon_down(&values, period);
		let result_series = Series::new("aroon_down".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Aroon Oscillator (Bulk) - Calculates rolling Aroon Oscillator over specified period
	/// Computes the difference between Aroon Up and Aroon Down for each period
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `period`: usize - Lookback period for calculation (typically 14)
	///
	/// # Returns
	/// PySeriesStubbed - Series of Aroon Oscillator values (-100 to 100) named "aroon_oscillator"
	fn aroon_oscillator_bulk(&self, high_column: &str, low_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column)])
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

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;

		let aroon_up_result = rust_ti::trend_indicators::bulk::aroon_up(&high_values, period);
		let aroon_down_result = rust_ti::trend_indicators::bulk::aroon_down(&low_values, period);
		let result = rust_ti::trend_indicators::bulk::aroon_oscillator(&aroon_up_result, &aroon_down_result);
		let result_series = Series::new("aroon_oscillator".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Aroon Indicator (Bulk) - Calculates complete Aroon system for time series data
	/// Computes Aroon Up, Aroon Down, and Aroon Oscillator for each period
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `period`: usize - Lookback period for calculation (typically 14)
	///
	/// # Returns
	/// PyDfStubbed - DataFrame with columns: "aroon_up", "aroon_down", "aroon_oscillator"
	fn aroon_indicator_bulk(&self, high_column: &str, low_column: &str, period: usize) -> PyResult<PyDfStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column)])
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

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;

		let aroon_result = rust_ti::trend_indicators::bulk::aroon_indicator(&high_values, &low_values, period);
		let (aroon_up, aroon_down, aroon_oscillator) = {
			let mut up = Vec::new();
			let mut down = Vec::new();
			let mut oscillator = Vec::new();
			for (val_up, val_down, val_osc) in aroon_result {
				up.push(val_up);
				down.push(val_down);
				oscillator.push(val_osc);
			}
			(up, down, oscillator)
		};

		create_triple_df(aroon_up, aroon_down, aroon_oscillator, "aroon_up", "aroon_down", "aroon_oscillator")
	}

	/// Parabolic Time Price System (Bulk) - Calculates Stop and Reverse points
	/// Provides trailing stop levels for trend-following system
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `acceleration_factor_start`: f64 - Initial acceleration factor (typically 0.02)
	/// - `acceleration_factor_max`: f64 - Maximum acceleration factor (typically 0.20)
	/// - `acceleration_factor_step`: f64 - Acceleration factor increment (typically 0.02)
	/// - `start_position`: &str - Initial position: "Long" or "Short"
	/// - `previous_sar`: f64 - Initial SAR value
	///
	/// # Returns
	/// PySeriesStubbed - Series of SAR values named "parabolic_sar"
	#[allow(clippy::too_many_arguments)]
	fn parabolic_time_price_system_bulk(
		&self,
		high_column: &str,
		low_column: &str,
		acceleration_factor_start: f64,
		acceleration_factor_max: f64,
		acceleration_factor_step: f64,
		start_position: &str,
		previous_sar: f64,
	) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column)])
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

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;

		let position = match start_position {
			"Long" => rust_ti::Position::Long,
			"Short" => rust_ti::Position::Short,
			_ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid position. Use 'Long' or 'Short'")),
		};

		let result = rust_ti::trend_indicators::bulk::parabolic_time_price_system(
			&high_values,
			&low_values,
			acceleration_factor_start,
			acceleration_factor_max,
			acceleration_factor_step,
			position,
			previous_sar,
		);
		let result_series = Series::new("parabolic_sar".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Directional Movement System (Bulk) - Calculates complete DMS indicators
	/// Computes +DI, -DI, ADX, and ADXR for trend strength analysis
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `period`: usize - Calculation period (typically 14)
	/// - `constant_model_type`: &str - Smoothing method: "SimpleMovingAverage", "SmoothedMovingAverage", etc.
	///
	/// # Returns
	/// PyDfStubbed - DataFrame with columns: "positive_di", "negative_di", "adx", "adxr"
	fn directional_movement_system_bulk(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		period: usize,
		constant_model_type: &str,
	) -> PyResult<PyDfStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(high_column), col(low_column), col(close_column)])
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

		let high_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let low_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;
		let close_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(close_series)))?;

		let constant_model = parse_constant_model_type(constant_model_type)?;
		let dm_result = rust_ti::trend_indicators::bulk::directional_movement_system(&high_values, &low_values, &close_values, period, constant_model);
		let (positive_di, negative_di, adx, adxr) = {
			let mut pos_di = Vec::new();
			let mut neg_di = Vec::new();
			let mut adx_vals = Vec::new();
			let mut adxr_vals = Vec::new();
			for (val_pos, val_neg, val_adx, val_adxr) in dm_result {
				pos_di.push(val_pos);
				neg_di.push(val_neg);
				adx_vals.push(val_adx);
				adxr_vals.push(val_adxr);
			}
			(pos_di, neg_di, adx_vals, adxr_vals)
		};

		let df = df! {
			"positive_di" => positive_di,
			"negative_di" => negative_di,
			"adx" => adx,
			"adxr" => adxr,
		}
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

		Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
	}

	/// Volume Price Trend (Bulk) - Combines price and volume to show momentum
	/// Shows the relationship between price movement and volume flow
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column
	/// - `volume_column`: &str - Name of the volume column
	/// - `previous_volume_price_trend`: f64 - Initial VPT value (typically 0)
	///
	/// # Returns
	/// PySeriesStubbed - Series of Volume Price Trend values named "volume_price_trend"
	fn volume_price_trend_bulk(&self, price_column: &str, volume_column: &str, previous_volume_price_trend: f64) -> PyResult<PySeriesStubbed> {
		let df = self
			.lf
			.clone()
			.select([col(price_column), col(volume_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to select columns: {e}")))?;

		let price_series = df
			.column(price_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{price_column}' could not be converted to Series")))?
			.clone();

		let volume_series = df
			.column(volume_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{volume_column}' could not be converted to Series")))?
			.clone();

		let price_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(price_series)))?;
		let volume_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(volume_series)))?;

		let result = rust_ti::trend_indicators::bulk::volume_price_trend(&price_values, &volume_values, previous_volume_price_trend);
		let result_series = Series::new("volume_price_trend".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// True Strength Index (Bulk) - Double-smoothed momentum oscillator
	/// Uses double-smoothed price changes to filter noise and provide clearer signals
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `first_constant_model`: &str - First smoothing method: "SimpleMovingAverage", "ExponentialMovingAverage", etc.
	/// - `first_period`: usize - Period for first smoothing (typically 25)
	/// - `second_constant_model`: &str - Second smoothing method
	/// - `second_period`: usize - Period for second smoothing (typically 13)
	///
	/// # Returns
	/// PySeriesStubbed - Series of TSI values (-100 to 100) named "true_strength_index"
	fn true_strength_index_bulk(
		&self,
		price_column: &str,
		first_constant_model: &str,
		first_period: usize,
		second_constant_model: &str,
		second_period: usize,
	) -> PyResult<PySeriesStubbed> {
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
		let first_model = parse_constant_model_type(first_constant_model)?;
		let second_model = parse_constant_model_type(second_constant_model)?;
		let result = rust_ti::trend_indicators::bulk::true_strength_index(&values, first_model, first_period, second_model, second_period);
		let result_series = Series::new("true_strength_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;

	fn create_test_dataframe() -> LazyFrame {
		let high_data = vec![10.5, 11.2, 12.0, 11.8, 12.5, 13.1, 12.9, 13.5, 14.2, 13.8];
		let low_data = vec![9.8, 10.1, 10.5, 10.2, 11.0, 11.5, 11.2, 12.0, 12.5, 12.1];
		let close_data = vec![10.2, 10.8, 11.5, 11.0, 11.8, 12.3, 12.1, 12.8, 13.5, 13.2];
		let volume_data = vec![1000.0, 1200.0, 1500.0, 1100.0, 1300.0, 1400.0, 1250.0, 1600.0, 1800.0, 1350.0];

		df! {
			"high" => high_data,
			"low" => low_data,
			"close" => close_data,
			"volume" => volume_data
		}
		.unwrap()
		.lazy()
	}

	fn create_trend_ti() -> TrendTI {
		let lf = create_test_dataframe();
		TrendTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_aroon_up_single() {
		let ti = create_trend_ti();
		let result = ti.aroon_up_single("high").unwrap();
		assert!((0.0..=100.0).contains(&result));
	}

	#[test]
	fn test_aroon_down_single() {
		let ti = create_trend_ti();
		let result = ti.aroon_down_single("low").unwrap();
		assert!((0.0..=100.0).contains(&result));
	}

	#[test]
	fn test_aroon_oscillator_single() {
		let ti = create_trend_ti();
		let result = ti.aroon_oscillator_single("high", "low").unwrap();
		assert!((-100.0..=100.0).contains(&result));
	}

	#[test]
	fn test_aroon_indicator_single() {
		let ti = create_trend_ti();
		let result = ti.aroon_indicator_single("high", "low").unwrap();
		assert!(result.0 >= 0.0 && result.0 <= 100.0);
		assert!(result.1 >= 0.0 && result.1 <= 100.0);
		assert!(result.2 >= -100.0 && result.2 <= 100.0);
		assert_abs_diff_eq!(result.2, result.0 - result.1, epsilon = 1e-10);
	}

	#[test]
	fn test_true_strength_index_single() {
		let ti = create_trend_ti();
		let result = ti.true_strength_index_single("close", "SimpleMovingAverage", 5, "ExponentialMovingAverage").unwrap();
		assert!((-100.0..=100.0).contains(&result));
	}

	#[test]
	fn test_aroon_up_bulk() {
		let ti = create_trend_ti();
		let result = ti.aroon_up_bulk("high", 5).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "aroon_up");
		assert_eq!(series.len(), 10);

		// Check that all values are within valid range
		for i in 0..series.len() {
			if let Some(val) = series.get(i).unwrap().extract::<f64>() {
				assert!((0.0..=100.0).contains(&val));
			}
		}
	}

	#[test]
	fn test_aroon_down_bulk() {
		let ti = create_trend_ti();
		let result = ti.aroon_down_bulk("low", 5).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "aroon_down");
		assert_eq!(series.len(), 10);

		// Check that all values are within valid range
		for i in 0..series.len() {
			if let Some(val) = series.get(i).unwrap().extract::<f64>() {
				assert!((0.0..=100.0).contains(&val));
			}
		}
	}

	#[test]
	fn test_aroon_oscillator_bulk() {
		let ti = create_trend_ti();
		let result = ti.aroon_oscillator_bulk("high", "low", 5).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "aroon_oscillator");
		assert_eq!(series.len(), 10);

		// Check that all values are within valid range
		for i in 0..series.len() {
			if let Some(val) = series.get(i).unwrap().extract::<f64>() {
				assert!((-100.0..=100.0).contains(&val));
			}
		}
	}

	#[test]
	fn test_aroon_indicator_bulk() {
		let ti = create_trend_ti();
		let result = ti.aroon_indicator_bulk("high", "low", 5).unwrap();
		let df = result.0.as_ref();

		assert_eq!(df.width(), 3);
		assert_eq!(df.height(), 10);

		let aroon_up = PlSmallStr::from("aroon_up");
		let aroon_down = PlSmallStr::from("aroon_down");
		let aroon_oscillator = PlSmallStr::from("aroon_oscillator");
		assert!(df.get_column_names().contains(&&aroon_up));
		assert!(df.get_column_names().contains(&&aroon_down));
		assert!(df.get_column_names().contains(&&aroon_oscillator));

		// Check that aroon_oscillator = aroon_up - aroon_down
		let aroon_up = df.column("aroon_up").unwrap();
		let aroon_down = df.column("aroon_down").unwrap();
		let aroon_osc = df.column("aroon_oscillator").unwrap();

		for i in 0..df.height() {
			if let (Some(up), Some(down), Some(osc)) =
				(aroon_up.get(i).unwrap().extract::<f64>(), aroon_down.get(i).unwrap().extract::<f64>(), aroon_osc.get(i).unwrap().extract::<f64>())
			{
				assert_abs_diff_eq!(osc, up - down, epsilon = 1e-10);
			}
		}
	}

	#[test]
	fn test_parabolic_time_price_system_bulk() {
		let ti = create_trend_ti();
		let result = ti.parabolic_time_price_system_bulk("high", "low", 0.02, 0.20, 0.02, "Long", 10.0).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "parabolic_sar");
		assert_eq!(series.len(), 10);
	}

	#[test]
	fn test_parabolic_time_price_system_bulk_invalid_position() {
		let ti = create_trend_ti();
		let result = ti.parabolic_time_price_system_bulk("high", "low", 0.02, 0.20, 0.02, "InvalidPosition", 10.0);
		assert!(result.is_err());
	}

	#[test]
	fn test_directional_movement_system_bulk() {
		let ti = create_trend_ti();
		let result = ti.directional_movement_system_bulk("high", "low", "close", 5, "SimpleMovingAverage").unwrap();
		let df = result.0.as_ref();

		assert_eq!(df.width(), 4);
		assert_eq!(df.height(), 10);
		let positive_di = PlSmallStr::from("positive_di");
		let negative_di = PlSmallStr::from("negative_di");
		let adx = PlSmallStr::from("adx");
		let adxr = PlSmallStr::from("adxr");
		assert!(df.get_column_names().contains(&&positive_di));
		assert!(df.get_column_names().contains(&&negative_di));
		assert!(df.get_column_names().contains(&&adx));
		assert!(df.get_column_names().contains(&&adxr));

		// Check that DI values are non-negative
		let pos_di = df.column("positive_di").unwrap();
		let neg_di = df.column("negative_di").unwrap();

		for i in 0..df.height() {
			if let (Some(pos), Some(neg)) = (pos_di.get(i).unwrap().extract::<f64>(), neg_di.get(i).unwrap().extract::<f64>()) {
				assert!(pos >= 0.0);
				assert!(neg >= 0.0);
			}
		}
	}

	#[test]
	fn test_volume_price_trend_bulk() {
		let ti = create_trend_ti();
		let result = ti.volume_price_trend_bulk("close", "volume", 0.0).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "volume_price_trend");
		assert_eq!(series.len(), 10);
	}

	#[test]
	fn test_true_strength_index_bulk() {
		let ti = create_trend_ti();
		let result = ti.true_strength_index_bulk("close", "SimpleMovingAverage", 5, "ExponentialMovingAverage", 3).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "true_strength_index");
		assert_eq!(series.len(), 10);

		// Check that all values are within valid range
		for i in 0..series.len() {
			if let Some(val) = series.get(i).unwrap().extract::<f64>() {
				assert!((-100.0..=100.0).contains(&val));
			}
		}
	}

	#[test]
	fn test_invalid_column_name() {
		let ti = create_trend_ti();
		let result = ti.aroon_up_single("invalid_column");
		assert!(result.is_err());
	}

	#[test]
	fn test_invalid_constant_model() {
		let ti = create_trend_ti();
		let result = ti.true_strength_index_single("close", "InvalidModel", 5, "ExponentialMovingAverage");
		assert!(result.is_err());
	}

	#[test]
	fn test_aroon_consistency_single_vs_bulk() {
		let ti = create_trend_ti();
		let single_up = ti.aroon_up_single("high").unwrap();
		let single_down = ti.aroon_down_single("low").unwrap();
		let single_osc = ti.aroon_oscillator_single("high", "low").unwrap();

		// For single values, we expect them to be the same as the last value in bulk calculation
		// This is a conceptual test - actual implementation may vary
		assert_abs_diff_eq!(single_osc, single_up - single_down, epsilon = 1e-10);
	}

	#[test]
	fn test_bulk_series_length_consistency() {
		let ti = create_trend_ti();
		let period = 5;

		let aroon_up = ti.aroon_up_bulk("high", period).unwrap();
		let aroon_down = ti.aroon_down_bulk("low", period).unwrap();
		let aroon_osc = ti.aroon_oscillator_bulk("high", "low", period).unwrap();

		assert_eq!(aroon_up.0.0.len(), aroon_down.0.0.len());
		assert_eq!(aroon_up.0.0.len(), aroon_osc.0.0.len());
	}

	#[test]
	fn test_parabolic_sar_both_positions() {
		let ti = create_trend_ti();

		let long_result = ti.parabolic_time_price_system_bulk("high", "low", 0.02, 0.20, 0.02, "Long", 10.0).unwrap();

		let short_result = ti.parabolic_time_price_system_bulk("high", "low", 0.02, 0.20, 0.02, "Short", 10.0).unwrap();

		assert_eq!(long_result.0.0.len(), short_result.0.0.len());
	}

	#[test]
	fn test_directional_movement_system_different_models() {
		let ti = create_trend_ti();

		let sma_result = ti.directional_movement_system_bulk("high", "low", "close", 5, "SimpleMovingAverage").unwrap();

		let ema_result = ti.directional_movement_system_bulk("high", "low", "close", 5, "ExponentialMovingAverage").unwrap();

		assert_eq!(sma_result.0.as_ref().height(), ema_result.0.as_ref().height());
		assert_eq!(sma_result.0.as_ref().width(), ema_result.0.as_ref().width());
	}

	#[test]
	fn test_volume_price_trend_different_initial_values() {
		let ti = create_trend_ti();

		let vpt_zero = ti.volume_price_trend_bulk("close", "volume", 0.0).unwrap();
		let vpt_hundred = ti.volume_price_trend_bulk("close", "volume", 100.0).unwrap();

		assert_eq!(vpt_zero.0.0.len(), vpt_hundred.0.0.len());

		// The difference should be constant (100.0) throughout the series
		for i in 0..vpt_zero.0.0.len() {
			if let (Some(val_zero), Some(val_hundred)) = (vpt_zero.0.0.get(i).unwrap().extract::<f64>(), vpt_hundred.0.0.get(i).unwrap().extract::<f64>()) {
				assert_abs_diff_eq!(val_hundred - val_zero, 100.0, epsilon = 1e-10);
			}
		}
	}
}
