use {
	crate::utils::{create_triple_df, extract_f64_values, parse_constant_model_type},
	ezpz_stubz::{frame::PyDfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

/// Trend Technical Indicators - A collection of trend analysis functions for financial data
#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct TrendTI;

#[gen_stub_pymethods]
#[pymethods]
impl TrendTI {
	// Single value functions (return a single value from the entire series)

	/// Calculate Aroon Up indicator for a single value
	///
	/// The Aroon Up indicator measures the strength of upward price momentum by calculating
	/// the percentage of time since the highest high within the given period.
	///
	/// # Arguments
	/// * `highs` - PySeriesStubbed containing high price values
	///
	/// # Returns
	/// * `PyResult<f64>` - Aroon Up value (0-100), where higher values indicate stronger upward momentum
	///
	/// # Errors
	/// * Returns PyValueError if highs series is empty
	#[staticmethod]
	fn aroon_up_single(highs: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(highs)?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Highs cannot be empty"));
		}

		let result = rust_ti::trend_indicators::single::aroon_up(&values);
		Ok(result)
	}

	/// Calculate Aroon Down indicator for a single value
	///
	/// The Aroon Down indicator measures the strength of downward price momentum by calculating
	/// the percentage of time since the lowest low within the given period.
	///
	/// # Arguments
	/// * `lows` - PySeriesStubbed containing low price values
	///
	/// # Returns
	/// * `PyResult<f64>` - Aroon Down value (0-100), where higher values indicate stronger downward momentum
	///
	/// # Errors
	/// * Returns PyValueError if lows series is empty
	#[staticmethod]
	fn aroon_down_single(lows: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(lows)?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Lows cannot be empty"));
		}

		let result = rust_ti::trend_indicators::single::aroon_down(&values);
		Ok(result)
	}

	/// Calculate Aroon Oscillator from Aroon Up and Aroon Down values
	///
	/// The Aroon Oscillator is the difference between Aroon Up and Aroon Down indicators,
	/// providing a single measure of trend direction and strength.
	///
	/// # Arguments
	/// * `aroon_up` - f64 value of Aroon Up indicator (0-100)
	/// * `aroon_down` - f64 value of Aroon Down indicator (0-100)
	///
	/// # Returns
	/// * `PyResult<f64>` - Aroon Oscillator value (-100 to 100), where positive values indicate upward trend
	#[staticmethod]
	fn aroon_oscillator_single(aroon_up: f64, aroon_down: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::aroon_oscillator(&aroon_up, &aroon_down);
		Ok(result)
	}

	/// Calculate complete Aroon Indicator (Up, Down, and Oscillator) for single values
	///
	/// Computes all three Aroon components in one call: Aroon Up, Aroon Down, and Aroon Oscillator.
	///
	/// # Arguments
	/// * `highs` - PySeriesStubbed containing high price values
	/// * `lows` - PySeriesStubbed containing low price values
	///
	/// # Returns
	/// * `PyResult<(f64, f64, f64)>` - Tuple containing (Aroon Up, Aroon Down, Aroon Oscillator)
	///
	/// # Errors
	/// * Returns PyValueError if highs and lows series have different lengths
	#[staticmethod]
	fn aroon_indicator_single(highs: PySeriesStubbed, lows: PySeriesStubbed) -> PyResult<(f64, f64, f64)> {
		let highs_values: Vec<f64> = extract_f64_values(highs)?;
		let lows_values = extract_f64_values(lows)?;

		if highs_values.len() != lows_values.len() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
				"Length of highs ({}) must match length of lows ({})",
				highs_values.len(),
				lows_values.len()
			)));
		}

		let result = rust_ti::trend_indicators::single::aroon_indicator(&highs_values, &lows_values);
		Ok(result)
	}

	/// Calculate Parabolic SAR for long positions (single value)
	///
	/// Computes the Stop and Reverse point for long positions in the Parabolic Time/Price System.
	///
	/// # Arguments
	/// * `previous_sar` - f64 previous SAR value
	/// * `extreme_point` - f64 highest high reached during the current trend
	/// * `acceleration_factor` - f64 current acceleration factor (typically 0.02 to 0.20)
	/// * `low` - f64 current period's low price
	///
	/// # Returns
	/// * `PyResult<f64>` - New SAR value for long position
	#[staticmethod]
	fn long_parabolic_time_price_system_single(previous_sar: f64, extreme_point: f64, acceleration_factor: f64, low: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::long_parabolic_time_price_system(&previous_sar, &extreme_point, &acceleration_factor, &low);
		Ok(result)
	}

	/// Calculate Parabolic SAR for short positions (single value)
	///
	/// Computes the Stop and Reverse point for short positions in the Parabolic Time/Price System.
	///
	/// # Arguments
	/// * `previous_sar` - f64 previous SAR value
	/// * `extreme_point` - f64 lowest low reached during the current trend
	/// * `acceleration_factor` - f64 current acceleration factor (typically 0.02 to 0.20)
	/// * `high` - f64 current period's high price
	///
	/// # Returns
	/// * `PyResult<f64>` - New SAR value for short position
	#[staticmethod]
	fn short_parabolic_time_price_system_single(previous_sar: f64, extreme_point: f64, acceleration_factor: f64, high: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::short_parabolic_time_price_system(&previous_sar, &extreme_point, &acceleration_factor, &high);
		Ok(result)
	}

	/// Calculate Volume Price Trend indicator (single value)
	///
	/// VPT combines price and volume to show the relationship between a security's price movement and volume.
	///
	/// # Arguments
	/// * `current_price` - f64 current period's price
	/// * `previous_price` - f64 previous period's price
	/// * `volume` - f64 current period's volume
	/// * `previous_volume_price_trend` - f64 previous VPT value
	///
	/// # Returns
	/// * `PyResult<f64>` - New Volume Price Trend value
	#[staticmethod]
	fn volume_price_trend_single(current_price: f64, previous_price: f64, volume: f64, previous_volume_price_trend: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::volume_price_trend(&current_price, &previous_price, &volume, &previous_volume_price_trend);
		Ok(result)
	}

	/// Calculate True Strength Index (single value)
	///
	/// TSI is a momentum oscillator that uses moving averages of price changes to filter out price noise.
	///
	/// # Arguments
	/// * `prices` - PySeriesStubbed containing price values
	/// * `first_constant_model` - &str smoothing method for first smoothing ("SimpleMovingAverage", "ExponentialMovingAverage", etc.)
	/// * `first_period` - usize period for first smoothing
	/// * `second_constant_model` - &str smoothing method for second smoothing
	///
	/// # Returns
	/// * `PyResult<f64>` - True Strength Index value (-100 to 100)
	///
	/// # Errors
	/// * Returns PyValueError if prices series is empty or invalid constant model type
	#[staticmethod]
	fn true_strength_index_single(prices: PySeriesStubbed, first_constant_model: &str, first_period: usize, second_constant_model: &str) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Prices cannot be empty"));
		}

		// Convert string to ConstantModelType
		let first_model = parse_constant_model_type(first_constant_model)?;
		let second_model = parse_constant_model_type(second_constant_model)?;

		let result = rust_ti::trend_indicators::single::true_strength_index(&values, &first_model, &first_period, &second_model);
		Ok(result)
	}

	// Bulk functions (return series of values)

	/// Calculate Aroon Up indicator for time series data
	///
	/// Computes Aroon Up values for each period in the time series, measuring upward momentum strength.
	///
	/// # Arguments
	/// * `highs` - PySeriesStubbed containing high price values
	/// * `period` - usize lookback period for calculation (typically 14)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series of Aroon Up values (0-100) named "aroon_up"
	#[staticmethod]
	fn aroon_up_bulk(highs: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let highs_values: Vec<f64> = extract_f64_values(highs)?;

		let result = rust_ti::trend_indicators::bulk::aroon_up(&highs_values, &period);
		let result_series = Series::new("aroon_up".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate Aroon Down indicator for time series data
	///
	/// Computes Aroon Down values for each period in the time series, measuring downward momentum strength.
	///
	/// # Arguments
	/// * `lows` - PySeriesStubbed containing low price values
	/// * `period` - usize lookback period for calculation (typically 14)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series of Aroon Down values (0-100) named "aroon_down"
	#[staticmethod]
	fn aroon_down_bulk(lows: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let lows_values: Vec<f64> = extract_f64_values(lows)?;

		let result = rust_ti::trend_indicators::bulk::aroon_down(&lows_values, &period);
		let result_series = Series::new("aroon_down".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate Aroon Oscillator for time series data
	///
	/// Computes the difference between Aroon Up and Aroon Down for each period.
	///
	/// # Arguments
	/// * `aroon_up` - PySeriesStubbed containing Aroon Up values (0-100)
	/// * `aroon_down` - PySeriesStubbed containing Aroon Down values (0-100)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series of Aroon Oscillator values (-100 to 100) named "aroon_oscillator"
	#[staticmethod]
	fn aroon_oscillator_bulk(aroon_up: PySeriesStubbed, aroon_down: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let aroon_up_values: Vec<f64> = extract_f64_values(aroon_up)?;
		let aroon_down_values: Vec<f64> = extract_f64_values(aroon_down)?;

		let result = rust_ti::trend_indicators::bulk::aroon_oscillator(&aroon_up_values, &aroon_down_values);
		let result_series = Series::new("aroon_oscillator".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate complete Aroon Indicator system for time series data
	///
	/// Computes Aroon Up, Aroon Down, and Aroon Oscillator for each period in one operation.
	///
	/// # Arguments
	/// * `highs` - PySeriesStubbed containing high price values
	/// * `lows` - PySeriesStubbed containing low price values
	/// * `period` - usize lookback period for calculation (typically 14)
	///
	/// # Returns
	/// * `PyResult<PyDfStubbed>` - DataFrame with columns: "aroon_up", "aroon_down", "aroon_oscillator"
	#[staticmethod]
	fn aroon_indicator_bulk(highs: PySeriesStubbed, lows: PySeriesStubbed, period: usize) -> PyResult<PyDfStubbed> {
		let highs_values: Vec<f64> = extract_f64_values(highs)?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;

		let aroon_result = rust_ti::trend_indicators::bulk::aroon_indicator(&highs_values, &lows_values, &period);

		// Extract individual components from tuples
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

	/// Calculate Parabolic Time Price System (SAR) for time series data
	///
	/// Computes Stop and Reverse points for trend-following system that provides trailing stop levels.
	///
	/// # Arguments
	/// * `highs` - PySeriesStubbed containing high price values
	/// * `lows` - PySeriesStubbed containing low price values
	/// * `acceleration_factor_start` - f64 initial acceleration factor (typically 0.02)
	/// * `acceleration_factor_max` - f64 maximum acceleration factor (typically 0.20)
	/// * `acceleration_factor_step` - f64 acceleration factor increment (typically 0.02)
	/// * `start_position` - &str initial position: "Long" or "Short"
	/// * `previous_sar` - f64 initial SAR value
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series of SAR values named "parabolic_sar"
	///
	/// # Errors
	/// * Returns PyValueError if start_position is not "Long" or "Short"
	#[staticmethod]
	fn parabolic_time_price_system_bulk(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		acceleration_factor_start: f64,
		acceleration_factor_max: f64,
		acceleration_factor_step: f64,
		start_position: &str, // "Long" or "Short"
		previous_sar: f64,
	) -> PyResult<PySeriesStubbed> {
		let highs_values: Vec<f64> = extract_f64_values(highs)?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;

		let position = match start_position {
			"Long" => rust_ti::Position::Long,
			"Short" => rust_ti::Position::Short,
			_ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid position. Use 'Long' or 'Short'".to_string())),
		};

		let result = rust_ti::trend_indicators::bulk::parabolic_time_price_system(
			&highs_values,
			&lows_values,
			&acceleration_factor_start,
			&acceleration_factor_max,
			&acceleration_factor_step,
			&position,
			&previous_sar,
		);

		let result_series = Series::new("parabolic_sar".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate Directional Movement System indicators for time series data
	///
	/// Computes the complete DMS including Positive Directional Indicator (+DI), Negative Directional
	/// Indicator (-DI), Average Directional Index (ADX), and Average Directional Rating (ADXR).
	///
	/// # Arguments
	/// * `highs` - PySeriesStubbed containing high price values
	/// * `lows` - PySeriesStubbed containing low price values
	/// * `closes` - PySeriesStubbed containing close price values
	/// * `period` - usize calculation period (typically 14)
	/// * `constant_model_type` - &str smoothing method: "SimpleMovingAverage", "SmoothedMovingAverage", "ExponentialMovingAverage", etc.
	///
	/// # Returns
	/// * `PyResult<PyDfStubbed>` - DataFrame with columns: "positive_di", "negative_di", "adx", "adxr"
	///
	/// # Errors
	/// * Returns PyValueError for invalid constant model type
	/// * Returns PyRuntimeError if DataFrame creation fails
	#[staticmethod]
	fn directional_movement_system_bulk(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		closes: PySeriesStubbed,
		period: usize,
		constant_model_type: &str, // "SimpleMovingAverage", "SmoothedMovingAverage", "ExponentialMovingAverage", etc.
	) -> PyResult<PyDfStubbed> {
		let highs_values: Vec<f64> = extract_f64_values(highs)?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;
		let closes_values: Vec<f64> = extract_f64_values(closes)?;

		let constant_model = parse_constant_model_type(constant_model_type)?;

		let dm_result = rust_ti::trend_indicators::bulk::directional_movement_system(&highs_values, &lows_values, &closes_values, &period, &constant_model);

		// Extract individual components from tuples
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
		.map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("DataFrame creation failed: {e}")))?;

		Ok(PyDfStubbed(pyo3_polars::PyDataFrame(df)))
	}

	/// Calculate Volume Price Trend indicator for time series data
	///
	/// VPT combines price and volume to show the relationship between price movement and volume flow.
	///
	/// # Arguments
	/// * `prices` - PySeriesStubbed containing price values
	/// * `volumes` - PySeriesStubbed containing volume values
	/// * `previous_volume_price_trend` - f64 initial VPT value (typically 0)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series of Volume Price Trend values named "volume_price_trend"
	#[staticmethod]
	fn volume_price_trend_bulk(prices: PySeriesStubbed, volumes: PySeriesStubbed, previous_volume_price_trend: f64) -> PyResult<PySeriesStubbed> {
		let prices_values: Vec<f64> = extract_f64_values(prices)?;
		let volumes_values: Vec<f64> = extract_f64_values(volumes)?;

		let result = rust_ti::trend_indicators::bulk::volume_price_trend(&prices_values, &volumes_values, &previous_volume_price_trend);

		let result_series = Series::new("volume_price_trend".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate True Strength Index for time series data
	///
	/// TSI is a momentum oscillator that uses double-smoothed price changes to filter noise
	/// and provide clearer signals of price momentum direction and strength.
	///
	/// # Arguments
	/// * `prices` - PySeriesStubbed containing price values
	/// * `first_constant_model` - &str first smoothing method: "SimpleMovingAverage", "ExponentialMovingAverage", etc.
	/// * `first_period` - usize period for first smoothing (typically 25)
	/// * `second_constant_model` - &str second smoothing method
	/// * `second_period` - usize period for second smoothing (typically 13)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series of TSI values (-100 to 100) named "true_strength_index"
	///
	/// # Errors
	/// * Returns PyValueError for invalid constant model types
	#[staticmethod]
	fn true_strength_index_bulk(
		prices: PySeriesStubbed,
		first_constant_model: &str,
		first_period: usize,
		second_constant_model: &str,
		second_period: usize,
	) -> PyResult<PySeriesStubbed> {
		let prices_values: Vec<f64> = extract_f64_values(prices)?;

		let first_model = parse_constant_model_type(first_constant_model)?;
		let second_model = parse_constant_model_type(second_constant_model)?;

		let result = rust_ti::trend_indicators::bulk::true_strength_index(&prices_values, &first_model, &first_period, &second_model, &second_period);

		let result_series = Series::new("true_strength_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}
