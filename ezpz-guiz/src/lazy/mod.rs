use {
	ezpz_stubz::lazy::PyLfStubbed,
	polars::prelude::*,
	pyo3::{PyResult, pyclass, pymethods},
	pyo3_stub_gen::{
		define_stub_info_gatherer,
		derive::{gen_stub_pyclass, gen_stub_pymethods},
	},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct LazyFrameViewer {
	lf: LazyFrame,
}

#[gen_stub_pymethods]
#[pymethods]
impl LazyFrameViewer {
	#[new]
	pub fn new(py_lf: PyLfStubbed) -> PyResult<Self> {
		Ok(Self { lf: py_lf.0.into() })
	}

	fn view(&self) -> Self {
		let _ = self.lf.clone();
		self.clone()
	}
}

define_stub_info_gatherer!(stub_info);
