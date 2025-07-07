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
pub struct OtherTI {
	pub series: PySeriesStubbed,
}

#[gen_stub_pymethods]
#[pymethods]
impl OtherTI {
	#[new]
	fn new(series: PySeriesStubbed) -> Self {
		Self { series }
	}

	/// Return on Investment - Calculates investment value and percentage change for a single period
	/// Uses the first and last values from the series as start and end prices
	///
	/// # Parameters
	/// - `investment`: f64 - Initial investment amount
	///
	/// # Returns
	/// Tuple of (final_investment_value: f64, percent_return: f64)
	/// - `final_investment_value`: The absolute value of the investment at the end
	/// - `percent_return`: The percentage return on the investment
	fn return_on_investment_single(&self, investment: f64) -> PyResult<(f64, f64)> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		if values.len() < 2 {
			return Err(pyo3::exceptions::PyValueError::new_err("Series must have at least 2 values"));
		}
		let start_price = values[0];
		let end_price = values[values.len() - 1];
		let result = rust_ti::other_indicators::single::return_on_investment(start_price, end_price, investment);
		Ok(result)
	}

	/// Return on Investment Bulk - Calculates ROI for a series of consecutive price periods
	/// Uses the series as price values for consecutive period calculations
	///
	/// # Parameters
	/// - `investment`: f64 - Initial investment amount
	///
	/// # Returns
	/// Tuple of (final_investment_values: PySeriesStubbed, percent_returns: PySeriesStubbed)
	/// - `final_investment_values`: Series of absolute investment values for each period
	/// - `percent_returns`: Series of percentage returns for each period
	fn return_on_investment_bulk(&self, investment: f64) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
		let values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let results = rust_ti::other_indicators::bulk::return_on_investment(&values, investment);

		let final_values: Vec<f64> = results.iter().map(|(final_val, _)| *final_val).collect();
		let percent_returns: Vec<f64> = results.iter().map(|(_, percent)| *percent).collect();

		let final_series = Series::new("final_investment_value".into(), final_values);
		let percent_series = Series::new("percent_return".into(), percent_returns);

		Ok((PySeriesStubbed(pyo3_polars::PySeries(final_series)), PySeriesStubbed(pyo3_polars::PySeries(percent_series))))
	}

	/// True Range - Calculates the greatest price movement for a single period
	/// Uses the series as closing prices along with provided high/low data
	///
	/// # Parameters
	/// - `high`: PySeriesStubbed - Series of high prices (f64)
	/// - `low`: PySeriesStubbed - Series of low prices (f64)
	///
	/// # Returns
	/// PySeriesStubbed - Series of true range values for each period
	fn true_range(&self, high: PySeriesStubbed, low: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let close_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;

		let results = rust_ti::other_indicators::bulk::true_range(&close_values, &high_values, &low_values);
		let result_series = Series::new("true_range".into(), results);

		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Average True Range - Calculates the moving average of true range values for a single result
	/// Uses the series as closing prices to calculate ATR from the entire price series
	///
	/// # Parameters
	/// - `high`: PySeriesStubbed - Series of high prices (f64)
	/// - `low`: PySeriesStubbed - Series of low prices (f64)
	/// - `constant_model_type`: &str - Type of moving average ("sma", "ema", "wma", etc.)
	///
	/// # Returns
	/// f64 - Single ATR value calculated from the entire price series
	fn average_true_range_single(&self, high: PySeriesStubbed, low: PySeriesStubbed, constant_model_type: &str) -> PyResult<f64> {
		let close_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let result = rust_ti::other_indicators::single::average_true_range(&close_values, &high_values, &low_values, constant_type);

		Ok(result)
	}

	/// Average True Range Bulk - Calculates rolling ATR values over specified periods
	/// Uses the series as closing prices for rolling ATR calculations
	///
	/// # Parameters
	/// - `high`: PySeriesStubbed - Series of high prices (f64)
	/// - `low`: PySeriesStubbed - Series of low prices (f64)
	/// - `constant_model_type`: &str - Type of moving average ("sma", "ema", "wma", etc.)
	/// - `period`: usize - Number of periods for the moving average calculation
	///
	/// # Returns
	/// PySeriesStubbed - Series of ATR values for each period
	fn average_true_range_bulk(&self, high: PySeriesStubbed, low: PySeriesStubbed, constant_model_type: &str, period: usize) -> PyResult<PySeriesStubbed> {
		let close_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let results = rust_ti::other_indicators::bulk::average_true_range(&close_values, &high_values, &low_values, constant_type, period);

		let result_series = Series::new("average_true_range".into(), results);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Internal Bar Strength - Calculates buy/sell oscillator based on close position within high-low range
	/// Uses the series as closing prices to calculate IBS values
	///
	/// # Parameters
	/// - `high`: PySeriesStubbed - Series of high prices (f64)
	/// - `low`: PySeriesStubbed - Series of low prices (f64)
	///
	/// # Returns
	/// PySeriesStubbed - Series of IBS values (0-1 range) for each period, where values closer to 1
	///                   indicate closes near the high, and values closer to 0 indicate closes near the low
	fn internal_bar_strength(&self, high: PySeriesStubbed, low: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let close_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let high_values: Vec<f64> = extract_f64_values(high)?;
		let low_values: Vec<f64> = extract_f64_values(low)?;

		let results = rust_ti::other_indicators::bulk::internal_bar_strength(&high_values, &low_values, &close_values);
		let result_series = Series::new("internal_bar_strength".into(), results);

		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Positivity Indicator - Generates trading signals based on open vs previous close comparison
	/// Uses the series as previous close prices for signal generation
	///
	/// # Parameters
	/// - `open`: PySeriesStubbed - Series of opening prices (f64)
	/// - `signal_period`: usize - Number of periods for signal line smoothing
	/// - `constant_model_type`: &str - Type of moving average for signal line ("sma", "ema", "wma", etc.)
	///
	/// # Returns
	/// Tuple of (positivity_indicator: PySeriesStubbed, signal_line: PySeriesStubbed)
	/// - `positivity_indicator`: Series of raw positivity values based on open/close comparison
	/// - `signal_line`: Series of smoothed signal values using specified moving average
	fn positivity_indicator(&self, open: PySeriesStubbed, signal_period: usize, constant_model_type: &str) -> PyResult<(PySeriesStubbed, PySeriesStubbed)> {
		let close_values: Vec<f64> = extract_f64_values(self.series.clone())?;
		let open_values: Vec<f64> = extract_f64_values(open)?;

		let constant_type = parse_constant_model_type(constant_model_type)?;
		let results = rust_ti::other_indicators::bulk::positivity_indicator(&open_values, &close_values, signal_period, constant_type);

		let positivity_values: Vec<f64> = results.iter().map(|(pos, _)| *pos).collect();
		let signal_values: Vec<f64> = results.iter().map(|(_, signal)| *signal).collect();

		let positivity_series = Series::new("positivity_indicator".into(), positivity_values);
		let signal_series = Series::new("signal_line".into(), signal_values);

		Ok((PySeriesStubbed(pyo3_polars::PySeries(positivity_series)), PySeriesStubbed(pyo3_polars::PySeries(signal_series))))
	}
}
