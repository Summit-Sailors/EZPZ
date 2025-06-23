use {
	crate::utils::{create_triple_df, extract_f64_values, parse_constant_model_type, parse_deviation_model, unzip_triple},
	ezpz_stubz::{frame::PyDfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct CandleTI;

#[gen_stub_pymethods]
#[pymethods]
impl CandleTI {
	/// Moving Constant Envelopes - Creates upper and lower bands from moving constant of price
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of price values
	/// - `constant_model_type`: &str - Type of moving average (e.g., "sma", "ema", "wma")
	/// - `difference`: f64 - Fixed difference value to create envelope bands
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_envelope`: f64 - Lower envelope band (middle - difference)
	/// - `middle_envelope`: f64 - Middle line (moving average)
	/// - `upper_envelope`: f64 - Upper envelope band (middle + difference)
	#[staticmethod]
	fn moving_constant_envelopes_single(prices: PySeriesStubbed, constant_model_type: &str, difference: f64) -> PyResult<PyDfStubbed> {
		let values = extract_f64_values(prices)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::candle_indicators::single::moving_constant_envelopes(&values, &constant_type, &difference);

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
	/// - `prices`: PySeriesStubbed - Series of price values
	/// - `difference`: f64 - Fixed difference value to create envelope bands
	/// - `previous_mcginley_dynamic`: f64 - Previous McGinley Dynamic value for calculation
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_envelope`: f64 - Lower envelope band (McGinley Dynamic - difference)
	/// - `mcginley_dynamic`: f64 - McGinley Dynamic value
	/// - `upper_envelope`: f64 - Upper envelope band (McGinley Dynamic + difference)
	#[staticmethod]
	fn mcginley_dynamic_envelopes_single(prices: PySeriesStubbed, difference: f64, previous_mcginley_dynamic: f64) -> PyResult<PyDfStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;
		let result = rust_ti::candle_indicators::single::mcginley_dynamic_envelopes(&values, &difference, &previous_mcginley_dynamic);

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
	/// - `prices`: PySeriesStubbed - Series of price values
	/// - `constant_model_type`: &str - Type of moving average for center line (e.g., "sma", "ema", "wma")
	/// - `deviation_model`: &str - Type of deviation calculation (e.g., "std", "mad")
	/// - `deviation_multiplier`: f64 - Multiplier for the deviation to create bands
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_band`: f64 - Lower band (moving average - deviation * multiplier)
	/// - `middle_band`: f64 - Middle band (moving average)
	/// - `upper_band`: f64 - Upper band (moving average + deviation * multiplier)
	#[staticmethod]
	fn moving_constant_bands_single(
		prices: PySeriesStubbed,
		constant_model_type: &str,
		deviation_model: &str,
		deviation_multiplier: f64,
	) -> PyResult<PyDfStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;
		let result = rust_ti::candle_indicators::single::moving_constant_bands(&values, &constant_type, &deviation_type, &deviation_multiplier);

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
	/// - `prices`: PySeriesStubbed - Series of price values
	/// - `deviation_model`: &str - Type of deviation calculation (e.g., "std", "mad")
	/// - `deviation_multiplier`: f64 - Multiplier for the deviation to create bands
	/// - `previous_mcginley_dynamic`: f64 - Previous McGinley Dynamic value for calculation
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_band`: f64 - Lower band (McGinley Dynamic - deviation * multiplier)
	/// - `mcginley_dynamic`: f64 - McGinley Dynamic value
	/// - `upper_band`: f64 - Upper band (McGinley Dynamic + deviation * multiplier)
	#[staticmethod]
	fn mcginley_dynamic_bands_single(
		prices: PySeriesStubbed,
		deviation_model: &str,
		deviation_multiplier: f64,
		previous_mcginley_dynamic: f64,
	) -> PyResult<PyDfStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;
		let deviation_type = parse_deviation_model(deviation_model)?;
		let result = rust_ti::candle_indicators::single::mcginley_dynamic_bands(&values, &deviation_type, &deviation_multiplier, &previous_mcginley_dynamic);

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
	/// - `highs`: PySeriesStubbed - Series of high prices
	/// - `lows`: PySeriesStubbed - Series of low prices
	/// - `close`: PySeriesStubbed - Series of closing prices
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
	#[staticmethod]
	fn ichimoku_cloud_single(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		close: PySeriesStubbed,
		conversion_period: usize,
		base_period: usize,
		span_b_period: usize,
	) -> PyResult<PyDfStubbed> {
		let high_values: Vec<f64> = extract_f64_values(highs)?;
		let low_values: Vec<f64> = extract_f64_values(lows)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let result = rust_ti::candle_indicators::single::ichimoku_cloud(&high_values, &low_values, &close_values, &conversion_period, &base_period, &span_b_period);

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
	/// - `highs`: PySeriesStubbed - Series of high prices
	/// - `lows`: PySeriesStubbed - Series of low prices
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `donchian_lower`: f64 - Lower channel (lowest low over period)
	/// - `donchian_middle`: f64 - Middle channel (average of upper and lower)
	/// - `donchian_upper`: f64 - Upper channel (highest high over period)
	#[staticmethod]
	fn donchian_channels_single(highs: PySeriesStubbed, lows: PySeriesStubbed) -> PyResult<PyDfStubbed> {
		let high_values: Vec<f64> = extract_f64_values(highs)?;
		let low_values: Vec<f64> = extract_f64_values(lows)?;
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
	/// - `highs`: PySeriesStubbed - Series of high prices
	/// - `lows`: PySeriesStubbed - Series of low prices
	/// - `close`: PySeriesStubbed - Series of closing prices
	/// - `constant_model_type`: &str - Type of moving average for center line (e.g., "sma", "ema", "wma")
	/// - `atr_constant_model_type`: &str - Type of moving average for ATR calculation (e.g., "sma", "ema", "wma")
	/// - `multiplier`: f64 - Multiplier for the ATR to create channel width
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `keltner_lower`: f64 - Lower channel (moving average - ATR * multiplier)
	/// - `keltner_middle`: f64 - Middle channel (moving average)
	/// - `keltner_upper`: f64 - Upper channel (moving average + ATR * multiplier)
	#[staticmethod]
	fn keltner_channel_single(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		close: PySeriesStubbed,
		constant_model_type: &str,
		atr_constant_model_type: &str,
		multiplier: f64,
	) -> PyResult<PyDfStubbed> {
		let high_values: Vec<f64> = extract_f64_values(highs)?;
		let low_values: Vec<f64> = extract_f64_values(lows)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let atr_constant_type = parse_constant_model_type(atr_constant_model_type)?;
		let result = rust_ti::candle_indicators::single::keltner_channel(&high_values, &low_values, &close_values, &constant_type, &atr_constant_type, &multiplier);

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
	/// - `highs`: PySeriesStubbed - Series of high prices
	/// - `lows`: PySeriesStubbed - Series of low prices
	/// - `close`: PySeriesStubbed - Series of closing prices
	/// - `constant_model_type`: &str - Type of moving average for ATR calculation (e.g., "sma", "ema", "wma")
	/// - `multiplier`: f64 - Multiplier for the ATR to determine trend sensitivity
	///
	/// # Returns
	/// Series containing:
	/// - `supertrend`: f64 - Supertrend value (support/resistance level based on trend direction)
	#[staticmethod]
	fn supertrend_single(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		close: PySeriesStubbed,
		constant_model_type: &str,
		multiplier: f64,
	) -> PyResult<PySeriesStubbed> {
		let high_values: Vec<f64> = extract_f64_values(highs)?;
		let low_values: Vec<f64> = extract_f64_values(lows)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::candle_indicators::single::supertrend(&high_values, &low_values, &close_values, &constant_type, &multiplier);

		let result_series = Series::new("supertrend".into(), vec![result]);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	// Bulk functions that return multiple values over time

	/// Moving Constant Envelopes (Bulk) - Returns envelopes over time periods
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of price values
	/// - `constant_model_type`: &str - Type of moving average (e.g., "sma", "ema", "wma")
	/// - `difference`: f64 - Fixed difference value to create envelope bands
	/// - `period`: usize - Rolling window period for calculations
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_envelope`: Vec<f64> - Time series of lower envelope bands
	/// - `middle_envelope`: Vec<f64> - Time series of middle lines (moving averages)
	/// - `upper_envelope`: Vec<f64> - Time series of upper envelope bands
	#[staticmethod]
	fn moving_constant_envelopes_bulk(prices: PySeriesStubbed, constant_model_type: &str, difference: f64, period: usize) -> PyResult<PyDfStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let results = rust_ti::candle_indicators::bulk::moving_constant_envelopes(&values, &constant_type, &difference, &period);

		let (lower_vals, middle_vals, upper_vals) = unzip_triple(results);
		create_triple_df(lower_vals, middle_vals, upper_vals, "lower_envelope", "middle_envelope", "upper_envelope")
	}

	/// McGinley Dynamic Envelopes (Bulk)
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of price values
	/// - `difference`: f64 - Fixed difference value to create envelope bands
	/// - `previous_mcginley_dynamic`: f64 - Initial McGinley Dynamic value for calculation
	/// - `period`: usize - Rolling window period for calculations
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_envelope`: Vec<f64> - Time series of lower envelope bands
	/// - `mcginley_dynamic`: Vec<f64> - Time series of McGinley Dynamic values
	/// - `upper_envelope`: Vec<f64> - Time series of upper envelope bands
	#[staticmethod]
	fn mcginley_dynamic_envelopes_bulk(prices: PySeriesStubbed, difference: f64, previous_mcginley_dynamic: f64, period: usize) -> PyResult<PyDfStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;
		let results = rust_ti::candle_indicators::bulk::mcginley_dynamic_envelopes(&values, &difference, &previous_mcginley_dynamic, &period);

		let (lower_vals, middle_vals, upper_vals) = unzip_triple(results);
		create_triple_df(lower_vals, middle_vals, upper_vals, "lower_envelope", "mcginley_dynamic", "upper_envelope")
	}

	/// Moving Constant Bands (Bulk)
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of price values
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
	#[staticmethod]
	fn moving_constant_bands_bulk(
		prices: PySeriesStubbed,
		constant_model_type: &str,
		deviation_model: &str,
		deviation_multiplier: f64,
		period: usize,
	) -> PyResult<PyDfStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;
		let results = rust_ti::candle_indicators::bulk::moving_constant_bands(&values, &constant_type, &deviation_type, &deviation_multiplier, &period);

		let (lower_vals, middle_vals, upper_vals) = unzip_triple(results);
		create_triple_df(lower_vals, middle_vals, upper_vals, "lower_band", "middle_band", "upper_band")
	}

	/// McGinley Dynamic Bands (Bulk)
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of price values
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
	#[staticmethod]
	fn mcginley_dynamic_bands_bulk(
		prices: PySeriesStubbed,
		deviation_model: &str,
		deviation_multiplier: f64,
		previous_mcginley_dynamic: f64,
		period: usize,
	) -> PyResult<PyDfStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;
		let deviation_type = parse_deviation_model(deviation_model)?;
		let results =
			rust_ti::candle_indicators::bulk::mcginley_dynamic_bands(&values, &deviation_type, &deviation_multiplier, &previous_mcginley_dynamic, &period);

		let (lower_vals, middle_vals, upper_vals) = unzip_triple(results);
		create_triple_df(lower_vals, middle_vals, upper_vals, "lower_band", "mcginley_dynamic", "upper_band")
	}

	/// Ichimoku Cloud (Bulk) - Returns ichimoku components over time
	///
	/// # Parameters
	/// - `highs`: PySeriesStubbed - Series of high prices
	/// - `lows`: PySeriesStubbed - Series of low prices
	/// - `closes`: PySeriesStubbed - Series of closing prices
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
	#[staticmethod]
	fn ichimoku_cloud_bulk(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		closes: PySeriesStubbed,
		conversion_period: usize,
		base_period: usize,
		span_b_period: usize,
	) -> PyResult<PyDfStubbed> {
		let high_values: Vec<f64> = extract_f64_values(highs)?;
		let low_values: Vec<f64> = extract_f64_values(lows)?;
		let close_values: Vec<f64> = extract_f64_values(closes)?;
		let ichimoku_result =
			rust_ti::candle_indicators::bulk::ichimoku_cloud(&high_values, &low_values, &close_values, &conversion_period, &base_period, &span_b_period);

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
	/// - `highs`: PySeriesStubbed - Series of high prices
	/// - `lows`: PySeriesStubbed - Series of low prices
	/// - `period`: usize - Rolling window period for channel calculation
	///
	/// # Returns
	/// DataFrame with columns:
	/// - `lower_band`: Vec<f64> - Time series of lower channels (lowest lows)
	/// - `middle_band`: Vec<f64> - Time series of middle channels (averages)
	/// - `upper_band`: Vec<f64> - Time series of upper channels (highest highs)
	#[staticmethod]
	fn donchian_channels_bulk(highs: PySeriesStubbed, lows: PySeriesStubbed, period: usize) -> PyResult<PyDfStubbed> {
		let highs_values: Vec<f64> = extract_f64_values(highs)?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;
		let donchian_result = rust_ti::candle_indicators::bulk::donchian_channels(&highs_values, &lows_values, &period);

		let (lower_band, middle_band, upper_band) = unzip_triple(donchian_result);
		create_triple_df(lower_band, middle_band, upper_band, "lower_band", "middle_band", "upper_band")
	}

	/// Keltner Channel (Bulk) - Returns keltner bands over time
	///
	/// # Parameters
	/// - `highs`: PySeriesStubbed - Series of high prices
	/// - `lows`: PySeriesStubbed - Series of low prices
	/// - `closes`: PySeriesStubbed - Series of closing prices
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
	#[staticmethod]
	fn keltner_channel_bulk(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		closes: PySeriesStubbed,
		constant_model_type: &str,
		atr_constant_model_type: &str,
		multiplier: f64,
		period: usize,
	) -> PyResult<PyDfStubbed> {
		let high_values: Vec<f64> = extract_f64_values(highs)?;
		let low_values: Vec<f64> = extract_f64_values(lows)?;
		let close_values: Vec<f64> = extract_f64_values(closes)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let atr_constant_type = parse_constant_model_type(atr_constant_model_type)?;
		let keltner_result =
			rust_ti::candle_indicators::bulk::keltner_channel(&high_values, &low_values, &close_values, &constant_type, &atr_constant_type, &multiplier, &period);

		let (lower_band, middle_band, upper_band) = unzip_triple(keltner_result);
		create_triple_df(lower_band, middle_band, upper_band, "lower_band", "middle_band", "upper_band")
	}

	/// Supertrend (Bulk) - Returns supertrend values over time
	///
	/// # Parameters
	/// - `highs`: PySeriesStubbed - Series of high prices
	/// - `lows`: PySeriesStubbed - Series of low prices
	/// - `closes`: PySeriesStubbed - Series of closing prices
	/// - `constant_model_type`: &str - Type of moving average for ATR calculation (e.g., "sma", "ema", "wma")
	/// - `multiplier`: f64 - Multiplier for the ATR to determine trend sensitivity
	/// - `period`: usize - Rolling window period for ATR calculation
	///
	/// # Returns
	/// Series containing:
	/// - `supertrend`: Vec<f64> - Time series of supertrend values (support/resistance levels)
	#[staticmethod]
	fn supertrend_bulk(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		closes: PySeriesStubbed,
		constant_model_type: &str,
		multiplier: f64,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let high_values: Vec<f64> = extract_f64_values(highs)?;
		let low_values: Vec<f64> = extract_f64_values(lows)?;
		let close_values: Vec<f64> = extract_f64_values(closes)?;
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let supertrend_result = rust_ti::candle_indicators::bulk::supertrend(&high_values, &low_values, &close_values, &constant_type, &multiplier, &period);

		let result_series = Series::new("supertrend".into(), supertrend_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}
