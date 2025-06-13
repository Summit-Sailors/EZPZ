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
pub struct CandleTI;

#[gen_stub_pymethods]
#[pymethods]
impl CandleTI {
	/// Moving Constant Envelopes - Creates upper and lower bands from moving constant of price
	/// Returns tuple of (lower_band, moving_constant, upper_band)
	#[staticmethod]
	fn moving_constant_envelopes(
		prices: PySeriesStubbed,
		constant_model_type: &str,
		difference: f64,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::candle_indicators::single::moving_constant_envelopes(&values, &constant_type, &difference);

		let lower_series = Series::new("lower_envelope".into(), vec![result.0]);
		let middle_series = Series::new("middle_envelope".into(), vec![result.1]);
		let upper_series = Series::new("upper_envelope".into(), vec![result.2]);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	/// McGinley Dynamic Envelopes - Variation of moving constant envelopes using McGinley Dynamic
	#[staticmethod]
	fn mcginley_dynamic_envelopes(
		prices: PySeriesStubbed,
		difference: f64,
		previous_mcginley_dynamic: f64,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let result = rust_ti::candle_indicators::single::mcginley_dynamic_envelopes(&values, &difference, &previous_mcginley_dynamic);

		let lower_series = Series::new("lower_envelope".into(), vec![result.0]);
		let middle_series = Series::new("mcginley_dynamic".into(), vec![result.1]);
		let upper_series = Series::new("upper_envelope".into(), vec![result.2]);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	/// Moving Constant Bands - Extended Bollinger Bands with configurable models
	#[staticmethod]
	fn moving_constant_bands(
		prices: PySeriesStubbed,
		constant_model_type: &str,
		deviation_model: &str,
		deviation_multiplier: f64,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;

		let result = rust_ti::candle_indicators::single::moving_constant_bands(&values, &constant_type, &deviation_type, &deviation_multiplier);

		let lower_series = Series::new("lower_band".into(), vec![result.0]);
		let middle_series = Series::new("middle_band".into(), vec![result.1]);
		let upper_series = Series::new("upper_band".into(), vec![result.2]);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	/// McGinley Dynamic Bands - Variation of moving constant bands using McGinley Dynamic
	#[staticmethod]
	fn mcginley_dynamic_bands(
		prices: PySeriesStubbed,
		deviation_model: &str,
		deviation_multiplier: f64,
		previous_mcginley_dynamic: f64,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let deviation_type = parse_deviation_model(deviation_model)?;

		let result = rust_ti::candle_indicators::single::mcginley_dynamic_bands(&values, &deviation_type, &deviation_multiplier, &previous_mcginley_dynamic);

		let lower_series = Series::new("lower_band".into(), vec![result.0]);
		let middle_series = Series::new("mcginley_dynamic".into(), vec![result.1]);
		let upper_series = Series::new("upper_band".into(), vec![result.2]);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	/// Ichimoku Cloud - Calculates support and resistance levels
	/// Returns (leading_span_a, leading_span_b, base_line, conversion_line, lagged_price)
	#[staticmethod]
	fn ichimoku_cloud(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		close: PySeriesStubbed,
		conversion_period: usize,
		base_period: usize,
		span_b_period: usize,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let high_series: Series = highs.0.into();
		let low_series: Series = lows.0.into();
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

		let result = rust_ti::candle_indicators::single::ichimoku_cloud(&high_values, &low_values, &close_values, &conversion_period, &base_period, &span_b_period);

		let leading_span_a = Series::new("leading_span_a".into(), vec![result.0]);
		let leading_span_b = Series::new("leading_span_b".into(), vec![result.1]);
		let base_line = Series::new("base_line".into(), vec![result.2]);
		let conversion_line = Series::new("conversion_line".into(), vec![result.3]);
		let lagged_price = Series::new("lagged_price".into(), vec![result.4]);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(leading_span_a)),
			PySeriesStubbed(pyo3_polars::PySeries(leading_span_b)),
			PySeriesStubbed(pyo3_polars::PySeries(base_line)),
			PySeriesStubbed(pyo3_polars::PySeries(conversion_line)),
			PySeriesStubbed(pyo3_polars::PySeries(lagged_price)),
		))
	}

	/// Donchian Channels - Produces bands from period highs and lows
	#[staticmethod]
	fn donchian_channels(highs: PySeriesStubbed, lows: PySeriesStubbed) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let high_series: Series = highs.0.into();
		let low_series: Series = lows.0.into();

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

		let result = rust_ti::candle_indicators::single::donchian_channels(&high_values, &low_values);

		let lower_series = Series::new("donchian_lower".into(), vec![result.0]);
		let middle_series = Series::new("donchian_middle".into(), vec![result.1]);
		let upper_series = Series::new("donchian_upper".into(), vec![result.2]);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	/// Keltner Channel - Bands based on moving average and average true range
	#[staticmethod]
	fn keltner_channel(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		close: PySeriesStubbed,
		constant_model_type: &str,
		atr_constant_model_type: &str,
		multiplier: f64,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let high_series: Series = highs.0.into();
		let low_series: Series = lows.0.into();
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

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let atr_constant_type = parse_constant_model_type(atr_constant_model_type)?;

		let result = rust_ti::candle_indicators::single::keltner_channel(&high_values, &low_values, &close_values, &constant_type, &atr_constant_type, &multiplier);

		let lower_series = Series::new("keltner_lower".into(), vec![result.0]);
		let middle_series = Series::new("keltner_middle".into(), vec![result.1]);
		let upper_series = Series::new("keltner_upper".into(), vec![result.2]);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	/// Supertrend - Trend indicator showing support and resistance levels
	#[staticmethod]
	fn supertrend(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		close: PySeriesStubbed,
		constant_model_type: &str,
		multiplier: f64,
	) -> PyResult<PySeriesStubbed> {
		let high_series: Series = highs.0.into();
		let low_series: Series = lows.0.into();
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

		let constant_type = parse_constant_model_type(constant_model_type)?;

		let result = rust_ti::candle_indicators::single::supertrend(&high_values, &low_values, &close_values, &constant_type, &multiplier);

		let result_series = Series::new("supertrend".into(), vec![result]);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	// Bulk functions that return multiple values over time

	/// Moving Constant Envelopes (Bulk) - Returns envelopes over time periods
	#[staticmethod]
	fn moving_constant_envelopes_bulk(
		prices: PySeriesStubbed,
		constant_model_type: &str,
		difference: f64,
		period: usize,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let results = rust_ti::candle_indicators::bulk::moving_constant_envelopes(&values, &constant_type, &difference, &period);

		let (lower_vals, middle_vals, upper_vals) = {
			let mut lower = Vec::new();
			let mut middle = Vec::new();
			let mut upper = Vec::new();
			for (l, m, u) in results {
				lower.push(l);
				middle.push(m);
				upper.push(u);
			}
			(lower, middle, upper)
		};

		let lower_series = Series::new("lower_envelope".into(), lower_vals);
		let middle_series = Series::new("middle_envelope".into(), middle_vals);
		let upper_series = Series::new("upper_envelope".into(), upper_vals);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	/// McGinley Dynamic Envelopes (Bulk)
	#[staticmethod]
	fn mcginley_dynamic_envelopes_bulk(
		prices: PySeriesStubbed,
		difference: f64,
		previous_mcginley_dynamic: f64,
		period: usize,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let results = rust_ti::candle_indicators::bulk::mcginley_dynamic_envelopes(&values, &difference, &previous_mcginley_dynamic, &period);

		let (lower_vals, middle_vals, upper_vals) = {
			let mut lower = Vec::new();
			let mut middle = Vec::new();
			let mut upper = Vec::new();
			for (l, m, u) in results {
				lower.push(l);
				middle.push(m);
				upper.push(u);
			}
			(lower, middle, upper)
		};

		let lower_series = Series::new("lower_envelope".into(), lower_vals);
		let middle_series = Series::new("mcginley_dynamic".into(), middle_vals);
		let upper_series = Series::new("upper_envelope".into(), upper_vals);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	/// Moving Constant Bands (Bulk)
	#[staticmethod]
	fn moving_constant_bands_bulk(
		prices: PySeriesStubbed,
		constant_model_type: &str,
		deviation_model: &str,
		deviation_multiplier: f64,
		period: usize,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let deviation_type = parse_deviation_model(deviation_model)?;

		let results = rust_ti::candle_indicators::bulk::moving_constant_bands(&values, &constant_type, &deviation_type, &deviation_multiplier, &period);

		let (lower_vals, middle_vals, upper_vals) = {
			let mut lower = Vec::new();
			let mut middle = Vec::new();
			let mut upper = Vec::new();
			for (l, m, u) in results {
				lower.push(l);
				middle.push(m);
				upper.push(u);
			}
			(lower, middle, upper)
		};
		let lower_series = Series::new("lower_band".into(), lower_vals);
		let middle_series = Series::new("middle_band".into(), middle_vals);
		let upper_series = Series::new("upper_band".into(), upper_vals);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	/// McGinley Dynamic Bands (Bulk)
	#[staticmethod]
	fn mcginley_dynamic_bands_bulk(
		prices: PySeriesStubbed,
		deviation_model: &str,
		deviation_multiplier: f64,
		previous_mcginley_dynamic: f64,
		period: usize,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = prices.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		let deviation_type = parse_deviation_model(deviation_model)?;

		let results =
			rust_ti::candle_indicators::bulk::mcginley_dynamic_bands(&values, &deviation_type, &deviation_multiplier, &previous_mcginley_dynamic, &period);

		let (lower_vals, middle_vals, upper_vals) = {
			let mut lower = Vec::new();
			let mut middle = Vec::new();
			let mut upper = Vec::new();
			for (l, m, u) in results {
				lower.push(l);
				middle.push(m);
				upper.push(u);
			}
			(lower, middle, upper)
		};

		let lower_series = Series::new("lower_band".into(), lower_vals);
		let middle_series = Series::new("mcginley_dynamic".into(), middle_vals);
		let upper_series = Series::new("upper_band".into(), upper_vals);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	#[staticmethod]
	fn ichimoku_cloud_bulk(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		closes: PySeriesStubbed,
		conversion_period: usize,
		base_period: usize,
		span_b_period: usize,
	) -> PyResult<Vec<PySeriesStubbed>> {
		let highs_series: Series = highs.0.into();
		let lows_series: Series = lows.0.into();
		let closes_series: Series = closes.0.into();

		// Convert to Vec<f64> for rustTI
		let highs_values: Vec<f64> = highs_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let lows_values: Vec<f64> = lows_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let closes_values: Vec<f64> = closes_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		// Use rustTI
		let ichimoku_result =
			rust_ti::candle_indicators::bulk::ichimoku_cloud(&highs_values, &lows_values, &closes_values, &conversion_period, &base_period, &span_b_period);

		// Extract individual components from tuples
		let (leading_span_a, leading_span_b, base_line, conversion_line, lagged_price) = {
			let mut a = Vec::new();
			let mut b = Vec::new();
			let mut c = Vec::new();
			let mut d = Vec::new();
			let mut e = Vec::new();
			for (val_a, val_b, val_c, val_d, val_e) in ichimoku_result {
				a.push(val_a);
				b.push(val_b);
				c.push(val_c);
				d.push(val_d);
				e.push(val_e);
			}
			(a, b, c, d, e)
		};

		// Convert back to Polars Series
		let leading_span_a_series = Series::new("leading_span_a".into(), leading_span_a);
		let leading_span_b_series = Series::new("leading_span_b".into(), leading_span_b);
		let base_line_series = Series::new("base_line".into(), base_line);
		let conversion_line_series = Series::new("conversion_line".into(), conversion_line);
		let lagged_price_series = Series::new("lagged_price".into(), lagged_price);

		Ok(vec![
			PySeriesStubbed(pyo3_polars::PySeries(leading_span_a_series)),
			PySeriesStubbed(pyo3_polars::PySeries(leading_span_b_series)),
			PySeriesStubbed(pyo3_polars::PySeries(base_line_series)),
			PySeriesStubbed(pyo3_polars::PySeries(conversion_line_series)),
			PySeriesStubbed(pyo3_polars::PySeries(lagged_price_series)),
		])
	}

	#[staticmethod]
	fn donchian_channels_bulk(highs: PySeriesStubbed, lows: PySeriesStubbed, period: usize) -> PyResult<Vec<PySeriesStubbed>> {
		let highs_series: Series = highs.0.into();
		let lows_series: Series = lows.0.into();

		// Convert to Vec<f64> for rustTI
		let highs_values: Vec<f64> = highs_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let lows_values: Vec<f64> = lows_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		// Use rustTI
		let donchian_result = rust_ti::candle_indicators::bulk::donchian_channels(&highs_values, &lows_values, &period);

		// Extract individual components from tuples
		let (lower_band, middle_band, upper_band) = {
			let mut lower = Vec::new();
			let mut middle = Vec::new();
			let mut upper = Vec::new();
			for (l, m, u) in donchian_result {
				lower.push(l);
				middle.push(m);
				upper.push(u);
			}
			(lower, middle, upper)
		};

		// Convert back to Polars Series
		let lower_band_series = Series::new("lower_band".into(), lower_band);
		let middle_band_series = Series::new("middle_band".into(), middle_band);
		let upper_band_series = Series::new("upper_band".into(), upper_band);

		Ok(vec![
			PySeriesStubbed(pyo3_polars::PySeries(lower_band_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_band_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_band_series)),
		])
	}

	#[staticmethod]
	fn keltner_channel_bulk(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		closes: PySeriesStubbed,
		constant_model_type: &str,
		atr_constant_model_type: &str,
		multiplier: f64,
		period: usize,
	) -> PyResult<Vec<PySeriesStubbed>> {
		let highs_series: Series = highs.0.into();
		let lows_series: Series = lows.0.into();
		let closes_series: Series = closes.0.into();

		// Convert to Vec<f64> for rustTI
		let highs_values: Vec<f64> = highs_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let lows_values: Vec<f64> = lows_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let closes_values: Vec<f64> = closes_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		// Convert string to ConstantModelType
		let constant_type = parse_constant_model_type(constant_model_type)?;
		let atr_constant_type = parse_constant_model_type(atr_constant_model_type)?;

		// Use rustTI
		let keltner_result =
			rust_ti::candle_indicators::bulk::keltner_channel(&highs_values, &lows_values, &closes_values, &constant_type, &atr_constant_type, &multiplier, &period);

		// Extract individual components from tuples
		let (lower_band, middle_band, upper_band) = {
			let mut lower = Vec::new();
			let mut middle = Vec::new();
			let mut upper = Vec::new();
			for (l, m, u) in keltner_result {
				lower.push(l);
				middle.push(m);
				upper.push(u);
			}
			(lower, middle, upper)
		};

		// Convert back to Polars Series
		let lower_band_series = Series::new("lower_band".into(), lower_band);
		let middle_band_series = Series::new("middle_band".into(), middle_band);
		let upper_band_series = Series::new("upper_band".into(), upper_band);

		Ok(vec![
			PySeriesStubbed(pyo3_polars::PySeries(lower_band_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_band_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_band_series)),
		])
	}

	#[staticmethod]
	fn supertrend_bulk(
		highs: PySeriesStubbed,
		lows: PySeriesStubbed,
		closes: PySeriesStubbed,
		constant_model_type: &str,
		multiplier: f64,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let highs_series: Series = highs.0.into();
		let lows_series: Series = lows.0.into();
		let closes_series: Series = closes.0.into();

		// Convert to Vec<f64> for rustTI
		let highs_values: Vec<f64> = highs_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let lows_values: Vec<f64> = lows_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();
		let closes_values: Vec<f64> = closes_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		// Convert string to ConstantModelType
		let constant_type = parse_constant_model_type(constant_model_type)?;

		// Use rustTI
		let supertrend_result = rust_ti::candle_indicators::bulk::supertrend(&highs_values, &lows_values, &closes_values, &constant_type, &multiplier, &period);

		// Convert back to Polars Series
		let result_series = Series::new("supertrend".into(), supertrend_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}
}
