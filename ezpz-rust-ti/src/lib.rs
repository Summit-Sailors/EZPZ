use {pyo3::prelude::*, pyo3_stub_gen::define_stub_info_gatherer};
mod indicators;
mod utils;

pub use indicators::{basic, candle, chart, correlation, ma, momentum, other, std_, strength, trend, volatility};
use {
	basic::BasicTI, candle::CandleTI, chart::ChartTrendsTI, correlation::CorrelationTI, ma::MATI, momentum::MomentumTI, other::OtherTI, std_::StandardTI,
	trend::TrendTI, volatility::VolatilityTI,
};

#[pymodule]
#[pyo3(name = "_ezpz_rust_ti")]
fn _ezpz_rust_ti(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<BasicTI>()?;
	m.add_class::<CorrelationTI>()?;
	m.add_class::<MATI>()?;
	m.add_class::<StandardTI>()?;
	m.add_class::<VolatilityTI>()?;
	m.add_class::<CandleTI>()?;
	m.add_class::<OtherTI>()?;
	m.add_class::<ChartTrendsTI>()?;
	m.add_class::<TrendTI>()?;
	m.add_class::<MomentumTI>()?;
	Ok(())
}

define_stub_info_gatherer!(stub_info);
