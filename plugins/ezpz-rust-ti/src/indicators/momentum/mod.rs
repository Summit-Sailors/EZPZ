use {
	crate::utils::{create_triple_df, extract_f64_values, parse_constant_model_type, parse_deviation_model},
	ezpz_stubz::{frame::PyDfStubbed, lazy::PyLfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct MomentumTI {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl MomentumTI {
	#[new]
	fn new(lf: PyLfStubbed) -> Self {
		Self { lf: lf.0.into() }
	}

	/// Aroon Up indicator
	///
	/// Calculates the Aroon Up indicator, which measures the time since the highest high
	/// within a given period as a percentage.
	///
	/// # Parameters
	/// * `high_column` - &str name of the column containing high price values
	///
	/// # Returns
	/// * `PyResult<f64>` - The Aroon Up value (0-100), where higher values indicate recent highs
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

	/// Aroon Down indicator
	///
	/// Calculates the Aroon Down indicator, which measures the time since the lowest low
	/// within a given period as a percentage.
	///
	/// # Parameters
	/// * `low_column` - &str name of the column containing low price values
	///
	/// # Returns
	/// * `PyResult<f64>` - The Aroon Down value (0-100), where higher values indicate recent lows
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

	/// Aroon Oscillator
	///
	/// Calculates the Aroon Oscillator by subtracting Aroon Down from Aroon Up.
	/// Values range from -100 to +100, indicating trend strength and direction.
	///
	/// # Parameters
	/// * `aroon_up` - f64 value of Aroon Up indicator (0-100)
	/// * `aroon_down` - f64 value of Aroon Down indicator (0-100)
	///
	/// # Returns
	/// * `PyResult<f64>` - The Aroon Oscillator value (-100 to +100)
	fn aroon_oscillator_single(&self, aroon_up: f64, aroon_down: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::aroon_oscillator(aroon_up, aroon_down);
		Ok(result)
	}

	/// Aroon Indicator (complete calculation)
	///
	/// Calculates all three Aroon components: Aroon Up, Aroon Down, and Aroon Oscillator
	/// in a single function call.
	///
	/// # Parameters
	/// * `high_column` - &str name of the column containing high price values
	/// * `low_column` - &str name of the column containing low price values
	///
	/// # Returns
	/// * `PyResult<(f64, f64, f64)>` - Tuple containing (aroon_up, aroon_down, aroon_oscillator)
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

	/// Long Parabolic Time Price System (Parabolic SAR for long positions)
	///
	/// Calculates the Parabolic SAR (Stop and Reverse) for long positions, used to determine
	/// potential reversal points in price movement.
	///
	/// # Parameters
	/// * `previous_sar` - f64 value of the previous SAR
	/// * `extreme_point` - f64 value of the extreme point (highest high for long positions)
	/// * `acceleration_factor` - f64 acceleration factor (typically starts at 0.02)
	/// * `low` - f64 current period's low price
	///
	/// # Returns
	/// * `PyResult<f64>` - The calculated SAR value for long positions
	fn long_parabolic_time_price_system_single(&self, previous_sar: f64, extreme_point: f64, acceleration_factor: f64, low: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::long_parabolic_time_price_system(previous_sar, extreme_point, acceleration_factor, low);
		Ok(result)
	}

	/// Short Parabolic Time Price System (Parabolic SAR for short positions)
	///
	/// Calculates the Parabolic SAR (Stop and Reverse) for short positions, used to determine
	/// potential reversal points in price movement.
	///
	/// # Parameters
	/// * `previous_sar` - f64 value of the previous SAR
	/// * `extreme_point` - f64 value of the extreme point (lowest low for short positions)
	/// * `acceleration_factor` - f64 acceleration factor (typically starts at 0.02)
	/// * `high` - f64 current period's high price
	///
	/// # Returns
	/// * `PyResult<f64>` - The calculated SAR value for short positions
	fn short_parabolic_time_price_system_single(&self, previous_sar: f64, extreme_point: f64, acceleration_factor: f64, high: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::short_parabolic_time_price_system(previous_sar, extreme_point, acceleration_factor, high);
		Ok(result)
	}

	/// Volume Price Trend
	///
	/// Calculates the Volume Price Trend indicator, which combines price and volume
	/// to show the relationship between volume and price changes.
	///
	/// # Parameters
	/// * `price_column` - &str name of the column containing price values
	/// * `previous_price` - f64 previous period's price
	/// * `volume` - f64 current period's volume
	/// * `previous_volume_price_trend` - f64 previous VPT value
	///
	/// # Returns
	/// * `PyResult<f64>` - The calculated Volume Price Trend value
	fn volume_price_trend_single(&self, price_column: &str, previous_price: f64, volume: f64, previous_volume_price_trend: f64) -> PyResult<f64> {
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
		let current_price = values[0];
		let result = rust_ti::trend_indicators::single::volume_price_trend(current_price, previous_price, volume, previous_volume_price_trend);
		Ok(result)
	}

	/// True Strength Index
	///
	/// Calculates the True Strength Index, a momentum oscillator that uses price changes
	/// smoothed by two exponential moving averages.
	///
	/// # Parameters
	/// * `price_column` - &str name of the column containing price values
	/// * `first_constant_model` - &str smoothing model for first smoothing ("sma", "ema", etc.)
	/// * `first_period` - usize period for first smoothing
	/// * `second_constant_model` - &str smoothing model for second smoothing ("sma", "ema", etc.)
	///
	/// # Returns
	/// * `PyResult<f64>` - The True Strength Index value (typically ranges from -100 to +100)
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

	/// Relative Strength Index (RSI) - bulk calculation
	///
	/// Calculates RSI values for an entire series of prices. RSI measures the speed and change
	/// of price movements, oscillating between 0 and 100.
	///
	/// # Parameters
	/// * `price_column` - &str name of the column containing price values
	/// * `constant_model_type` - &str smoothing model ("sma", "ema", etc.)
	/// * `period` - usize calculation period (commonly 14)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "rsi" containing RSI values (0-100)
	fn relative_strength_index_bulk(&self, price_column: &str, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
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
		let model_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::momentum_indicators::bulk::relative_strength_index(&values, model_type, period);
		let result_series = Series::new("rsi".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Stochastic Oscillator - bulk calculation
	///
	/// Calculates the Stochastic Oscillator, which compares a security's closing price
	/// to its price range over a given time period.
	///
	/// # Parameters
	/// * `price_column` - &str name of the column containing price values
	/// * `period` - usize lookback period for calculation
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "stochastic" containing oscillator values (0-100)
	fn stochastic_oscillator_bulk(&self, price_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
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
		let result = rust_ti::momentum_indicators::bulk::stochastic_oscillator(&values, period);
		let result_series = Series::new("stochastic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Slow Stochastic - bulk calculation
	///
	/// Calculates the Slow Stochastic by smoothing the regular Stochastic Oscillator
	/// to reduce noise and false signals.
	///
	/// # Parameters
	/// * `stochastic_column` - &str name of the column containing Stochastic Oscillator values
	/// * `constant_model_type` - &str smoothing model ("sma", "ema", etc.)
	/// * `period` - usize smoothing period
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "slow_stochastic" containing smoothed values (0-100)
	fn slow_stochastic_bulk(&self, stochastic_column: &str, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(stochastic_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{stochastic_column}': {e}")))?
			.column(stochastic_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{stochastic_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{stochastic_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let model_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::momentum_indicators::bulk::slow_stochastic(&values, model_type, period);
		let result_series = Series::new("slow_stochastic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Slowest Stochastic - bulk calculation
	///
	/// Calculates the Slowest Stochastic by applying additional smoothing to the Slow Stochastic
	/// for even more noise reduction.
	///
	/// # Parameters
	/// * `slow_stochastic_column` - &str name of the column containing Slow Stochastic values
	/// * `constant_model_type` - &str smoothing model ("sma", "ema", etc.)
	/// * `period` - usize smoothing period
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "slowest_stochastic" containing double-smoothed values (0-100)
	fn slowest_stochastic_bulk(&self, slow_stochastic_column: &str, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(slow_stochastic_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{slow_stochastic_column}': {e}")))?
			.column(slow_stochastic_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{slow_stochastic_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{slow_stochastic_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let model_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::momentum_indicators::bulk::slowest_stochastic(&values, model_type, period);
		let result_series = Series::new("slowest_stochastic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Williams %R - bulk calculation
	///
	/// Calculates Williams %R, a momentum indicator that measures overbought and oversold levels.
	/// Values range from -100 to 0, where -20 and above indicates overbought, -80 and below indicates oversold.
	///
	/// # Parameters
	/// * `high_column` - &str name of the column containing high price values
	/// * `low_column` - &str name of the column containing low price values
	/// * `close_column` - &str name of the column containing close price values
	/// * `period` - usize lookback period for calculation
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "williams_r" containing Williams %R values (-100 to 0)
	fn williams_percent_r_bulk(&self, high_column: &str, low_column: &str, close_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
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
		let result = rust_ti::momentum_indicators::bulk::williams_percent_r(&high_values, &low_values, &close_values, period);
		let result_series = Series::new("williams_r".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Money Flow Index - bulk calculation
	///
	/// Calculates the Money Flow Index, a volume-weighted RSI that measures buying and selling pressure.
	/// Values range from 0 to 100, where >80 indicates overbought and <20 indicates oversold.
	///
	/// # Parameters
	/// * `price_column` - &str name of the column containing price values
	/// * `volume_column` - &str name of the column containing volume values
	/// * `period` - usize calculation period (commonly 14)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "mfi" containing Money Flow Index values (0-100)
	fn money_flow_index_bulk(&self, price_column: &str, volume_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
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
		let result = rust_ti::momentum_indicators::bulk::money_flow_index(&price_values, &volume_values, period);
		let result_series = Series::new("mfi".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Rate of Change - bulk calculation
	///
	/// Calculates the Rate of Change, which measures the percentage change in price
	/// from one period to the next.
	///
	/// # Parameters
	/// * `price_column` - &str name of the column containing price values
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "roc" containing rate of change values as percentages
	fn rate_of_change_bulk(&self, price_column: &str) -> PyResult<PySeriesStubbed> {
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
		let result = rust_ti::momentum_indicators::bulk::rate_of_change(&values);
		let result_series = Series::new("roc".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// On Balance Volume (Bulk) - Calculates cumulative volume indicator
	/// Adds volume on up days and subtracts volume on down days to measure buying and selling pressure
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `volume_column`: &str - Name of the volume column
	/// - `previous_obv`: f64 - Starting OBV value (typically 0)
	///
	/// # Returns
	/// PySeriesStubbed - Series of OBV values with name "obv"
	fn on_balance_volume_bulk(&self, price_column: &str, volume_column: &str, previous_obv: f64) -> PyResult<PySeriesStubbed> {
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
		let result = rust_ti::momentum_indicators::bulk::on_balance_volume(&price_values, &volume_values, previous_obv);
		let result_series = Series::new("obv".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Commodity Channel Index (Bulk) - Calculates CCI over rolling periods
	/// Measures the variation of a security's price from its statistical mean
	/// Values typically range from -100 to +100
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `constant_model_type`: &str - Model for calculating moving average ("sma", "ema", etc.)
	/// - `deviation_model`: &str - Model for calculating deviation ("mad", "std", etc.)
	/// - `constant_multiplier`: f64 - Multiplier constant (typically 0.015)
	/// - `period`: usize - Calculation period (commonly 20)
	///
	/// # Returns
	/// PySeriesStubbed - Series of CCI values with name "cci"
	fn commodity_channel_index_bulk(
		&self,
		price_column: &str,
		constant_model_type: &str,
		deviation_model: &str,
		constant_multiplier: f64,
		period: usize,
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
		let model_type = parse_constant_model_type(constant_model_type)?;
		let dev_model = parse_deviation_model(deviation_model)?;
		let result = rust_ti::momentum_indicators::bulk::commodity_channel_index(&values, model_type, dev_model, constant_multiplier, period);
		let result_series = Series::new("cci".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// McGinley Dynamic Commodity Channel Index (Bulk) - CCI using McGinley Dynamic MA
	/// Uses McGinley Dynamic as the moving average, which adapts to market conditions
	/// better than traditional moving averages
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `previous_mcginley_dynamic`: f64 - Initial McGinley Dynamic value
	/// - `deviation_model`: &str - Model for calculating deviation ("mad", "std", etc.)
	/// - `constant_multiplier`: f64 - Multiplier constant (typically 0.015)
	/// - `period`: usize - Calculation period
	///
	/// # Returns
	/// (PySeriesStubbed, PySeriesStubbed) - Tuple containing (CCI series, McGinley Dynamic series)
	fn mcginley_dynamic_commodity_channel_index_bulk(
		&self,
		price_column: &str,
		previous_mcginley_dynamic: f64,
		deviation_model: &str,
		constant_multiplier: f64,
		period: usize,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
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
		let dev_model = parse_deviation_model(deviation_model)?;
		let result =
			rust_ti::momentum_indicators::bulk::mcginley_dynamic_commodity_channel_index(&values, previous_mcginley_dynamic, dev_model, constant_multiplier, period);
		let (cci_values, mcginley_values): (Vec<f64>, Vec<f64>) = result.into_iter().unzip();
		let cci_series = Series::new("cci".into(), cci_values);
		let mcginley_series = Series::new("mcginley_dynamic".into(), mcginley_values);
		Ok((PySeriesStubbed(pyo3_polars::PySeries(cci_series)), PySeriesStubbed(pyo3_polars::PySeries(mcginley_series))))
	}

	/// MACD Line (Bulk) - Calculates Moving Average Convergence Divergence line
	/// Subtracts the long-period moving average from the short-period moving average
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `short_period`: usize - Period for short moving average (commonly 12)
	/// - `short_period_model`: &str - Model for short MA ("sma", "ema", etc.)
	/// - `long_period`: usize - Period for long moving average (commonly 26)
	/// - `long_period_model`: &str - Model for long MA ("sma", "ema", etc.)
	///
	/// # Returns
	/// PySeriesStubbed - Series of MACD line values with name "macd"
	fn macd_line_bulk(
		&self,
		price_column: &str,
		short_period: usize,
		short_period_model: &str,
		long_period: usize,
		long_period_model: &str,
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
		let short_model = parse_constant_model_type(short_period_model)?;
		let long_model = parse_constant_model_type(long_period_model)?;
		let result = rust_ti::momentum_indicators::bulk::macd_line(&values, short_period, short_model, long_period, long_model);
		let result_series = Series::new("macd".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Signal Line (Bulk) - Calculates MACD Signal Line
	/// Applies a moving average to the MACD line for generating buy/sell signals
	///
	/// # Parameters
	/// - `macd_column`: &str - Name of the MACD column to analyze
	/// - `constant_model_type`: &str - Smoothing model ("sma", "ema", etc.)
	/// - `period`: usize - Signal line period (commonly 9)
	///
	/// # Returns
	/// PySeriesStubbed - Series of signal line values with name "signal"
	fn signal_line_bulk(&self, macd_column: &str, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let series = self
			.lf
			.clone()
			.select([col(macd_column)])
			.collect()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to collect column '{macd_column}': {e}")))?
			.column(macd_column)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{macd_column}' not found: {e}")))?
			.as_series()
			.ok_or_else(|| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Column '{macd_column}' could not be converted to Series")))?
			.clone();

		let values: Vec<f64> = extract_f64_values(PySeriesStubbed(pyo3_polars::PySeries(series)))?;
		let model_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::momentum_indicators::bulk::signal_line(&values, model_type, period);
		let result_series = Series::new("signal".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// McGinley Dynamic MACD Line (Bulk) - MACD using McGinley Dynamic moving averages
	/// Provides better adaptation to market volatility and reduces lag compared to traditional MACD
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `short_period`: usize - Period for short McGinley Dynamic
	/// - `previous_short_mcginley`: f64 - Initial short McGinley Dynamic value
	/// - `long_period`: usize - Period for long McGinley Dynamic
	/// - `previous_long_mcginley`: f64 - Initial long McGinley Dynamic value
	///
	/// # Returns
	/// PyDfStubbed - DataFrame with columns: "macd", "short_mcginley", "long_mcginley"
	fn mcginley_dynamic_macd_line_bulk(
		&self,
		price_column: &str,
		short_period: usize,
		previous_short_mcginley: f64,
		long_period: usize,
		previous_long_mcginley: f64,
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
		let result =
			rust_ti::momentum_indicators::bulk::mcginley_dynamic_macd_line(&values, short_period, previous_short_mcginley, long_period, previous_long_mcginley);
		let (macd_values, short_mcginley_values, long_mcginley_values): (Vec<f64>, Vec<f64>, Vec<f64>) =
			result.into_iter().fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, (a, b, c)| {
				acc.0.push(a);
				acc.1.push(b);
				acc.2.push(c);
				acc
			});
		create_triple_df(macd_values, short_mcginley_values, long_mcginley_values, "macd", "short_mcginley", "long_mcginley")
	}

	/// Chaikin Oscillator (Bulk) - Applies MACD to Accumulation/Distribution line
	/// Measures the momentum of the Accumulation/Distribution line
	///
	/// # Parameters
	/// - `high_column`: &str - Name of the high price column
	/// - `low_column`: &str - Name of the low price column
	/// - `close_column`: &str - Name of the close price column
	/// - `volume_column`: &str - Name of the volume column
	/// - `short_period`: usize - Short period for oscillator (commonly 3)
	/// - `long_period`: usize - Long period for oscillator (commonly 10)
	/// - `previous_accumulation_distribution`: f64 - Initial A/D line value
	/// - `short_period_model`: &str - Model for short MA ("sma", "ema", etc.)
	/// - `long_period_model`: &str - Model for long MA ("sma", "ema", etc.)
	///
	/// # Returns
	/// (PySeriesStubbed, PySeriesStubbed) - Tuple containing (Chaikin Oscillator, A/D Line)
	#[allow(clippy::too_many_arguments)]
	fn chaikin_oscillator_bulk(
		&self,
		high_column: &str,
		low_column: &str,
		close_column: &str,
		volume_column: &str,
		short_period: usize,
		long_period: usize,
		previous_accumulation_distribution: f64,
		short_period_model: &str,
		long_period_model: &str,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
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
		let short_model = parse_constant_model_type(short_period_model)?;
		let long_model = parse_constant_model_type(long_period_model)?;
		let result = rust_ti::momentum_indicators::bulk::chaikin_oscillator(
			&high_values,
			&low_values,
			&close_values,
			&volume_values,
			short_period,
			long_period,
			previous_accumulation_distribution,
			short_model,
			long_model,
		);
		let (chaikin_values, ad_values): (Vec<f64>, Vec<f64>) = result.into_iter().unzip();
		let chaikin_series = Series::new("chaikin_oscillator".into(), chaikin_values);
		let ad_series = Series::new("accumulation_distribution".into(), ad_values);
		Ok((PySeriesStubbed(pyo3_polars::PySeries(chaikin_series)), PySeriesStubbed(pyo3_polars::PySeries(ad_series))))
	}

	/// Percentage Price Oscillator (Bulk) - MACD expressed as percentage
	/// Similar to MACD but expressed as a percentage for easier comparison across securities
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `short_period`: usize - Short period for moving average (commonly 12)
	/// - `long_period`: usize - Long period for moving average (commonly 26)
	/// - `constant_model_type`: &str - Model for moving averages ("sma", "ema", etc.)
	///
	/// # Returns
	/// PySeriesStubbed - Series of PPO values as percentages with name "ppo"
	fn percentage_price_oscillator_bulk(
		&self,
		price_column: &str,
		short_period: usize,
		long_period: usize,
		constant_model_type: &str,
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
		let model_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::momentum_indicators::bulk::percentage_price_oscillator(&values, short_period, long_period, model_type);
		let result_series = Series::new("ppo".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Chande Momentum Oscillator (Bulk) - Measures momentum using gains and losses
	/// Calculates the difference between sum of gains and losses over a period
	/// Values range from -100 to +100
	///
	/// # Parameters
	/// - `price_column`: &str - Name of the price column to analyze
	/// - `period`: usize - Calculation period (commonly 14 or 20)
	///
	/// # Returns
	/// PySeriesStubbed - Series of CMO values (-100 to +100) with name "chande_momentum_oscillator"
	fn chande_momentum_oscillator_bulk(&self, price_column: &str, period: usize) -> PyResult<PySeriesStubbed> {
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
		let result = rust_ti::momentum_indicators::bulk::chande_momentum_oscillator(&values, period);
		let result_series = Series::new("chande_momentum_oscillator".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_abs_diff_eq;
	use ezpz_stubz::lazy::PyLfStubbed;

	fn create_test_dataframe() -> LazyFrame {
		df! {
			"high" => vec![12.0, 15.0, 18.0, 16.0, 14.0, 17.0, 19.0, 21.0, 20.0, 22.0],
			"low" => vec![10.0, 11.0, 13.0, 12.0, 10.0, 14.0, 15.0, 17.0, 16.0, 18.0],
			"close" => vec![11.0, 13.0, 16.0, 14.0, 12.0, 16.0, 18.0, 19.0, 18.0, 20.0],
			"price" => vec![11.0, 13.0, 16.0, 14.0, 12.0, 16.0, 18.0, 19.0, 18.0, 20.0],
			"volume" => vec![1000.0, 1200.0, 1500.0, 1300.0, 1100.0, 1400.0, 1600.0, 1800.0, 1700.0, 1900.0]
		}
		.unwrap()
		.lazy()
	}

	fn create_momentum_ti() -> MomentumTI {
		let lf = create_test_dataframe();
		MomentumTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(lf)))
	}

	#[test]
	fn test_aroon_up_single() {
		let ti = create_momentum_ti();
		let result = ti.aroon_up_single("high").unwrap();
		assert!((0.0..=100.0).contains(&result));
	}

	#[test]
	fn test_aroon_down_single() {
		let ti = create_momentum_ti();
		let result = ti.aroon_down_single("low").unwrap();
		assert!((0.0..=100.0).contains(&result));
	}

	#[test]
	fn test_aroon_oscillator_single() {
		let ti = create_momentum_ti();
		let aroon_up = 80.0;
		let aroon_down = 20.0;
		let result = ti.aroon_oscillator_single(aroon_up, aroon_down).unwrap();
		assert_abs_diff_eq!(result, 60.0, epsilon = 1e-10);
	}

	#[test]
	fn test_aroon_indicator_single() {
		let ti = create_momentum_ti();
		let result = ti.aroon_indicator_single("high", "low").unwrap();
		let (aroon_up, aroon_down, aroon_oscillator) = result;
		assert!((0.0..=100.0).contains(&aroon_up));
		assert!((0.0..=100.0).contains(&aroon_down));
		assert!((-100.0..=100.0).contains(&aroon_oscillator));
		assert_abs_diff_eq!(aroon_oscillator, aroon_up - aroon_down, epsilon = 1e-10);
	}

	#[test]
	fn test_long_parabolic_time_price_system_single() {
		let ti = create_momentum_ti();
		let result = ti.long_parabolic_time_price_system_single(10.0, 15.0, 0.02, 12.0).unwrap();
		assert!(result > 0.0);
	}

	#[test]
	fn test_short_parabolic_time_price_system_single() {
		let ti = create_momentum_ti();
		let result = ti.short_parabolic_time_price_system_single(20.0, 15.0, 0.02, 18.0).unwrap();
		assert!(result > 0.0);
	}

	#[test]
	fn test_volume_price_trend_single() {
		let ti = create_momentum_ti();
		let result = ti.volume_price_trend_single("price", 10.0, 1000.0, 0.0).unwrap();
		assert!(result.is_finite());
	}

	#[test]
	fn test_true_strength_index_single() {
		let ti = create_momentum_ti();
		let result = ti.true_strength_index_single("price", "ema", 14, "ema").unwrap();
		assert!((-100.0..=100.0).contains(&result));
	}

	#[test]
	fn test_relative_strength_index_bulk() {
		let ti = create_momentum_ti();
		let result = ti.relative_strength_index_bulk("price", "ema", 14).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "rsi");
		assert!(!series.is_empty());
	}

	#[test]
	fn test_stochastic_oscillator_bulk() {
		let ti = create_momentum_ti();
		let result = ti.stochastic_oscillator_bulk("price", 14).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "stochastic");
		assert!(!series.is_empty());
	}

	#[test]
	fn test_slow_stochastic_bulk() {
		let ti = create_momentum_ti();
		let stoch_result = ti.stochastic_oscillator_bulk("price", 14).unwrap();
		let df = df! {
			"stoch" => stoch_result.0.0.f64().unwrap().into_iter().map(|v| v.unwrap_or(0.0)).collect::<Vec<f64>>()
		}
		.unwrap()
		.lazy();
		let ti_with_stoch = MomentumTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(df)));
		let result = ti_with_stoch.slow_stochastic_bulk("stoch", "sma", 3).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "slow_stochastic");
	}

	#[test]
	fn test_williams_percent_r_bulk() {
		let ti = create_momentum_ti();
		let result = ti.williams_percent_r_bulk("high", "low", "close", 14).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "williams_r");
		assert!(!series.is_empty());
	}

	#[test]
	fn test_money_flow_index_bulk() {
		let ti = create_momentum_ti();
		let result = ti.money_flow_index_bulk("price", "volume", 14).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "mfi");
		assert!(!series.is_empty());
	}

	#[test]
	fn test_rate_of_change_bulk() {
		let ti = create_momentum_ti();
		let result = ti.rate_of_change_bulk("price").unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "roc");
		assert!(!series.is_empty());
	}

	#[test]
	fn test_on_balance_volume_bulk() {
		let ti = create_momentum_ti();
		let result = ti.on_balance_volume_bulk("price", "volume", 0.0).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "obv");
		assert!(!series.is_empty());
	}

	#[test]
	fn test_commodity_channel_index_bulk() {
		let ti = create_momentum_ti();
		let result = ti.commodity_channel_index_bulk("price", "sma", "mad", 0.015, 20).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "cci");
		assert!(!series.is_empty());
	}

	#[test]
	fn test_mcginley_dynamic_commodity_channel_index_bulk() {
		let ti = create_momentum_ti();
		let result = ti.mcginley_dynamic_commodity_channel_index_bulk("price", 15.0, "mad", 0.015, 20).unwrap();
		let (cci_series, mcginley_series) = result;
		assert_eq!(cci_series.0.0.name(), "cci");
		assert_eq!(mcginley_series.0.0.name(), "mcginley_dynamic");
	}

	#[test]
	fn test_macd_line_bulk() {
		let ti = create_momentum_ti();
		let result = ti.macd_line_bulk("price", 12, "ema", 26, "ema").unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "macd");
		assert!(!series.is_empty());
	}

	#[test]
	fn test_signal_line_bulk() {
		let ti = create_momentum_ti();
		let macd_result = ti.macd_line_bulk("price", 12, "ema", 26, "ema").unwrap();
		let df = df! {
			"macd" => macd_result.0.0.f64().unwrap().into_iter().map(|v| v.unwrap_or(0.0)).collect::<Vec<f64>>()
		}
		.unwrap()
		.lazy();
		let ti_with_macd = MomentumTI::new(PyLfStubbed(pyo3_polars::PyLazyFrame(df)));
		let result = ti_with_macd.signal_line_bulk("macd", "ema", 9).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "signal");
	}

	#[test]
	fn test_mcginley_dynamic_macd_line_bulk() {
		let ti = create_momentum_ti();
		let result = ti.mcginley_dynamic_macd_line_bulk("price", 12, 15.0, 26, 18.0).unwrap();
		let df = result.0.0;
		let columns = df.get_column_names();
		let macd = PlSmallStr::from("macd");
		let short_mcginley = PlSmallStr::from("short_mcginley");
		let long_mcginley = PlSmallStr::from("long_mcginley");
		assert!(columns.contains(&&macd));
		assert!(columns.contains(&&short_mcginley));
		assert!(columns.contains(&&long_mcginley));
	}

	#[test]
	fn test_chaikin_oscillator_bulk() {
		let ti = create_momentum_ti();
		let result = ti.chaikin_oscillator_bulk("high", "low", "close", "volume", 3, 10, 0.0, "ema", "ema").unwrap();
		let (chaikin_series, ad_series) = result;
		assert_eq!(chaikin_series.0.0.name(), "chaikin_oscillator");
		assert_eq!(ad_series.0.0.name(), "accumulation_distribution");
	}

	#[test]
	fn test_percentage_price_oscillator_bulk() {
		let ti = create_momentum_ti();
		let result = ti.percentage_price_oscillator_bulk("price", 12, 26, "ema").unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "ppo");
		assert!(!series.is_empty());
	}

	#[test]
	fn test_chande_momentum_oscillator_bulk() {
		let ti = create_momentum_ti();
		let result = ti.chande_momentum_oscillator_bulk("price", 14).unwrap();
		let series = result.0.0;
		assert_eq!(series.name(), "chande_momentum_oscillator");
		assert!(!series.is_empty());
	}

	#[test]
	fn test_invalid_column_name() {
		let ti = create_momentum_ti();
		let result = ti.aroon_up_single("invalid_column");
		assert!(result.is_err());
	}

	#[test]
	fn test_aroon_oscillator_boundary_values() {
		let ti = create_momentum_ti();
		let result_max = ti.aroon_oscillator_single(100.0, 0.0).unwrap();
		let result_min = ti.aroon_oscillator_single(0.0, 100.0).unwrap();
		assert_abs_diff_eq!(result_max, 100.0, epsilon = 1e-10);
		assert_abs_diff_eq!(result_min, -100.0, epsilon = 1e-10);
	}

	#[test]
	fn test_parabolic_sar_acceleration_factor_zero() {
		let ti = create_momentum_ti();
		let result = ti.long_parabolic_time_price_system_single(10.0, 15.0, 0.0, 12.0).unwrap();
		assert_abs_diff_eq!(result, 10.0, epsilon = 1e-10);
	}

	#[test]
	fn test_volume_price_trend_no_change() {
		let ti = create_momentum_ti();
		let result = ti.volume_price_trend_single("price", 15.0, 1000.0, 100.0).unwrap();
		assert!(result.is_finite());
	}
}
