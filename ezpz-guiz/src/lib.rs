use {pyo3::prelude::*, pyo3_stub_gen::define_stub_info_gatherer};

mod frame;

use frame::DataFrameViewer;

mod lazy;

use lazy::LazyFrameViewer;

#[pymodule]
#[pyo3(name = "_ezpz_guiz")]
fn _ezpz_guiz(m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add_class::<DataFrameViewer>()?;
	m.add_class::<LazyFrameViewer>()?;
	Ok(())
}

define_stub_info_gatherer!(stub_info);
