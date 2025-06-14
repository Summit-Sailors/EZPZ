use {
	crate::utils::{extract_f64_values, parse_constant_model_type},
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct OtherTI;

#[gen_stub_pymethods]
#[pymethods]
impl OtherTI {
	/// Return on Investment - Calculates investment value and percentage change
	/// Returns tuple of (final_investment_value, percent_return)
	#[staticmethod]
	fn return_on_investment_single(start_price: f64, end_price: f64, investment: f64) -> PyResult<(f64, f64)> {
		let result = rust_ti::other_indicators::single::return_on_investment(&start_price, &end_price, &investment);
		Ok(result)
	}

	/// Return on Investment Bulk - Calculates ROI for a series of prices
	/// Returns tuple of (final_investment_values, percent_returns)
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

	/// True Range - Calculates the greatest price movement over a period
	#[staticmethod]
	fn true_range_single(close: f64, high: f64, low: f64) -> PyResult<f64> {
		let result = rust_ti::other_indicators::single::true_range(&close, &high, &low);
		Ok(result)
	}

	/// True Range Bulk - Calculates true range for series of OHLC data
	#[staticmethod]
	fn true_range_bulk(close: PySeriesStubbed, high: PySeriesStubbed, low: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;

		let results = rust_ti::other_indicators::bulk::true_range(&close_values, &high_values, &low_values);
		let result_series = Series::new("true_range".into(), results);

		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Average True Range - Moving average of true range values
	#[staticmethod]
	fn average_true_range_single(close: PySeriesStubbed, high: PySeriesStubbed, low: PySeriesStubbed, constant_model_type: &str) -> PyResult<f64> {
		let close_values: Vec<f64> = extract_f64_values(close)?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::other_indicators::single::average_true_range(&close_values, &high_values, &low_values, &constant_type);

		Ok(result)
	}

	/// Average True Range Bulk - Moving average of true range values over periods
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

	/// Internal Bar Strength - Buy/sell oscillator based on close position within high-low range
	#[staticmethod]
	fn internal_bar_strength_single(high: f64, low: f64, close: f64) -> PyResult<f64> {
		let result = rust_ti::other_indicators::single::internal_bar_strength(&high, &low, &close);
		Ok(result)
	}

	/// Internal Bar Strength Bulk - IBS for series of OHLC data
	#[staticmethod]
	fn internal_bar_strength_bulk(high: PySeriesStubbed, low: PySeriesStubbed, close: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;
		let close_values: Vec<f64> = extract_f64_values(close)?;

		let results = rust_ti::other_indicators::bulk::internal_bar_strength(&high_values, &low_values, &close_values);
		let result_series = Series::new("internal_bar_strength".into(), results);

		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Positivity Indicator - Signal based on open vs previous close comparison
	/// Returns tuple of (positivity_indicator, signal_line)
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
