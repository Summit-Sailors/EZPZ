use {
	crate::utils::{create_triple_df, extract_f64_values, parse_constant_model_type},
	ezpz_stubz::{frame::PyDfStubbed, series::PySeriesStubbed},
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct TrendTI;

#[gen_stub_pymethods]
#[pymethods]
impl TrendTI {
	// Single value functions (return a single value from the entire series)

	#[staticmethod]
	fn aroon_up_single(highs: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(highs)?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Highs cannot be empty"));
		}

		let result = rust_ti::trend_indicators::single::aroon_up(&values);
		Ok(result)
	}

	#[staticmethod]
	fn aroon_down_single(lows: PySeriesStubbed) -> PyResult<f64> {
		let values: Vec<f64> = extract_f64_values(lows)?;

		if values.is_empty() {
			return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>("Lows cannot be empty"));
		}

		let result = rust_ti::trend_indicators::single::aroon_down(&values);
		Ok(result)
	}

	#[staticmethod]
	fn aroon_oscillator_single(aroon_up: f64, aroon_down: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::aroon_oscillator(&aroon_up, &aroon_down);
		Ok(result)
	}

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

	#[staticmethod]
	fn long_parabolic_time_price_system_single(previous_sar: f64, extreme_point: f64, acceleration_factor: f64, low: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::long_parabolic_time_price_system(&previous_sar, &extreme_point, &acceleration_factor, &low);
		Ok(result)
	}

	#[staticmethod]
	fn short_parabolic_time_price_system_single(previous_sar: f64, extreme_point: f64, acceleration_factor: f64, high: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::short_parabolic_time_price_system(&previous_sar, &extreme_point, &acceleration_factor, &high);
		Ok(result)
	}

	#[staticmethod]
	fn volume_price_trend_single(current_price: f64, previous_price: f64, volume: f64, previous_volume_price_trend: f64) -> PyResult<f64> {
		let result = rust_ti::trend_indicators::single::volume_price_trend(&current_price, &previous_price, &volume, &previous_volume_price_trend);
		Ok(result)
	}

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

	// Aroon Up bulk function
	#[staticmethod]
	fn aroon_up_bulk(highs: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let highs_values: Vec<f64> = extract_f64_values(highs)?;

		let result = rust_ti::trend_indicators::bulk::aroon_up(&highs_values, &period);
		let result_series = Series::new("aroon_up".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate Aroon Down indicator
	#[staticmethod]
	fn aroon_down_bulk(lows: PySeriesStubbed, period: usize) -> PyResult<PySeriesStubbed> {
		let lows_values: Vec<f64> = extract_f64_values(lows)?;

		let result = rust_ti::trend_indicators::bulk::aroon_down(&lows_values, &period);
		let result_series = Series::new("aroon_down".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate Aroon Oscillator
	#[staticmethod]
	fn aroon_oscillator_bulk(aroon_up: PySeriesStubbed, aroon_down: PySeriesStubbed) -> PyResult<PySeriesStubbed> {
		let aroon_up_values: Vec<f64> = extract_f64_values(aroon_up)?;
		let aroon_down_values: Vec<f64> = extract_f64_values(aroon_down)?;

		let result = rust_ti::trend_indicators::bulk::aroon_oscillator(&aroon_up_values, &aroon_down_values);
		let result_series = Series::new("aroon_oscillator".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate Aroon Indicator (returns Aroon Up, Aroon Down, and Aroon Oscillator)
	///
	/// Returns a DataFrame with Columns ("aroon_up", "aroon_down", "aroon_oscillator")
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

	/// Calculate Parabolic Time Price System (SAR)
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

	/// Calculate Directional Movement System (returns +DI, -DI, ADX, ADXR)
	///
	/// Returns a DataFrame with columns: (positive_di, negative_di, adx, adxr)
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

	/// Calculate Volume Price Trend
	#[staticmethod]
	fn volume_price_trend_bulk(prices: PySeriesStubbed, volumes: PySeriesStubbed, previous_volume_price_trend: f64) -> PyResult<PySeriesStubbed> {
		let prices_values: Vec<f64> = extract_f64_values(prices)?;
		let volumes_values: Vec<f64> = extract_f64_values(volumes)?;

		let result = rust_ti::trend_indicators::bulk::volume_price_trend(&prices_values, &volumes_values, &previous_volume_price_trend);

		let result_series = Series::new("volume_price_trend".into(), result);
		Ok(PySeriesStubbed(pyo3_polars::PySeries(result_series)))
	}

	/// Calculate True Strength Index
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
