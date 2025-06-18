use {
	crate::utils::{extract_f64_values, parse_constant_model_type},
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

/// Other Technical Indicators - A collection of other analysis functions for financial data

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct OtherTI;

#[gen_stub_pymethods]
#[pymethods]
impl OtherTI {
	/// Return on Investment - Calculates investment value and percentage change for a single period
	///
	/// # Parameters
	/// - `start_price`: f64 - Initial price of the asset
	/// - `end_price`: f64 - Final price of the asset
	/// - `investment`: f64 - Initial investment amount
	///
	/// # Returns
	/// Tuple of (final_investment_value: f64, percent_return: f64)
	/// - `final_investment_value`: The absolute value of the investment at the end
	/// - `percent_return`: The percentage return on the investment
	#[staticmethod]
	fn return_on_investment_single(start_price: f64, end_price: f64, investment: f64) -> PyResult<(f64, f64)> {
		let result = rust_ti::other_indicators::single::return_on_investment(&start_price, &end_price, &investment);
		Ok(result)
	}

	/// Return on Investment Bulk - Calculates ROI for a series of consecutive price periods
	///
	/// # Parameters
	/// - `prices`: PySeriesStubbed - Series of price values (f64)
	/// - `investment`: f64 - Initial investment amount
	///
	/// # Returns
	/// Tuple of (final_investment_values: PySeriesStubbed, percent_returns: PySeriesStubbed)
	/// - `final_investment_values`: Series of absolute investment values for each period
	/// - `percent_returns`: Series of percentage returns for each period
	#[staticmethod]
	fn return_on_investment_bulk(prices: PySeriesStubbed, investment: f64) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
		let values: Vec<f64> = extract_f64_values(prices)?;

		let results = rust_ti::other_indicators::bulk::return_on_investment(&values, &investment);

		let final_values: Vec<f64> = results.iter().map(|(final_val, _)| *final_val).collect();
		let percent_returns: Vec<f64> = results.iter().map(|(_, percent)| *percent).collect();

		let final_series = Series::new("final_investment_value".into(), final_values);
		let percent_series = Series::new("percent_return".into(), percent_returns);

		Ok((PySeriesStubbed(pyo3_polars::PySeries(final_series)), PySeriesStubbed(pyo3_polars::PySeries(percent_series))))
	}

	/// True Range - Calculates the greatest price movement for a single period
	///
	/// # Parameters
	/// - `close`: f64 - Current period's closing price
	/// - `high`: f64 - Current period's highest price
	/// - `low`: f64 - Current period's lowest price
	///
	/// # Returns
	/// f64 - The true range value (maximum of: high-low, |high-prev_close|, |low-prev_close|)
	#[staticmethod]
	fn true_range_single(close: f64, high: f64, low: f64) -> PyResult<f64> {
		let result = rust_ti::other_indicators::single::true_range(&close, &high, &low);
		Ok(result)
	}

	/// True Range Bulk - Calculates true range for a series of OHLC data
	///
	/// # Parameters
	/// - `close`: PySeriesStubbed - Series of closing prices (f64)
	/// - `high`: PySeriesStubbed - Series of high prices (f64)
	/// - `low`: PySeriesStubbed - Series of low prices (f64)
	///
	/// # Returns
	/// PySeriesStubbed - Series of true range values for each period
	#[staticmethod]
	fn true_range_bulk(close: PySeriesStubbed, high: PySeriesStubbed, low: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;

		let results = rust_ti::other_indicators::bulk::true_range(&close_values, &high_values, &low_values);
		let result_series = Series::new("true_range".into(), results);

		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Average True Range - Calculates the moving average of true range values for a single result
	///
	/// # Parameters
	/// - `close`: PySeriesStubbed - Series of closing prices (f64)
	/// - `high`: PySeriesStubbed - Series of high prices (f64)
	/// - `low`: PySeriesStubbed - Series of low prices (f64)
	/// - `constant_model_type`: &str - Type of moving average ("sma", "ema", "wma", etc.)
	///
	/// # Returns
	/// f64 - Single ATR value calculated from the entire price series
	#[staticmethod]
	fn average_true_range_single(close: PySeriesStubbed, high: PySeriesStubbed, low: PySeriesStubbed, constant_model_type: &str) -> PyResult<f64> {
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::other_indicators::single::average_true_range(&close_values, &high_values, &low_values, &constant_type);

		Ok(result)
	}

	/// Average True Range Bulk - Calculates rolling ATR values over specified periods
	///
	/// # Parameters
	/// - `close`: PySeriesStubbed - Series of closing prices (f64)
	/// - `high`: PySeriesStubbed - Series of high prices (f64)
	/// - `low`: PySeriesStubbed - Series of low prices (f64)
	/// - `constant_model_type`: &str - Type of moving average ("sma", "ema", "wma", etc.)
	/// - `period`: usize - Number of periods for the moving average calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series of ATR values for each period
	#[staticmethod]
	fn average_true_range_bulk(
		close: PySeriesStubbed,
		high: PySeriesStubbed,
		low: PySeriesStubbed,
		constant_model_type: &str,
		period: usize,
	) -> PyResult<PySeriesStubbed> {
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let results = rust_ti::other_indicators::bulk::average_true_range(&close_values, &high_values, &low_values, &constant_type, &period);

		let result_series = Series::new("average_true_range".into(), results);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Internal Bar Strength - Calculates buy/sell oscillator based on close position within high-low range
	///
	/// # Parameters
	/// - `high`: f64 - Period's highest price
	/// - `low`: f64 - Period's lowest price
	/// - `close`: f64 - Period's closing price
	///
	/// # Returns
	/// f64 - IBS value between 0 and 1, where values closer to 1 indicate closes near the high,
	///       and values closer to 0 indicate closes near the low
	#[staticmethod]
	fn internal_bar_strength_single(high: f64, low: f64, close: f64) -> PyResult<f64> {
		let result = rust_ti::other_indicators::single::internal_bar_strength(&high, &low, &close);
		Ok(result)
	}

	/// Internal Bar Strength Bulk - Calculates IBS for a series of OHLC data
	///
	/// # Parameters
	/// - `high`: PySeriesStubbed - Series of high prices (f64)
	/// - `low`: PySeriesStubbed - Series of low prices (f64)
	/// - `close`: PySeriesStubbed - Series of closing prices (f64)
	///
	/// # Returns
	/// PySeriesStubbed - Series of IBS values (0-1 range) for each period
	#[staticmethod]
	fn internal_bar_strength_bulk(high: PySeriesStubbed, low: PySeriesStubbed, close: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;

		let results = rust_ti::other_indicators::bulk::internal_bar_strength(&high_values, &low_values, &close_values);
		let result_series = Series::new("internal_bar_strength".into(), results);

		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Positivity Indicator - Generates trading signals based on open vs previous close comparison
	///
	/// # Parameters
	/// - `open`: PySeriesStubbed - Series of opening prices (f64)
	/// - `previous_close`: PySeriesStubbed - Series of previous period closing prices (f64)
	/// - `signal_period`: usize - Number of periods for signal line smoothing
	/// - `constant_model_type`: &str - Type of moving average for signal line ("sma", "ema", "wma", etc.)
	///
	/// # Returns
	/// Tuple of (positivity_indicator: PySeriesStubbed, signal_line: PySeriesStubbed)
	/// - `positivity_indicator`: Series of raw positivity values based on open/close comparison
	/// - `signal_line`: Series of smoothed signal values using specified moving average
	#[staticmethod]
	fn positivity_indicator(
		open: PySeriesStubbed,
		previous_close: PySeriesStubbed,
		signal_period: usize,
		constant_model_type: &str,
	) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
		let open_values: Vec<f64> = extract_f64_values(open)?;
		let close_values: Vec<f64> = extract_f64_values(previous_close)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let results = rust_ti::other_indicators::bulk::positivity_indicator(&open_values, &close_values, &signal_period, &constant_type);

		let positivity_values: Vec<f64> = results.iter().map(|(pos, _)| *pos).collect();
		let signal_values: Vec<f64> = results.iter().map(|(_, signal)| *signal).collect();

		let positivity_series = Series::new("positivity_indicator".into(), positivity_values);
		let signal_series = Series::new("signal_line".into(), signal_values);

		Ok((PySeriesStubbed(pyo3_polars::PySeries(positivity_series)), PySeriesStubbed(pyo3_polars::PySeries(signal_series))))
	}
}
