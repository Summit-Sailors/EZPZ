// Standard Indicators
use {
	ezpz_stubz::series::PySeriesStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct StandardTI;

#[gen_stub_pymethods]
#[pymethods]
impl StandardTI {
	/// Simple Moving Average - calculates the mean over a rolling window
	#[staticmethod]
	fn sma(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.len() < period {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least period ({})", values.len(), period)));
		}

		let sma_result = rust_ti::standard_indicators::bulk::simple_moving_average(&values, &period);
		let result_series = Series::new("sma".into(), sma_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Smoothed Moving Average - puts more weight on recent prices
	#[staticmethod]
	fn smma(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.len() < period {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least period ({})", values.len(), period)));
		}

		let smma_result = rust_ti::standard_indicators::bulk::smoothed_moving_average(&values, &period);
		let result_series = Series::new("smma".into(), smma_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Exponential Moving Average - puts exponentially more weight on recent prices
	#[staticmethod]
	fn ema(series: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.len() < period {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least period ({})", values.len(), period)));
		}

		let ema_result = rust_ti::standard_indicators::bulk::exponential_moving_average(&values, &period);
		let result_series = Series::new("ema".into(), ema_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Bollinger Bands - returns three series: lower band, middle (SMA), upper band
	/// Standard period is 20 with 2 standard deviations
	#[staticmethod]
	fn bollinger_bands(series: PySeriesStubbed) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.len() < 20 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least 20 for Bollinger Bands", values.len())));
		}

		let bb_result = rust_ti::standard_indicators::bulk::bollinger_bands(&values);

		let lower: Vec<f64> = bb_result.iter().map(|(l, _, _)| *l).collect();
		let middle: Vec<f64> = bb_result.iter().map(|(_, m, _)| *m).collect();
		let upper: Vec<f64> = bb_result.iter().map(|(_, _, u)| *u).collect();

		let lower_series = Series::new("bb_lower".into(), lower);
		let middle_series = Series::new("bb_middle".into(), middle);
		let upper_series = Series::new("bb_upper".into(), upper);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(lower_series)),
			PySeriesStubbed(pyo3_polars::PySeries(middle_series)),
			PySeriesStubbed(pyo3_polars::PySeries(upper_series)),
		))
	}

	/// MACD - Moving Average Convergence Divergence
	/// Returns three series: MACD line, Signal line, Histogram
	/// Standard periods: 12, 26, 9
	#[staticmethod]
	fn macd(series: PySeriesStubbed) -> PyResult<(PySeriesStubbed, PySeriesStubbed, PySeriesStubbed)> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.len() < 34 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least 34 for MACD", values.len())));
		}

		let macd_result = rust_ti::standard_indicators::bulk::macd(&values);

		let macd_line: Vec<f64> = macd_result.iter().map(|(m, _, _)| *m).collect();
		let signal_line: Vec<f64> = macd_result.iter().map(|(_, s, _)| *s).collect();
		let histogram: Vec<f64> = macd_result.iter().map(|(_, _, h)| *h).collect();

		let macd_series = Series::new("macd".into(), macd_line);
		let signal_series = Series::new("macd_signal".into(), signal_line);
		let histogram_series = Series::new("macd_histogram".into(), histogram);

		Ok((
			PySeriesStubbed(pyo3_polars::PySeries(macd_series)),
			PySeriesStubbed(pyo3_polars::PySeries(signal_series)),
			PySeriesStubbed(pyo3_polars::PySeries(histogram_series)),
		))
	}

	/// RSI - Relative Strength Index
	/// Standard period is 14 using smoothed moving average
	#[staticmethod]
	fn rsi(series: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.len() < 14 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Series length ({}) must be at least 14 for RSI", values.len())));
		}

		let rsi_result = rust_ti::standard_indicators::bulk::rsi(&values);
		let result_series = Series::new("rsi".into(), rsi_result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	// Single value methods (for when you want just one calculation)

	/// Simple Moving Average - single value calculation
	#[staticmethod]
	fn sma_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}

		let result = rust_ti::standard_indicators::single::simple_moving_average(&values);
		Ok(result)
	}

	/// Smoothed Moving Average - single value calculation
	#[staticmethod]
	fn smma_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}

		let result = rust_ti::standard_indicators::single::smoothed_moving_average(&values);
		Ok(result)
	}

	/// Exponential Moving Average - single value calculation
	#[staticmethod]
	fn ema_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Series cannot be empty"));
		}

		let result = rust_ti::standard_indicators::single::exponential_moving_average(&values);
		Ok(result)
	}

	/// Bollinger Bands - single value calculation (requires exactly 20 periods)
	#[staticmethod]
	fn bollinger_bands_single(series: PySeriesStubbed) -> PyResult<(f64, f64, f64)> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.len() != 20 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
				"Series length must be exactly 20 for single Bollinger Bands calculation, got {}",
				values.len()
			)));
		}

		let result = rust_ti::standard_indicators::single::bollinger_bands(&values);
		Ok(result)
	}

	/// MACD - single value calculation (requires exactly 34 periods)
	#[staticmethod]
	fn macd_single(series: PySeriesStubbed) -> PyResult<(f64, f64, f64)> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.len() != 34 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
				"Series length must be exactly 34 for single MACD calculation, got {}",
				values.len()
			)));
		}

		let result = rust_ti::standard_indicators::single::macd(&values);
		Ok(result)
	}

	/// RSI - single value calculation (requires exactly 14 periods)
	#[staticmethod]
	fn rsi_single(series: PySeriesStubbed) -> PyResult<f64> {
		let polars_series: Series = series.0.into();
		let values: Vec<f64> = polars_series
			.cast(&DataType::Float64)
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.f64()
			.map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?
			.into_no_null_iter()
			.collect();

		if values.len() != 14 {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
				"Series length must be exactly 14 for single RSI calculation, got {}",
				values.len()
			)));
		}

		let result = rust_ti::standard_indicators::single::rsi(&values);
		Ok(result)
	}
}
