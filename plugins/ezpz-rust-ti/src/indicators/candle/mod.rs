use {
	crate::utils::{create_triple_df, extract_f64_values, parse_constant_model_type, parse_deviation_model, unzip_triple},
	ezpz_stubz::{frame::PyDfStubbed, lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

/// Candle Technical Indicators - A collection of candle analysis functions for financial data

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct CandleTI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl CandleTI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	/// Moving Constant Envelopes - Creates upper and lower bands from moving constant of price
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `constant_model_type`: &str - Type of moving average (e.g., "sma", "ema", "wma")
	/// - `difference`: f64 - Fixed difference value to create envelope bands
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_envelope`: f64 - Lower envelope band (middle - difference)
	/// - `middle_envelope`: f64 - Middle line (moving average)
	/// - `upper_envelope`: f64 - Upper envelope band (middle + difference)
	fn moving_constant_envelopes_single(&self, price_column: &str, constant_model_type: &str, difference: f64) -> PyResult<PyDfStubbed> {
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

		let values = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::candle_indicators::single::moving_constant_envelopes(&values, constant_type, difference);

		let df = df! {
			"lower_envelope" => [result.0],
			"middle_envelope" => [result.1],
			"upper_envelope" => [result.2],
		}
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

		Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
	}

	/// McGinley Dynamic Envelopes - Variation of moving constant envelopes using McGinley Dynamic
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `difference`: f64 - Fixed difference value to create envelope bands
	/// - `previous_mcginley_dynamic`: f64 - Previous McGinley Dynamic value for calculation
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_envelope`: f64 - Lower envelope band (McGinley Dynamic - difference)
	/// - `mcginley_dynamic`: f64 - McGinley Dynamic value
	/// - `upper_envelope`: f64 - Upper envelope band (McGinley Dynamic + difference)
	fn mcginley_dynamic_envelopes_single(&self, price_column: &str, difference: f64, previous_mcginley_dynamic: f64) -> PyResult<PyDfStubbed> {
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
		let result = rust_ti::candle_indicators::single::mcginley_dynamic_envelopes(&values, difference, previous_mcginley_dynamic);

		let df = df! {
			"lower_envelope" => [result.0],
			"mcginley_dynamic" => [result.1],
			"upper_envelope" => [result.2],
		}
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

		Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
	}

	/// Moving Constant Bands - Extended Bollinger Bands with configurable models
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `constant_model_type`: &str - Type of moving average for center line (e.g., "sma", "ema", "wma")
	/// - `deviation_model`: &str - Type of deviation calculation (e.g., "std", "mad")
	/// - `deviation_multiplier`: f64 - Multiplier for the deviation to create bands
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_band`: f64 - Lower band (moving average - deviation * multiplier)
	/// - `middle_band`: f64 - Middle band (moving average)
	/// - `upper_band`: f64 - Upper band (moving average + deviation * multiplier)
	fn moving_constant_bands_single(
		&self,
		price_column: &str,
		constant_model_type: &str,
		deviation_model: &str,
		deviation_multiplier: f64,
	) -> PyResult<PyDfStubbed> {
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
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;
		let result = rust_ti::candle_indicators::single::moving_constant_bands(&values, constant_type, deviation_type, deviation_multiplier);

		let df = df! {
			"lower_band" => [result.0],
			"middle_band" => [result.1],
			"upper_band" => [result.2],
		}
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

		Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
	}

	/// McGinley Dynamic Bands - Variation of moving constant bands using McGinley Dynamic
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `deviation_model`: &str - Type of deviation calculation (e.g., "std", "mad")
	/// - `deviation_multiplier`: f64 - Multiplier for the deviation to create bands
	/// - `previous_mcginley_dynamic`: f64 - Previous McGinley Dynamic value for calculation
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_band`: f64 - Lower band (McGinley Dynamic - deviation * multiplier)
	/// - `mcginley_dynamic`: f64 - McGinley Dynamic value
	/// - `upper_band`: f64 - Upper band (McGinley Dynamic + deviation * multiplier)
	fn mcginley_dynamic_bands_single(
		&self,
		price_column: &str,
		deviation_model: &str,
		deviation_multiplier: f64,
		previous_mcginley_dynamic: f64,
	) -> PyResult<PyDfStubbed> {
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
		let deviation_type = parse_deviation_model(deviation_model)?;
		let result = rust_ti::candle_indicators::single::mcginley_dynamic_bands(&values, deviation_type, deviation_multiplier, previous_mcginley_dynamic);

		let df = df! {
			"lower_band" => [result.0],
			"mcginley_dynamic" => [result.1],
			"upper_band" => [result.2],
		}
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

		Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
	}

	/// Ichimoku Cloud - Calculates support and resistance levels
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `conversion_period`: usize - Period for conversion line calculation (typically 9)
	/// - `base_period`: usize - Period for base line calculation (typically 26)
	/// - `span_b_period`: usize - Period for leading span B calculation (typically 52)
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `leading_span_a`: f64 - Leading Span A (future support/resistance)
	/// - `leading_span_b`: f64 - Leading Span B (future support/resistance)
	/// - `base_line`: f64 - Base Line (Kijun-sen)
	/// - `conversion_line`: f64 - Conversion Line (Tenkan-sen)
	/// - `lagged_price`: f64 - Lagging Span (Chikou Span)
	fn ichimoku_cloud_single(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		conversion_period: usize,
		base_period: usize,
		span_b_period: usize,
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
		let result = rust_ti::candle_indicators::single::ichimoku_cloud(&high_values, &low_values, &close_values, conversion_period, base_period, span_b_period);

		let df = df! {
			"leading_span_a" => [result.0],
			"leading_span_b" => [result.1],
			"base_line" => [result.2],
			"conversion_line" => [result.3],
			"lagged_price" => [result.4],
		}
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

		Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
	}

	/// Donchian Channels - Produces bands from period highs and lows
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `donchian_lower`: f64 - Lower channel (lowest low over period)
	/// - `donchian_middle`: f64 - Middle channel (average of upper and lower)
	/// - `donchian_upper`: f64 - Upper channel (highest high over period)
	fn donchian_channels_single(&self, high_column: &str, low_column: &str) -> PyResult<PyDfStubbed> {
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
		let result = rust_ti::candle_indicators::single::donchian_channels(&high_values, &low_values);

		let df = df! {
			"donchian_lower" => [result.0],
			"donchian_middle" => [result.1],
			"donchian_upper" => [result.2],
		}
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

		Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
	}

	/// Keltner Channel - Bands based on moving average and average true range
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `constant_model_type`: &str - Type of moving average for center line (e.g., "sma", "ema", "wma")
	/// - `atr_constant_model_type`: &str - Type of moving average for ATR calculation (e.g., "sma", "ema", "wma")
	/// - `multiplier`: f64 - Multiplier for the ATR to create channel width
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `keltner_lower`: f64 - Lower channel (moving average - ATR * multiplier)
	/// - `keltner_middle`: f64 - Middle channel (moving average)
	/// - `keltner_upper`: f64 - Upper channel (moving average + ATR * multiplier)
	fn keltner_channel_single(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		constant_model_type: &str,
		atr_constant_model_type: &str,
		multiplier: f64,
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
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let atr_constant_type = parse_constant_model_type(atr_constant_model_type)?;
		let result = rust_ti::candle_indicators::single::keltner_channel(&high_values, &low_values, &close_values, constant_type, atr_constant_type, multiplier);

		let df = df! {
			"keltner_lower" => [result.0],
			"keltner_middle" => [result.1],
			"keltner_upper" => [result.2],
		}
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

		Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
	}

	/// Supertrend - Trend indicator showing support and resistance levels
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `constant_model_type`: &str - Type of moving average for ATR calculation (e.g., "sma", "ema", "wma")
	/// - `multiplier`: f64 - Multiplier for the ATR to determine trend sensitivity
	///
	/// # Returns
	/// Series containing:
	/// - `supertrend`: f64 - Supertrend value (support/resistance level based on trend direction)
	fn supertrend_single(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		constant_model_type: &str,
		multiplier: f64,
	) -> PyResult<PySeriesStubbed> {
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
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::candle_indicators::single::supertrend(&high_values, &low_values, &close_values, constant_type, multiplier);

		let result_series = Series::new("supertrend".into(), vec![result]);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	// Bulk functions that return multiple values over time

	/// Moving Constant Envelopes (Bulk) - Returns envelopes over time periods
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `constant_model_type`: &str - Type of moving average (e.g., "sma", "ema", "wma")
	/// - `difference`: f64 - Fixed difference value to create envelope bands
	/// - `period`: usize - Rolling window period for calculations
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_envelope`: Vec<f64> - Time series of lower envelope bands
	/// - `middle_envelope`: Vec<f64> - Time series of middle lines (moving averages)
	/// - `upper_envelope`: Vec<f64> - Time series of upper envelope bands
	fn moving_constant_envelopes_bulk(&self, price_column: &str, constant_model_type: &str, difference: f64, period: usize) -> PyResult<PyDfStubbed> {
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
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let results = rust_ti::candle_indicators::bulk::moving_constant_envelopes(&values, constant_type, difference, period);

		let (lower_vals, middle_vals, upper_vals) = unzip_triple(results);
		create_triple_df(lower_vals, middle_vals, upper_vals, "lower_envelope", "middle_envelope", "upper_envelope")
	}

	//// McGinley Dynamic Envelopes (Bulk)
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `difference`: f64 - Fixed difference value to create envelope bands
	/// - `previous_mcginley_dynamic`: f64 - Initial McGinley Dynamic value for calculation
	/// - `period`: usize - Rolling window period for calculations
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_envelope`: Vec<f64> - Time series of lower envelope bands
	/// - `mcginley_dynamic`: Vec<f64> - Time series of McGinley Dynamic values
	/// - `upper_envelope`: Vec<f64> - Time series of upper envelope bands
	fn mcginley_dynamic_envelopes_bulk(&self, price_column: &str, difference: f64, previous_mcginley_dynamic: f64, period: usize) -> PyResult<PyDfStubbed> {
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
		let results = rust_ti::candle_indicators::bulk::mcginley_dynamic_envelopes(&values, difference, previous_mcginley_dynamic, period);

		let (lower_vals, middle_vals, upper_vals) = unzip_triple(results);
		create_triple_df(lower_vals, middle_vals, upper_vals, "lower_envelope", "mcginley_dynamic", "upper_envelope")
	}

	/// Moving Constant Bands (Bulk)
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `constant_model_type`: &str - Type of moving average for center line (e.g., "sma", "ema", "wma")
	/// - `deviation_model`: &str - Type of deviation calculation (e.g., "std", "mad")
	/// - `deviation_multiplier`: f64 - Multiplier for the deviation to create bands
	/// - `period`: usize - Rolling window period for calculations
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_band`: Vec<f64> - Time series of lower bands
	/// - `middle_band`: Vec<f64> - Time series of middle bands (moving averages)
	/// - `upper_band`: Vec<f64> - Time series of upper bands
	fn moving_constant_bands_bulk(
		&self,
		price_column: &str,
		constant_model_type: &str,
		deviation_model: &str,
		deviation_multiplier: f64,
		period: usize,
	) -> PyResult<PyDfStubbed> {
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
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;
		let results = rust_ti::candle_indicators::bulk::moving_constant_bands(&values, constant_type, deviation_type, deviation_multiplier, period);

		let (lower_vals, middle_vals, upper_vals) = unzip_triple(results);
		create_triple_df(lower_vals, middle_vals, upper_vals, "lower_band", "middle_band", "upper_band")
	}

	/// McGinley Dynamic Bands (Bulk)
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `deviation_model`: &str - Type of deviation calculation (e.g., "std", "mad")
	/// - `deviation_multiplier`: f64 - Multiplier for the deviation to create bands
	/// - `previous_mcginley_dynamic`: f64 - Initial McGinley Dynamic value for calculation
	/// - `period`: usize - Rolling window period for calculations
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_band`: Vec<f64> - Time series of lower bands
	/// - `mcginley_dynamic`: Vec<f64> - Time series of McGinley Dynamic values
	/// - `upper_band`: Vec<f64> - Time series of upper bands
	fn mcginley_dynamic_bands_bulk(
		&self,
		price_column: &str,
		deviation_model: &str,
		deviation_multiplier: f64,
		previous_mcginley_dynamic: f64,
		period: usize,
	) -> PyResult<PyDfStubbed> {
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
		let deviation_type = parse_deviation_model(deviation_model)?;
		let results = rust_ti::candle_indicators::bulk::mcginley_dynamic_bands(&values, deviation_type, deviation_multiplier, previous_mcginley_dynamic, period);

		let (lower_vals, middle_vals, upper_vals) = unzip_triple(results);
		create_triple_df(lower_vals, middle_vals, upper_vals, "lower_band", "mcginley_dynamic", "upper_band")
	}

	/// Ichimoku Cloud (Bulk) - Returns ichimoku components over time
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `conversion_period`: usize - Period for conversion line calculation (typically 9)
	/// - `base_period`: usize - Period for base line calculation (typically 26)
	/// - `span_b_period`: usize - Period for leading span B calculation (typically 52)
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `leading_span_a`: Vec<f64> - Time series of Leading Span A values
	/// - `leading_span_b`: Vec<f64> - Time series of Leading Span B values
	/// - `base_line`: Vec<f64> - Time series of Base Line (Kijun-sen) values
	/// - `conversion_line`: Vec<f64> - Time series of Conversion Line (Tenkan-sen) values
	/// - `lagged_price`: Vec<f64> - Time series of Lagging Span (Chikou Span) values
	fn ichimoku_cloud_bulk(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		conversion_period: usize,
		base_period: usize,
		span_b_period: usize,
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
		let ichimoku_result =
			rust_ti::candle_indicators::bulk::ichimoku_cloud(&high_values, &low_values, &close_values, conversion_period, base_period, span_b_period);

		let capacity = ichimoku_result.len();
		let mut leading_span_a = Vec::with_capacity(capacity);
		let mut leading_span_b = Vec::with_capacity(capacity);
		let mut base_line = Vec::with_capacity(capacity);
		let mut conversion_line = Vec::with_capacity(capacity);
		let mut lagged_price = Vec::with_capacity(capacity);

		for (a, b, c, d, e) in ichimoku_result {
			leading_span_a.push(a);
			leading_span_b.push(b);
			base_line.push(c);
			conversion_line.push(d);
			lagged_price.push(e);
		}

		let df = df! {
			"leading_span_a" => leading_span_a,
			"leading_span_b" => leading_span_b,
			"base_line" => base_line,
			"conversion_line" => conversion_line,
			"lagged_price" => lagged_price,
		}
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

		Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
	}

	/// Donchian Channels (Bulk) - Returns donchian bands over time
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `period`: usize - Rolling window period for channel calculation
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_band`: Vec<f64> - Time series of lower channels (lowest lows)
	/// - `middle_band`: Vec<f64> - Time series of middle channels (averages)
	/// - `upper_band`: Vec<f64> - Time series of upper channels (highest highs)
	fn donchian_channels_bulk(&self, high_column: &str, low_column: &str, period: usize) -> PyResult<PyDfStubbed> {
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

		let highs_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(high_series)))?;
		let lows_values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(low_series)))?;
		let donchian_result = rust_ti::candle_indicators::bulk::donchian_channels(&highs_values, &lows_values, period);

		let (lower_band, middle_band, upper_band) = unzip_triple(donchian_result);
		create_triple_df(lower_band, middle_band, upper_band, "lower_band", "middle_band", "upper_band")
	}

	/// Keltner Channel (Bulk) - Returns keltner bands over time
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `constant_model_type`: &str - Type of moving average for center line (e.g., "sma", "ema", "wma")
	/// - `atr_constant_model_type`: &str - Type of moving average for ATR calculation (e.g., "sma", "ema", "wma")
	/// - `multiplier`: f64 - Multiplier for the ATR to create channel width
	/// - `period`: usize - Rolling window period for calculations
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_band`: Vec<f64> - Time series of lower channels
	/// - `middle_band`: Vec<f64> - Time series of middle channels (moving averages)
	/// - `upper_band`: Vec<f64> - Time series of upper channels
	#[allow(clippy::too_many_arguments)]
	fn keltner_channel_bulk(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		constant_model_type: &str,
		atr_constant_model_type: &str,
		multiplier: f64,
		period: usize,
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
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let atr_constant_type = parse_constant_model_type(atr_constant_model_type)?;
		let keltner_result =
			rust_ti::candle_indicators::bulk::keltner_channel(&high_values, &low_values, &close_values, constant_type, atr_constant_type, multiplier, period);

		let (lower_band, middle_band, upper_band) = unzip_triple(keltner_result);
		create_triple_df(lower_band, middle_band, upper_band, "lower_band", "middle_band", "upper_band")
	}

	/// Supertrend (Bulk) - Returns supertrend values over time
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `constant_model_type`: &str - Type of moving average for ATR calculation (e.g., "sma", "ema", "wma")
	/// - `multiplier`: f64 - Multiplier for the ATR to determine trend sensitivity
	/// - `period`: usize - Rolling window period for ATR calculation
	///
	/// # Returns
	/// Series containing:
	/// - `supertrend`: Vec<f64> - Time series of supertrend values (support/resistance levels)
	fn supertrend_bulk(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		constant_model_type: &str,
		multiplier: f64,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
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
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let supertrend_result = rust_ti::candle_indicators::bulk::supertrend(&high_values, &low_values, &close_values, constant_type, multiplier, period);

		let result_series = Series::new("supertrend".into(), supertrend_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;

	fn create_test_ohlc_dataframe() -> LazyFrame {
		let high = vec![105.0, 110.0, 108.0, 112.0, 115.0, 118.0, 120.0, 122.0, 125.0, 128.0];
		let low = vec![95.0, 98.0, 96.0, 100.0, 103.0, 106.0, 108.0, 110.0, 113.0, 116.0];
		let close = vec![100.0, 105.0, 102.0, 108.0, 112.0, 115.0, 118.0, 120.0, 122.0, 125.0];
		df! {
			"high" => high,
			"low" => low,
			"close" => close.clone(),
			"price" => close
		}
		.unwrap()
		.lazy()
	}

	fn create_candle_ti() -> CandleTI {
		let lf = create_test_ohlc_dataframe();
		CandleTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_moving_constant_envelopes_single() {
		let ti = create_candle_ti();
		let result = ti.moving_constant_envelopes_single("price", "sma", 5.0).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["lower_envelope", "middle_envelope", "upper_envelope"]);
		assert_eq!(df.height(), 1);

		let lower = df.column("lower_envelope").unwrap().get(0).unwrap();
		let middle = df.column("middle_envelope").unwrap().get(0).unwrap();
		let upper = df.column("upper_envelope").unwrap().get(0).unwrap();

		if let (Ok(lower_val), Ok(middle_val), Ok(upper_val)) = (lower.try_extract::<f64>(), middle.try_extract::<f64>(), upper.try_extract::<f64>()) {
			assert_abs_diff_eq!(upper_val - middle_val, 5.0, epsilon = 1e-10);
			assert_abs_diff_eq!(middle_val - lower_val, 5.0, epsilon = 1e-10);
		}
	}

	#[test]
	fn test_mcginley_dynamic_envelopes_single() {
		let ti = create_candle_ti();
		let result = ti.mcginley_dynamic_envelopes_single("price", 3.0, 100.0).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["lower_envelope", "mcginley_dynamic", "upper_envelope"]);
		assert_eq!(df.height(), 1);

		let lower = df.column("lower_envelope").unwrap().get(0).unwrap();
		let mcginley = df.column("mcginley_dynamic").unwrap().get(0).unwrap();
		let upper = df.column("upper_envelope").unwrap().get(0).unwrap();

		if let (Ok(lower_val), Ok(mcginley_val), Ok(upper_val)) = (lower.try_extract::<f64>(), mcginley.try_extract::<f64>(), upper.try_extract::<f64>()) {
			assert_abs_diff_eq!(upper_val - mcginley_val, 3.0, epsilon = 1e-10);
			assert_abs_diff_eq!(mcginley_val - lower_val, 3.0, epsilon = 1e-10);
		}
	}

	#[test]
	fn test_moving_constant_bands_single() {
		let ti = create_candle_ti();
		let result = ti.moving_constant_bands_single("price", "sma", "std", 2.0).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["lower_band", "middle_band", "upper_band"]);
		assert_eq!(df.height(), 1);

		let lower = df.column("lower_band").unwrap().get(0).unwrap();
		let middle = df.column("middle_band").unwrap().get(0).unwrap();
		let upper = df.column("upper_band").unwrap().get(0).unwrap();

		assert!(lower.try_extract::<f64>().is_ok());
		assert!(middle.try_extract::<f64>().is_ok());
		assert!(upper.try_extract::<f64>().is_ok());
	}

	#[test]
	fn test_mcginley_dynamic_bands_single() {
		let ti = create_candle_ti();
		let result = ti.mcginley_dynamic_bands_single("price", "std", 1.5, 110.0).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["lower_band", "mcginley_dynamic", "upper_band"]);
		assert_eq!(df.height(), 1);

		let lower = df.column("lower_band").unwrap().get(0).unwrap();
		let mcginley = df.column("mcginley_dynamic").unwrap().get(0).unwrap();
		let upper = df.column("upper_band").unwrap().get(0).unwrap();

		assert!(lower.try_extract::<f64>().is_ok());
		assert!(mcginley.try_extract::<f64>().is_ok());
		assert!(upper.try_extract::<f64>().is_ok());
	}

	#[test]
	fn test_ichimoku_cloud_single() {
		let ti = create_candle_ti();
		let result = ti.ichimoku_cloud_single("high", "low", "close", 9, 26, 52).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["leading_span_a", "leading_span_b", "base_line", "conversion_line", "lagged_price"]);
		assert_eq!(df.height(), 1);

		let leading_span_a = df.column("leading_span_a").unwrap().get(0).unwrap();
		let leading_span_b = df.column("leading_span_b").unwrap().get(0).unwrap();
		let base_line = df.column("base_line").unwrap().get(0).unwrap();
		let conversion_line = df.column("conversion_line").unwrap().get(0).unwrap();
		let lagged_price = df.column("lagged_price").unwrap().get(0).unwrap();

		assert!(leading_span_a.try_extract::<f64>().is_ok());
		assert!(leading_span_b.try_extract::<f64>().is_ok());
		assert!(base_line.try_extract::<f64>().is_ok());
		assert!(conversion_line.try_extract::<f64>().is_ok());
		assert!(lagged_price.try_extract::<f64>().is_ok());
	}

	#[test]
	fn test_donchian_channels_single() {
		let ti = create_candle_ti();
		let result = ti.donchian_channels_single("high", "low").unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["donchian_lower", "donchian_middle", "donchian_upper"]);
		assert_eq!(df.height(), 1);

		let lower = df.column("donchian_lower").unwrap().get(0).unwrap();
		let middle = df.column("donchian_middle").unwrap().get(0).unwrap();
		let upper = df.column("donchian_upper").unwrap().get(0).unwrap();

		if let (Ok(lower_val), Ok(middle_val), Ok(upper_val)) = (lower.try_extract::<f64>(), middle.try_extract::<f64>(), upper.try_extract::<f64>()) {
			assert!(lower_val <= middle_val);
			assert!(middle_val <= upper_val);
		}
	}

	#[test]
	fn test_keltner_channel_single() {
		let ti = create_candle_ti();
		let result = ti.keltner_channel_single("high", "low", "close", "sma", "sma", 2.0).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["keltner_lower", "keltner_middle", "keltner_upper"]);
		assert_eq!(df.height(), 1);

		let lower = df.column("keltner_lower").unwrap().get(0).unwrap();
		let middle = df.column("keltner_middle").unwrap().get(0).unwrap();
		let upper = df.column("keltner_upper").unwrap().get(0).unwrap();

		if let (Ok(lower_val), Ok(middle_val), Ok(upper_val)) = (lower.try_extract::<f64>(), middle.try_extract::<f64>(), upper.try_extract::<f64>()) {
			assert!(lower_val <= middle_val);
			assert!(middle_val <= upper_val);
		}
	}

	#[test]
	fn test_supertrend_single() {
		let ti = create_candle_ti();
		let result = ti.supertrend_single("high", "low", "close", "sma", 3.0).unwrap();
		let series = result.0.0;

		assert_eq!(series.name(), "supertrend");
		assert_eq!(series.len(), 1);
		assert!(series.get(0).unwrap().try_extract::<f64>().is_ok());
	}

	#[test]
	fn test_moving_constant_envelopes_bulk() {
		let ti = create_candle_ti();
		let result = ti.moving_constant_envelopes_bulk("price", "sma", 5.0, 3).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["lower_envelope", "middle_envelope", "upper_envelope"]);
		assert!(df.height() > 0);

		let lower_col = df.column("lower_envelope").unwrap();
		let middle_col = df.column("middle_envelope").unwrap();
		let upper_col = df.column("upper_envelope").unwrap();

		for i in 0..df.height() {
			if let (Ok(lower_val), Ok(middle_val), Ok(upper_val)) =
				(lower_col.get(i).unwrap().try_extract::<f64>(), middle_col.get(i).unwrap().try_extract::<f64>(), upper_col.get(i).unwrap().try_extract::<f64>())
			{
				assert_abs_diff_eq!(upper_val - middle_val, 5.0, epsilon = 1e-10);
				assert_abs_diff_eq!(middle_val - lower_val, 5.0, epsilon = 1e-10);
			}
		}
	}

	#[test]
	fn test_mcginley_dynamic_envelopes_bulk() {
		let ti = create_candle_ti();
		let result = ti.mcginley_dynamic_envelopes_bulk("price", 3.0, 100.0, 5).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["lower_envelope", "mcginley_dynamic", "upper_envelope"]);
		assert!(df.height() > 0);

		let lower_col = df.column("lower_envelope").unwrap();
		let mcginley_col = df.column("mcginley_dynamic").unwrap();
		let upper_col = df.column("upper_envelope").unwrap();

		for i in 0..df.height() {
			if let (Ok(lower_val), Ok(mcginley_val), Ok(upper_val)) =
				(lower_col.get(i).unwrap().try_extract::<f64>(), mcginley_col.get(i).unwrap().try_extract::<f64>(), upper_col.get(i).unwrap().try_extract::<f64>())
			{
				assert_abs_diff_eq!(upper_val - mcginley_val, 3.0, epsilon = 1e-10);
				assert_abs_diff_eq!(mcginley_val - lower_val, 3.0, epsilon = 1e-10);
			}
		}
	}

	#[test]
	fn test_moving_constant_bands_bulk() {
		let ti = create_candle_ti();
		let result = ti.moving_constant_bands_bulk("price", "sma", "std", 2.0, 5).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["lower_band", "middle_band", "upper_band"]);
		assert!(df.height() > 0);

		let lower_col = df.column("lower_band").unwrap();
		let middle_col = df.column("middle_band").unwrap();
		let upper_col = df.column("upper_band").unwrap();

		for i in 0..df.height() {
			assert!(lower_col.get(i).unwrap().try_extract::<f64>().is_ok());
			assert!(middle_col.get(i).unwrap().try_extract::<f64>().is_ok());
			assert!(upper_col.get(i).unwrap().try_extract::<f64>().is_ok());
		}
	}

	#[test]
	fn test_mcginley_dynamic_bands_bulk() {
		let ti = create_candle_ti();
		let result = ti.mcginley_dynamic_bands_bulk("price", "std", 1.5, 110.0, 5).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["lower_band", "mcginley_dynamic", "upper_band"]);
		assert!(df.height() > 0);

		let lower_col = df.column("lower_band").unwrap();
		let mcginley_col = df.column("mcginley_dynamic").unwrap();
		let upper_col = df.column("upper_band").unwrap();

		for i in 0..df.height() {
			assert!(lower_col.get(i).unwrap().try_extract::<f64>().is_ok());
			assert!(mcginley_col.get(i).unwrap().try_extract::<f64>().is_ok());
			assert!(upper_col.get(i).unwrap().try_extract::<f64>().is_ok());
		}
	}

	#[test]
	fn test_ichimoku_cloud_bulk() {
		let ti = create_candle_ti();
		let result = ti.ichimoku_cloud_bulk("high", "low", "close", 9, 26, 52).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["leading_span_a", "leading_span_b", "base_line", "conversion_line", "lagged_price"]);
		assert!(df.height() > 0);

		for col_name in &["leading_span_a", "leading_span_b", "base_line", "conversion_line", "lagged_price"] {
			let col = df.column(col_name).unwrap();
			for i in 0..df.height() {
				assert!(col.get(i).unwrap().try_extract::<f64>().is_ok());
			}
		}
	}

	#[test]
	fn test_donchian_channels_bulk() {
		let ti = create_candle_ti();
		let result = ti.donchian_channels_bulk("high", "low", 5).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["lower_band", "middle_band", "upper_band"]);
		assert!(df.height() > 0);

		let lower_col = df.column("lower_band").unwrap();
		let middle_col = df.column("middle_band").unwrap();
		let upper_col = df.column("upper_band").unwrap();

		for i in 0..df.height() {
			if let (Ok(lower_val), Ok(middle_val), Ok(upper_val)) =
				(lower_col.get(i).unwrap().try_extract::<f64>(), middle_col.get(i).unwrap().try_extract::<f64>(), upper_col.get(i).unwrap().try_extract::<f64>())
			{
				assert!(lower_val <= middle_val);
				assert!(middle_val <= upper_val);
			}
		}
	}

	#[test]
	fn test_keltner_channel_bulk() {
		let ti = create_candle_ti();
		let result = ti.keltner_channel_bulk("high", "low", "close", "sma", "sma", 2.0, 5).unwrap();
		let df = result.0.0;

		assert_eq!(df.get_column_names(), vec!["lower_band", "middle_band", "upper_band"]);
		assert!(df.height() > 0);

		let lower_col = df.column("lower_band").unwrap();
		let middle_col = df.column("middle_band").unwrap();
		let upper_col = df.column("upper_band").unwrap();

		for i in 0..df.height() {
			if let (Ok(lower_val), Ok(middle_val), Ok(upper_val)) =
				(lower_col.get(i).unwrap().try_extract::<f64>(), middle_col.get(i).unwrap().try_extract::<f64>(), upper_col.get(i).unwrap().try_extract::<f64>())
			{
				assert!(lower_val <= middle_val);
				assert!(middle_val <= upper_val);
			}
		}
	}

	#[test]
	fn test_supertrend_bulk() {
		let ti = create_candle_ti();
		let result = ti.supertrend_bulk("high", "low", "close", "sma", 3.0, 5).unwrap();
		let series = result.0.0;

		assert_eq!(series.name(), "supertrend");
		assert!(!series.is_empty());

		for i in 0..series.len() {
			assert!(series.get(i).unwrap().try_extract::<f64>().is_ok());
		}
	}

	#[test]
	fn test_invalid_column_name() {
		let ti = create_candle_ti();
		let result = ti.moving_constant_envelopes_single("invalid_column", "sma", 5.0);
		assert!(result.is_err());
	}

	#[test]
	fn test_invalid_model_type() {
		let ti = create_candle_ti();
		let result = ti.moving_constant_envelopes_single("price", "invalid_model", 5.0);
		assert!(result.is_err());
	}

	#[test]
	fn test_zero_period_bulk() {
		let ti = create_candle_ti();
		let result = ti.moving_constant_envelopes_bulk("price", "sma", 5.0, 0);
		assert!(result.is_err());
	}

	#[test]
	fn test_envelope_difference_validation() {
		let ti = create_candle_ti();
		let result = ti.moving_constant_envelopes_single("price", "sma", 0.0).unwrap();
		let df = result.0.0;

		let lower = df.column("lower_envelope").unwrap().get(0).unwrap();
		let middle = df.column("middle_envelope").unwrap().get(0).unwrap();
		let upper = df.column("upper_envelope").unwrap().get(0).unwrap();

		if let (Ok(lower_val), Ok(middle_val), Ok(upper_val)) = (lower.try_extract::<f64>(), middle.try_extract::<f64>(), upper.try_extract::<f64>()) {
			assert_abs_diff_eq!(lower_val, middle_val, epsilon = 1e-10);
			assert_abs_diff_eq!(middle_val, upper_val, epsilon = 1e-10);
		}
	}

	#[test]
	fn test_different_ma_types() {
		let ti = create_candle_ti();

		let sma_result = ti.moving_constant_envelopes_single("price", "sma", 5.0).unwrap();
		let ema_result = ti.moving_constant_envelopes_single("price", "ema", 5.0).unwrap();
		let wma_result = ti.moving_constant_envelopes_single("price", "wma", 5.0).unwrap();

		let sma_df = sma_result.0.0;
		let ema_df = ema_result.0.0;
		let wma_df = wma_result.0.0;

		assert_eq!(sma_df.get_column_names(), vec!["lower_envelope", "middle_envelope", "upper_envelope"]);
		assert_eq!(ema_df.get_column_names(), vec!["lower_envelope", "middle_envelope", "upper_envelope"]);
		assert_eq!(wma_df.get_column_names(), vec!["lower_envelope", "middle_envelope", "upper_envelope"]);
	}
}
