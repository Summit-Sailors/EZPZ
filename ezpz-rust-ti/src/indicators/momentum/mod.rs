use {
	crate::utils::{parse_constant_model_type, parse_deviation_model},
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct MomentumTI;

#[gen_stub_pymethods]
#[pymethods]
impl MomentumTI {
	/// Aroon Up indicator
	#[staticmethod]
	fn aroon_up_single(highs: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = highs.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::trend_indicators::single::aroon_up(&values);
		Ok(result)
	}

	/// Aroon Down indicator
	#[staticmethod]
	fn aroon_down_single(lows: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = lows.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::trend_indicators::single::aroon_down(&values);
		Ok(result)
	}

	/// Aroon Oscillator
	#[staticmethod]
	fn aroon_oscillator_single(aroon_up: f64, aroon_down: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::aroon_oscillator(&aroon_up, &aroon_down);
		Ok(result)
	}

	/// Aroon Indicator (returns tuple of aroon_up, aroon_down, aroon_oscillator)
	#[staticmethod]
	fn aroon_indicator_single(highs: PySeriesStubbed, lows: PySeriesStubbed) -> PyResult<(f64, f64, f64)> {
		let highs_series: Series = highs.0.into();
		let highs_values: Vec<f64> = highs_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let lows_series: Series = lows.0.into();
		let lows_values: Vec<f64> = lows_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::trend_indicators::single::aroon_indicator(&highs_values, &lows_values);
		Ok(result)
	}

	/// Long Parabolic Time Price System (Parabolic SAR for long positions)
	#[staticmethod]
	fn long_parabolic_time_price_system_single(previous_sar: f64, extreme_point: f64, acceleration_factor: f64, low: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::long_parabolic_time_price_system(&previous_sar, &extreme_point, &acceleration_factor, &low);
		Ok(result)
	}

	/// Short Parabolic Time Price System (Parabolic SAR for short positions)
	#[staticmethod]
	fn short_parabolic_time_price_system_single(previous_sar: f64, extreme_point: f64, acceleration_factor: f64, high: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::short_parabolic_time_price_system(&previous_sar, &extreme_point, &acceleration_factor, &high);
		Ok(result)
	}

	/// Volume Price Trend
	#[staticmethod]
	fn volume_price_trend_single(current_price: f64, previous_price: f64, volume: f64, previous_volume_price_trend: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::volume_price_trend(&current_price, &previous_price, &volume, &previous_volume_price_trend);
		Ok(result)
	}

	/// True Strength Index
	#[staticmethod]
	fn true_strength_index_single(prices: PySeriesStubbed, first_constant_model: &str, first_period: usize, second_constant_model: &str) -> PyResult<f64> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		// Convert string parameters to ConstantModelType enums
		let first_model = parse_constant_model_type(first_constant_model)?;

		let second_model = parse_constant_model_type(second_constant_model)?;

		let result = rust_ti::trend_indicators::single::true_strength_index(&values, &first_model, &first_period, &second_model);
		Ok(result)
	}

	/// Bulk calculations
	/// Relative Strength Index (RSI) - bulk calculation
	#[staticmethod]
	fn relative_strength_index_bulk(prices: PySeriesStubbed, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let model_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::momentum_indicators::bulk::relative_strength_index(&values, &model_type, &period);
		let series = Series::new("rsi".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Stochastic Oscillator - bulk calculation
	#[staticmethod]
	fn stochastic_oscillator_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::momentum_indicators::bulk::stochastic_oscillator(&values, &period);
		let series = Series::new("stochastic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Slow Stochastic - bulk calculation
	#[staticmethod]
	fn slow_stochastic_bulk(stochastics: PySeriesStubbed, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = stochastics.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let model_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::momentum_indicators::bulk::slow_stochastic(&values, &model_type, &period);
		let series = Series::new("slow_stochastic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Slowest Stochastic - bulk calculation
	#[staticmethod]
	fn slowest_stochastic_bulk(slow_stochastics: PySeriesStubbed, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = slow_stochastics.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let model_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::momentum_indicators::bulk::slowest_stochastic(&values, &model_type, &period);
		let series = Series::new("slowest_stochastic".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Williams %R - bulk calculation
	#[staticmethod]
	fn williams_percent_r_bulk(high: PySeriesStubbed, low: PySeriesStubbed, close: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let high_series: Series = high.0.into();
		let low_series: Series = low.0.into();
		let close_series: Series = close.0.into();

		let high_values: Vec<f64> = high_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let low_values: Vec<f64> = low_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let close_values: Vec<f64> = close_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::momentum_indicators::bulk::williams_percent_r(&high_values, &low_values, &close_values, &period);
		let series = Series::new("williams_r".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Money Flow Index - bulk calculation
	#[staticmethod]
	fn money_flow_index_bulk(prices: PySeriesStubbed, volume: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let price_series: Series = prices.0.into();
		let volume_series: Series = volume.0.into();

		let price_values: Vec<f64> = price_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let volume_values: Vec<f64> = volume_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::momentum_indicators::bulk::money_flow_index(&price_values, &volume_values, &period);
		let series = Series::new("mfi".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Rate of Change - bulk calculation
	#[staticmethod]
	fn rate_of_change_bulk(prices: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::momentum_indicators::bulk::rate_of_change(&values);
		let series = Series::new("roc".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// On Balance Volume - bulk calculation
	#[staticmethod]
	fn on_balance_volume_bulk(prices: PySeriesStubbed, volume: PySeriesStubbed, previous_obv: f64) -> PyResult<PySeriesStubbed> {
		let price_series: Series = prices.0.into();
		let volume_series: Series = volume.0.into();

		let price_values: Vec<f64> = price_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let volume_values: Vec<f64> = volume_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::momentum_indicators::bulk::on_balance_volume(&price_values, &volume_values, &previous_obv);
		let series = Series::new("obv".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Commodity Channel Index - bulk calculation
	#[staticmethod]
	fn commodity_channel_index_bulk(
		prices: PySeriesStubbed,
		constant_model_type: &str,
		deviation_model: &str,
		constant_multiplier: f64,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let model_type = parse_constant_model_type(constant_model_type)?;

		let dev_model = parse_deviation_model(deviation_model)?;

		let result = rust_ti::momentum_indicators::bulk::commodity_channel_index(&values, &model_type, &dev_model, &constant_multiplier, &period);
		let series = Series::new("cci".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// McGinley Dynamic Commodity Channel Index - bulk calculation
	/// Returns a tuple series with (CCI, McGinley Dynamic)
	#[staticmethod]
	fn mcginley_dynamic_commodity_channel_index_bulk(
		prices: PySeriesStubbed,
		previous_mcginley_dynamic: f64,
		deviation_model: &str,
		constant_multiplier: f64,
		period: usize,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

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
	#[staticmethod]
	fn macd_line_bulk(
		prices: PySeriesStubbed,
		short_period: usize,
		short_period_model: &str,
		long_period: usize,
		long_period_model: &str,
	) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let short_model = parse_constant_model_type(short_period_model)?;

		let long_model = parse_constant_model_type(long_period_model)?;

		let result = rust_ti::momentum_indicators::bulk::macd_line(&values, &short_period, &short_model, &long_period, &long_model);
		let series = Series::new("macd".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Signal Line - bulk calculation
	#[staticmethod]
	fn signal_line_bulk(macds: PySeriesStubbed, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = macds.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let model_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::momentum_indicators::bulk::signal_line(&values, &model_type, &period);
		let series = Series::new("signal".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// McGinley Dynamic MACD Line - bulk calculation
	/// Returns a tuple with (MACD, Short McGinley Dynamic, Long McGinley Dynamic)
	#[staticmethod]
	fn mcginley_dynamic_macd_line_bulk(
		prices: PySeriesStubbed,
		short_period: usize,
		previous_short_mcginley: f64,
		long_period: usize,
		previous_long_mcginley: f64,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result =
			rust_ti::momentum_indicators::bulk::mcginley_dynamic_macd_line(&values, &short_period, &previous_short_mcginley, &long_period, &previous_long_mcginley);

		let (macd_values, short_mcginley_values, long_mcginley_values): (Vec<f64>, Vec<f64>, Vec<f64>) =
			result.into_iter().fold((Vec::new(), Vec::new(), Vec::new()), |mut acc, (a, b, c)| {
				acc.0.push(a);
				acc.1.push(b);
				acc.2.push(c);
				acc
			});

		let macd_series = Series::new("macd".into(), macd_values);
		let short_mcginley_series = Series::new("short_mcginley".into(), short_mcginley_values);
		let long_mcginley_series = Series::new("long_mcginley".into(), long_mcginley_values);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(macd_series)),
			PySeriesStubbed(pyo3_polars::PySeries(short_mcginley_series)),
			PySeriesStubbed(pyo3_polars::PySeries(long_mcginley_series)),
		))
	}

	/// Chaikin Oscillator - bulk calculation
	/// Returns a tuple with (Chaikin Oscillator, Accumulation Distribution)
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
		let high_series: Series = highs.0.into();
		let low_series: Series = lows.0.into();
		let close_series: Series = close.0.into();
		let volume_series: Series = volume.0.into();

		let high_values: Vec<f64> = high_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let low_values: Vec<f64> = low_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let close_values: Vec<f64> = close_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let volume_values: Vec<f64> = volume_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

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
	#[staticmethod]
	fn percentage_price_oscillator_bulk(
		prices: PySeriesStubbed,
		short_period: usize,
		long_period: usize,
		constant_model_type: &str,
	) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let model_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::momentum_indicators::bulk::percentage_price_oscillator(&values, &short_period, &long_period, &model_type);
		let series = Series::new("ppo".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}

	/// Chande Momentum Oscillator - bulk calculation
	#[staticmethod]
	fn chande_momentum_oscillator_bulk(prices: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::momentum_indicators::bulk::chande_momentum_oscillator(&values, &period);
		let series = Series::new("chande_momentum_oscillator".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(series)))
	}
}
