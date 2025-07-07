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
pub struct TrendTI {
	pub series: PySeriesStubbed,
}

#[gen_stub_pymethods]
#[pymethods]
impl TrendTI {
	#[new]
	fn new(series: PySeriesStubbed) -> Self {
		Self { series }
	}

	// Single value functions (return a single value from the entire series)

	/// Aroon Up (Single) - Measures the strength of upward price momentum
	/// Calculates the percentage of time since the highest high within the series
	///
	/// # Returns
	/// f64 - Aroon Up value (0-100), where higher values indicate stronger upward momentum
	fn aroon_up_single(&self) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}
		let result = rust_ti::trend_indicators::single::aroon_up(&values);
		Ok(result)
	}

	/// Aroon Down (Single) - Measures the strength of downward price momentum
	/// Calculates the percentage of time since the lowest low within the series
	///
	/// # Returns
	/// f64 - Aroon Down value (0-100), where higher values indicate stronger downward momentum
	fn aroon_down_single(&self) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}
		let result = rust_ti::trend_indicators::single::aroon_down(&values);
		Ok(result)
	}

	/// Aroon Oscillator (Single) - Calculates the difference between Aroon Up and Aroon Down
	/// Provides a single measure of trend direction and strength
	///
	/// # Parameters
	/// - `lows`: PySeriesStubbed - Series of low price values for Aroon Down calculation
	///
	/// # Returns
	/// f64 - Aroon Oscillator value (-100 to 100), where positive values indicate upward trend
	fn aroon_oscillator_single(&self, lows: PySeriesStubbed) -> PyResult<f64> {
		let highs_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;

		if highs_values.len() != lows_values.len() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Length of highs must match length of lows"));
		}

		let result = rust_ti::trend_indicators::single::aroon_indicator(&highs_values, &lows_values);
		Ok(result.2) // Return the oscillator component
	}

	/// Aroon Indicator (Single) - Calculates complete Aroon system in one call
	/// Computes Aroon Up, Aroon Down, and Aroon Oscillator
	///
	/// # Parameters
	/// - `lows`: PySeriesStubbed - Series of low price values
	///
	/// # Returns
	/// (f64, f64, f64) - Tuple containing (Aroon Up, Aroon Down, Aroon Oscillator)
	fn aroon_indicator_single(&self, lows: PySeriesStubbed) -> PyResult<(f64, f64, f64)> {
		let highs_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;

		if highs_values.len() != lows_values.len() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Length of highs must match length of lows"));
		}

		let result = rust_ti::trend_indicators::single::aroon_indicator(&highs_values, &lows_values);
		Ok(result)
	}

	/// True Strength Index (Single) - Momentum oscillator using double-smoothed price changes
	/// Filters out price noise to provide clearer momentum signals
	///
	/// # Parameters
	/// - `first_constant_model`: &str - First smoothing method ("SimpleMovingAverage", "ExponentialMovingAverage", etc.)
	/// - `first_period`: usize - Period for first smoothing
	/// - `second_constant_model`: &str - Second smoothing method
	///
	/// # Returns
	/// f64 - True Strength Index value (-100 to 100)
	fn true_strength_index_single(&self, first_constant_model: &str, first_period: usize, second_constant_model: &str) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}

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
	/// - `period`: usize - Lookback period for calculation (typically 14)
	///
	/// # Returns
	/// PySeriesStubbed - Series of Aroon Up values (0-100) named "aroon_up"
	fn aroon_up_bulk(&self, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let result = rust_ti::trend_indicators::bulk::aroon_up(&values, period);
		let result_series = Series::new("aroon_up".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Aroon Down (Bulk) - Calculates rolling Aroon Down indicator over specified period
	/// Measures downward momentum strength for each period in the time series
	///
	/// # Parameters
	/// - `period`: usize - Lookback period for calculation (typically 14)
	///
	/// # Returns
	/// PySeriesStubbed - Series of Aroon Down values (0-100) named "aroon_down"
	fn aroon_down_bulk(&self, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let result = rust_ti::trend_indicators::bulk::aroon_down(&values, period);
		let result_series = Series::new("aroon_down".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Aroon Oscillator (Bulk) - Calculates rolling Aroon Oscillator over specified period
	/// Computes the difference between Aroon Up and Aroon Down for each period
	///
	/// # Parameters
	/// - `lows`: PySeriesStubbed - Series of low price values
	/// - `period`: usize - Lookback period for calculation (typically 14)
	///
	/// # Returns
	/// PySeriesStubbed - Series of Aroon Oscillator values (-100 to 100) named "aroon_oscillator"
	fn aroon_oscillator_bulk(&self, lows: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let highs_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;

		let aroon_up_result = rust_ti::trend_indicators::bulk::aroon_up(&highs_values, period);
		let aroon_down_result = rust_ti::trend_indicators::bulk::aroon_down(&lows_values, period);

		let result = rust_ti::trend_indicators::bulk::aroon_oscillator(&aroon_up_result, &aroon_down_result);
		let result_series = Series::new("aroon_oscillator".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Aroon Indicator (Bulk) - Calculates complete Aroon system for time series data
	/// Computes Aroon Up, Aroon Down, and Aroon Oscillator for each period
	///
	/// # Parameters
	/// - `lows`: PySeriesStubbed - Series of low price values
	/// - `period`: usize - Lookback period for calculation (typically 14)
	///
	/// # Returns
	/// PyDfStubbed - DataFrame with columns: "aroon_up", "aroon_down", "aroon_oscillator"
	fn aroon_indicator_bulk(&self, lows: PySeriesStubbed, period: usize) -> PyResult<PyDfStubbed> {
		let highs_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;

		let aroon_result = rust_ti::trend_indicators::bulk::aroon_indicator(&highs_values, &lows_values, period);

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
	/// - `lows`: PySeriesStubbed - Series of low price values
	/// - `acceleration_factor_start`: f64 - Initial acceleration factor (typically 0.02)
	/// - `acceleration_factor_max`: f64 - Maximum acceleration factor (typically 0.20)
	/// - `acceleration_factor_step`: f64 - Acceleration factor increment (typically 0.02)
	/// - `start_position`: &str - Initial position: "Long" or "Short"
	/// - `previous_sar`: f64 - Initial SAR value
	///
	/// # Returns
	/// PySeriesStubbed - Series of SAR values named "parabolic_sar"
	fn parabolic_time_price_system_bulk(
		&self,
		lows: PySeriesStubbed,
		acceleration_factor_start: f64,
		acceleration_factor_max: f64,
		acceleration_factor_step: f64,
		start_position: &str,
		previous_sar: f64,
	) -> PyResult<PySeriesStubbed> {
		let highs_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;

		let position = match start_position {
			"Long" => rust_ti::Position::Long,
			"Short" => rust_ti::Position::Short,
			_ => return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid position. Use 'Long' or 'Short'")),
		};

		let result = rust_ti::trend_indicators::bulk::parabolic_time_price_system(
			&highs_values,
			&lows_values,
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
	/// - `lows`: PySeriesStubbed - Series of low price values
	/// - `closes`: PySeriesStubbed - Series of close price values
	/// - `period`: usize - Calculation period (typically 14)
	/// - `constant_model_type`: &str - Smoothing method: "SimpleMovingAverage", "SmoothedMovingAverage", etc.
	///
	/// # Returns
	/// PyDfStubbed - DataFrame with columns: "positive_di", "negative_di", "adx", "adxr"
	fn directional_movement_system_bulk(
		&self,
		lows: PySeriesStubbed,
		closes: PySeriesStubbed,
		period: usize,
		constant_model_type: &str,
	) -> PyResult<PyDfStubbed> {
		let highs_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;
		let closes_values: Vec<f64> = extract_f64_values(closes)?;

		let constant_model = parse_constant_model_type(constant_model_type)?;

		let dm_result = rust_ti::trend_indicators::bulk::directional_movement_system(&highs_values, &lows_values, &closes_values, period, constant_model);

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
	/// - `volumes`: PySeriesStubbed - Series of volume values
	/// - `previous_volume_price_trend`: f64 - Initial VPT value (typically 0)
	///
	/// # Returns
	/// PySeriesStubbed - Series of Volume Price Trend values named "volume_price_trend"
	fn volume_price_trend_bulk(&self, volumes: PySeriesStubbed, previous_volume_price_trend: f64) -> PyResult<PySeriesStubbed> {
		let prices_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let volumes_values: Vec<f64> = extract_f64_values(volumes)?;

		let result = rust_ti::trend_indicators::bulk::volume_price_trend(&prices_values, &volumes_values, previous_volume_price_trend);

		let result_series = Series::new("volume_price_trend".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// True Strength Index (Bulk) - Double-smoothed momentum oscillator
	/// Uses double-smoothed price changes to filter noise and provide clearer signals
	///
	/// # Parameters
	/// - `first_constant_model`: &str - First smoothing method: "SimpleMovingAverage", "ExponentialMovingAverage", etc.
	/// - `first_period`: usize - Period for first smoothing (typically 25)
	/// - `second_constant_model`: &str - Second smoothing method
	/// - `second_period`: usize - Period for second smoothing (typically 13)
	///
	/// # Returns
	/// PySeriesStubbed - Series of TSI values (-100 to 100) named "true_strength_index"
	fn true_strength_index_bulk(
		&self,
		first_constant_model: &str,
		first_period: usize,
		second_constant_model: &str,
		second_period: usize,
	) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;

		let first_model = parse_constant_model_type(first_constant_model)?;
		let second_model = parse_constant_model_type(second_constant_model)?;

		let result = rust_ti::trend_indicators::bulk::true_strength_index(&values, first_model, first_period, second_model, second_period);

		let result_series = Series::new("true_strength_index".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}
