use {
	ezpz_stubz::frame::PyDfStubbed,
	polars::prelude::*,
	pyo3::prelude::*,
	pyo3_stub_gen::{
		define_stub_info_gatherer,
		derive::{gen_stub_pyclass, gen_stub_pymethods},
	},
};

#[gen_stub_pyclass]
#[pyclass]
#[derive(Clone)]
pub struct DataFrameViewer {
	df: DataFrame,
}

impl DataFrameViewer {}

#[gen_stub_pymethods]
#[pymethods]
impl DataFrameViewer {
	#[new]
	fn new(py_df: PyDfStubbed) -> Self {
		Self { df: py_df.0.into() }
	}

	fn view(&self) -> Self {
		let _ = self.df.clone();
		self.clone()
	}
}

define_stub_info_gatherer!(stub_info);
