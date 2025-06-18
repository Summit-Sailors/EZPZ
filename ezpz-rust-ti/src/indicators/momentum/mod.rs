use {
	crate::utils::{create_triple_df, extract_f64_values, parse_constant_model_type, parse_deviation_model},
	ezpz_stubz::{frame::PyDfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

/// Momentum Technical Indicators - A collection of momentum analysis functions for financial data

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct MomentumTI;

#[gen_stub_pymethods]
#[pymethods]
impl MomentumTI {
	/// Aroon Up indicator
	///
	/// Calculates the Aroon Up indicator, which measures the time since the highest high
	/// within a given period as a percentage.
	///
	/// # Parameters
	/// * `highs` - PySeriesStubbed containing high price values
	///
	/// # Returns
	/// * `PyResult<f64>` - The Aroon Up value (0-100), where higher values indicate recent highs
	#[staticmethod]
	fn aroon_up_single(highs: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(highs)?;

		let result = rust_ti::trend_indicators::single::aroon_up(&values);
		Ok(result)
	}

	/// Aroon Down indicator
	///
	/// Calculates the Aroon Down indicator, which measures the time since the lowest low
	/// within a given period as a percentage.
	///
	/// # Parameters
	/// * `lows` - PySeriesStubbed containing low price values
	///
	/// # Returns
	/// * `PyResult<f64>` - The Aroon Down value (0-100), where higher values indicate recent lows
	#[staticmethod]
	fn aroon_down_single(lows: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(lows)?;

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
	#[staticmethod]
	fn aroon_oscillator_single(aroon_up: f64, aroon_down: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::aroon_oscillator(&aroon_up, &aroon_down);
		Ok(result)
	}

	/// Aroon Indicator (complete calculation)
	///
	/// Calculates all three Aroon components: Aroon Up, Aroon Down, and Aroon Oscillator
	/// in a single function call.
	///
	/// # Parameters
	/// * `highs` - PySeriesStubbed containing high price values
	/// * `lows` - PySeriesStubbed containing low price values
	///
	/// # Returns
	/// * `PyResult<(f64, f64, f64)>` - Tuple containing (aroon_up, aroon_down, aroon_oscillator)
	#[staticmethod]
	fn aroon_indicator_single(highs: PySeriesStubbed, lows: PySeriesStubbed) -> PyResult<(f64, f64, f64)> {
		let highs_values: Vec<f64> = extract_f64_values(highs)?;
		let lows_values: Vec<f64> = extract_f64_values(lows)?;

		let result = rust_ti::trend_indicators::single::aroon_indicator(&highs_values, &lows_values);
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
	#[staticmethod]
	fn long_parabolic_time_price_system_single(previous_sar: f64, extreme_point: f64, acceleration_factor: f64, low: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::long_parabolic_time_price_system(&previous_sar, &extreme_point, &acceleration_factor, &low);
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
	#[staticmethod]
	fn short_parabolic_time_price_system_single(previous_sar: f64, extreme_point: f64, acceleration_factor: f64, high: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::short_parabolic_time_price_system(&previous_sar, &extreme_point, &acceleration_factor, &high);
		Ok(result)
	}

	/// Volume Price Trend
	///
	/// Calculates the Volume Price Trend indicator, which combines price and volume
	/// to show the relationship between volume and price changes.
	///
	/// # Parameters
	/// * `current_price` - f64 current period's price
	/// * `previous_price` - f64 previous period's price
	/// * `volume` - f64 current period's volume
	/// * `previous_volume_price_trend` - f64 previous VPT value
	///
	/// # Returns
	/// * `PyResult<f64>` - The calculated Volume Price Trend value
	#[staticmethod]
	fn volume_price_trend_single(current_price: f64, previous_price: f64, volume: f64, previous_volume_price_trend: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::volume_price_trend(&current_price, &previous_price, &volume, &previous_volume_price_trend);
		Ok(result)
	}

	/// True Strength Index
	///
	/// Calculates the True Strength Index, a momentum oscillator that uses price changes
	/// smoothed by two exponential moving averages.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing price values
	/// * `first_constant_model` - &str smoothing model for first smoothing ("sma", "ema", etc.)
	/// * `first_period` - usize period for first smoothing
	/// * `second_constant_model` - &str smoothing model for second smoothing ("sma", "ema", etc.)
	///
	/// # Returns
	/// * `PyResult<f64>` - The True Strength Index value (typically ranges from -100 to +100)
	#[staticmethod]
	fn true_strength_index_single(prices: PySeriesStubbed, first_constant_model: &str, first_period: usize, second_constant_model: &str) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		// Convert string parameters to ConstantModelType enums
		let first_model = parse_constant_model_type(first_constant_model)?;

		let second_model = parse_constant_model_type(second_constant_model)?;

		let result = rust_ti::trend_indicators::single::true_strength_index(&values, &first_model, &first_period, &second_model);
		Ok(result)
	}

	/// Relative Strength Index (RSI) - bulk calculation
	///
	/// Calculates RSI values for an entire series of prices. RSI measures the speed and change
	/// of price movements, oscillating between 0 and 100.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing price values
	/// * `constant_model_type` - &str smoothing model ("sma", "ema", etc.)
	/// * `period` - usize calculation period (commonly 14)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "rsi" containing RSI values (0-100)
	#[staticmethod]
	fn relative_strength_index_bulk(prices: PySeriesStubbed, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let model_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::momentum_indicators::bulk::relative_strength_index(&values, &model_type, &period);
		let series = Series::new("rsi".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Stochastic Oscillator - bulk calculation
	///
	/// Calculates the Stochastic Oscillator, which compares a security's closing price
	/// to its price range over a given time period.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing price values
	/// * `period` - usize lookback period for calculation
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "stochastic" containing oscillator values (0-100)
	#[staticmethod]
	fn stochastic_oscillator_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result = rust_ti::momentum_indicators::bulk::stochastic_oscillator(&values, &period);
		let series = Series::new("stochastic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Slow Stochastic - bulk calculation
	///
	/// Calculates the Slow Stochastic by smoothing the regular Stochastic Oscillator
	/// to reduce noise and false signals.
	///
	/// # Parameters
	/// * `stochastics` - PySeriesStubbed containing Stochastic Oscillator values
	/// * `constant_model_type` - &str smoothing model ("sma", "ema", etc.)
	/// * `period` - usize smoothing period
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "slow_stochastic" containing smoothed values (0-100)
	#[staticmethod]
	fn slow_stochastic_bulk(stochastics: PySeriesStubbed, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(stochastics)?;

		let model_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::momentum_indicators::bulk::slow_stochastic(&values, &model_type, &period);
		let series = Series::new("slow_stochastic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Slowest Stochastic - bulk calculation
	///
	/// Calculates the Slowest Stochastic by applying additional smoothing to the Slow Stochastic
	/// for even more noise reduction.
	///
	/// # Parameters
	/// * `slow_stochastics` - PySeriesStubbed containing Slow Stochastic values
	/// * `constant_model_type` - &str smoothing model ("sma", "ema", etc.)
	/// * `period` - usize smoothing period
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "slowest_stochastic" containing double-smoothed values (0-100)
	#[staticmethod]
	fn slowest_stochastic_bulk(slow_stochastics: PySeriesStubbed, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(slow_stochastics)?;

		let model_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::momentum_indicators::bulk::slowest_stochastic(&values, &model_type, &period);
		let series = Series::new("slowest_stochastic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Williams %R - bulk calculation
	///
	/// Calculates Williams %R, a momentum indicator that measures overbought and oversold levels.
	/// Values range from -100 to 0, where -20 and above indicates overbought, -80 and below indicates oversold.
	///
	/// # Parameters
	/// * `high` - PySeriesStubbed containing high price values
	/// * `low` - PySeriesStubbed containing low price values
	/// * `close` - PySeriesStubbed containing closing price values
	/// * `period` - usize lookback period for calculation
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "williams_r" containing Williams %R values (-100 to 0)
	#[staticmethod]
	fn williams_percent_r_bulk(high: PySeriesStubbed, low: PySeriesStubbed, close: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;

		let result = rust_ti::momentum_indicators::bulk::williams_percent_r(&high_values, &low_values, &close_values, &period);
		let series = Series::new("williams_r".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Money Flow Index - bulk calculation
	///
	/// Calculates the Money Flow Index, a volume-weighted RSI that measures buying and selling pressure.
	/// Values range from 0 to 100, where >80 indicates overbought and <20 indicates oversold.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing typical price values ((high + low + close) / 3)
	/// * `volume` - PySeriesStubbed containing volume values
	/// * `period` - usize calculation period (commonly 14)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "mfi" containing Money Flow Index values (0-100)
	#[staticmethod]
	fn money_flow_index_bulk(prices: PySeriesStubbed, volume: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let price_values: Vec<f64> = extract_f64_values(prices)?;
		let volume_values: Vec<f64> = extract_f64_values(volume)?;

		let result = rust_ti::momentum_indicators::bulk::money_flow_index(&price_values, &volume_values, &period);
		let series = Series::new("mfi".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Rate of Change - bulk calculation
	///
	/// Calculates the Rate of Change, which measures the percentage change in price
	/// from one period to the next.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing price values
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "roc" containing rate of change values as percentages
	#[staticmethod]
	fn rate_of_change_bulk(prices: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result = rust_ti::momentum_indicators::bulk::rate_of_change(&values);
		let series = Series::new("roc".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// On Balance Volume - bulk calculation
	///
	/// Calculates On Balance Volume, a cumulative volume indicator that adds volume on up days
	/// and subtracts volume on down days to measure buying and selling pressure.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing price values
	/// * `volume` - PySeriesStubbed containing volume values
	/// * `previous_obv` - f64 starting OBV value (typically 0)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "obv" containing cumulative OBV values
	#[staticmethod]
	fn on_balance_volume_bulk(prices: PySeriesStubbed, volume: PySeriesStubbed, previous_obv: f64) -> PyResult<PySeriesStubbed> {
		let price_values: Vec<f64> = extract_f64_values(prices)?;
		let volume_values: Vec<f64> = extract_f64_values(volume)?;

		let result = rust_ti::momentum_indicators::bulk::on_balance_volume(&price_values, &volume_values, &previous_obv);
		let series = Series::new("obv".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Commodity Channel Index - bulk calculation
	///
	/// Calculates the Commodity Channel Index, which measures the variation of a security's price
	/// from its statistical mean. Values typically range from -100 to +100.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing typical price values
	/// * `constant_model_type` - &str model for calculating moving average ("sma", "ema", etc.)
	/// * `deviation_model` - &str model for calculating deviation ("mad", "std", etc.)
	/// * `constant_multiplier` - f64 multiplier constant (typically 0.015)
	/// * `period` - usize calculation period (commonly 20)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "cci" containing CCI values
	#[staticmethod]
	fn commodity_channel_index_bulk(
		prices: PySeriesStubbed,
		constant_model_type: &str,
		deviation_model: &str,
		constant_multiplier: f64,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let model_type = parse_constant_model_type(constant_model_type)?;

		let dev_model = parse_deviation_model(deviation_model)?;

		let result = rust_ti::momentum_indicators::bulk::commodity_channel_index(&values, &model_type, &dev_model, &constant_multiplier, &period);
		let series = Series::new("cci".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// McGinley Dynamic Commodity Channel Index - bulk calculation
	///
	/// Calculates CCI using McGinley Dynamic as the moving average, which adapts to market conditions
	/// better than traditional moving averages.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing typical price values
	/// * `previous_mcginley_dynamic` - f64 initial McGinley Dynamic value
	/// * `deviation_model` - &str model for calculating deviation ("mad", "std", etc.)
	/// * `constant_multiplier` - f64 multiplier constant (typically 0.015)
	/// * `period` - usize calculation period
	///
	/// # Returns
	/// * `PyResult<(PySeriesStubbed, PySeriesStubbed)>` - Tuple containing (CCI series, McGinley Dynamic series)
	#[staticmethod]
	fn mcginley_dynamic_commodity_channel_index_bulk(
		prices: PySeriesStubbed,
		previous_mcginley_dynamic: f64,
		deviation_model: &str,
		constant_multiplier: f64,
		period: usize,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let dev_model = parse_deviation_model(deviation_model)?;

		let result = rust_ti::momentum_indicators::bulk::mcginley_dynamic_commodity_channel_index(
			&values,
			&previous_mcginley_dynamic,
			&dev_model,
			&constant_multiplier,
			&period,
		);

		let (cci_values, mcginley_values): (Vec<f64>, Vec<f64>) = result.into_iter().unzip();

		let cci_series = Series::new("cci".into(), cci_values);
		let mcginley_series = Series::new("mcginley_dynamic".into(), mcginley_values);

		Ok((PySeriesStubbed(pyo3_polars::PySeries(cci_series)), PySeriesStubbed(pyo3_polars::PySeries(mcginley_series))))
	}

	/// MACD Line - bulk calculation
	///
	/// Calculates the MACD (Moving Average Convergence Divergence) line by subtracting
	/// the long-period moving average from the short-period moving average.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing price values
	/// * `short_period` - usize period for short moving average (commonly 12)
	/// * `short_period_model` - &str model for short MA ("sma", "ema", etc.)
	/// * `long_period` - usize period for long moving average (commonly 26)
	/// * `long_period_model` - &str model for long MA ("sma", "ema", etc.)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "macd" containing MACD line values
	#[staticmethod]
	fn macd_line_bulk(
		prices: PySeriesStubbed,
		short_period: usize,
		short_period_model: &str,
		long_period: usize,
		long_period_model: &str,
	) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let short_model = parse_constant_model_type(short_period_model)?;
		let long_model = parse_constant_model_type(long_period_model)?;

		let result = rust_ti::momentum_indicators::bulk::macd_line(&values, &short_period, &short_model, &long_period, &long_model);
		let series = Series::new("macd".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Signal Line - bulk calculation
	///
	/// Calculates the MACD Signal Line by applying a moving average to the MACD line.
	/// Used to generate buy/sell signals when MACD crosses above or below the signal line.
	///
	/// # Parameters
	/// * `macds` - PySeriesStubbed containing MACD line values
	/// * `constant_model_type` - &str smoothing model ("sma", "ema", etc.)
	/// * `period` - usize signal line period (commonly 9)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "signal" containing signal line values
	#[staticmethod]
	fn signal_line_bulk(macds: PySeriesStubbed, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(macds)?;

		let model_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::momentum_indicators::bulk::signal_line(&values, &model_type, &period);
		let series = Series::new("signal".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// McGinley Dynamic MACD Line - bulk calculation
	///
	/// Calculates MACD using McGinley Dynamic moving averages instead of traditional MAs,
	/// providing better adaptation to market volatility and reducing lag.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing price values
	/// * `short_period` - usize period for short McGinley Dynamic
	/// * `previous_short_mcginley` - f64 initial short McGinley Dynamic value
	/// * `long_period` - usize period for long McGinley Dynamic
	/// * `previous_long_mcginley` - f64 initial long McGinley Dynamic value
	///
	/// # Returns
	/// * `PyResult<PyDfStubbed>` - DataFrame with columns: "macd", "short_mcginley", "long_mcginley"
	#[staticmethod]
	fn mcginley_dynamic_macd_line_bulk(
		prices: PySeriesStubbed,
		short_period: usize,
		previous_short_mcginley: f64,
		long_period: usize,
		previous_long_mcginley: f64,
	) -> PyResult<PyDfStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result =
			rust_ti::momentum_indicators::bulk::mcginley_dynamic_macd_line(&values, &short_period, &previous_short_mcginley, &long_period, &previous_long_mcginley);

		let (macd_values, short_mcginley_values, long_mcginley_values): (Vec<f64>, Vec<f64>, Vec<f64>) =
			result.into_iter().fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, (a, b, c)| {
				acc.0.push(a);
				acc.1.push(b);
				acc.2.push(c);
				acc
			});

		create_triple_df(macd_values, short_mcginley_values, long_mcginley_values, "macd", "short_mcginley", "long_mcginley")
	}

	/// Chaikin Oscillator - bulk calculation
	///
	/// Calculates the Chaikin Oscillator, which applies MACD to the Accumulation/Distribution line
	/// to measure the momentum of the Accumulation/Distribution line.
	///
	/// # Parameters
	/// * `highs` - PySeriesStubbed containing high price values
	/// * `lows` - PySeriesStubbed containing low price values
	/// * `close` - PySeriesStubbed containing closing price values
	/// * `volume` - PySeriesStubbed containing volume values
	/// * `short_period` - usize short period for oscillator (commonly 3)
	/// * `long_period` - usize long period for oscillator (commonly 10)
	/// * `previous_accumulation_distribution` - f64 initial A/D line value
	/// * `short_period_model` - &str model for short MA ("sma", "ema", etc.)
	/// * `long_period_model` - &str model for long MA ("sma", "ema", etc.)
	///
	/// # Returns
	/// * `PyResult<(PySeriesStubbed, PySeriesStubbed)>` - Tuple containing (Chaikin Oscillator, A/D Line)
	#[staticmethod]
	fn chaikin_oscillator_bulk(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		close: PySeriesStubbed,
		volume: PySeriesStubbed,
		short_period: usize,
		long_period: usize,
		previous_accumulation_distribution: f64,
		short_period_model: &str,
		long_period_model: &str,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
		let high_values: Vec<f64> = extract_f64_values(highs)?;
		let low_values: Vec<f64> = extract_f64_values(lows)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let volume_values: Vec<f64> = extract_f64_values(volume)?;

		let short_model = parse_constant_model_type(short_period_model)?;

		let long_model = parse_constant_model_type(long_period_model)?;

		let result = rust_ti::momentum_indicators::bulk::chaikin_oscillator(
			&high_values,
			&low_values,
			&close_values,
			&volume_values,
			&short_period,
			&long_period,
			&previous_accumulation_distribution,
			&short_model,
			&long_model,
		);

		let (chaikin_values, ad_values): (Vec<f64>, Vec<f64>) = result.into_iter().unzip();

		let chaikin_series = Series::new("chaikin_oscillator".into(), chaikin_values);
		let ad_series = Series::new("accumulation_distribution".into(), ad_values);

		Ok((PySeriesStubbed(pyo3_polars::PySeries(chaikin_series)), PySeriesStubbed(pyo3_polars::PySeries(ad_series))))
	}

	/// Percentage Price Oscillator - bulk calculation
	///
	/// Calculates the Percentage Price Oscillator, which is similar to MACD but expressed as a percentage.
	/// This makes it easier to compare securities with different price levels.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing price values
	/// * `short_period` - usize short period for moving average (commonly 12)
	/// * `long_period` - usize long period for moving average (commonly 26)
	/// * `constant_model_type` - &str model for moving averages ("sma", "ema", etc.)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "ppo" containing PPO values as percentages
	#[staticmethod]
	fn percentage_price_oscillator_bulk(
		prices: PySeriesStubbed,
		short_period: usize,
		long_period: usize,
		constant_model_type: &str,
	) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let model_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::momentum_indicators::bulk::percentage_price_oscillator(&values, &short_period, &long_period, &model_type);
		let series = Series::new("ppo".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Chande Momentum Oscillator - bulk calculation
	///
	/// Calculates the Chande Momentum Oscillator, which measures momentum by calculating
	/// the difference between the sum of gains and losses over a given period.
	/// Values range from -100 to +100.
	///
	/// # Parameters
	/// * `prices` - PySeriesStubbed containing price values
	/// * `period` - usize calculation period (commonly 14 or 20)
	///
	/// # Returns
	/// * `PyResult<PySeriesStubbed>` - Series named "chande_momentum_oscillator" containing CMO values (-100 to +100)
	#[staticmethod]
	fn chande_momentum_oscillator_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let result = rust_ti::momentum_indicators::bulk::chande_momentum_oscillator(&values, &period);
		let series = Series::new("chande_momentum_oscillator".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}
}
