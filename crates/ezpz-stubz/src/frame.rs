use {
	pyo3::prelude::*,
	pyo3_polars::PyDataFrame,
	pyo3_stub_gen::{define_stub_info_gatherer, PyStubType, TypeInfo},
};

#[derive(Clone, FromPyObject)]
pub struct PyDfStubbed(pub PyDataFrame);

impl From<PyDataFrame> for PyDfStubbed {
	fn from(df: PyDataFrame) -> Self {
		PyDfStubbed(df)
	}
}

impl From<PyDfStubbed> for PyDataFrame {
	fn from(value: PyDfStubbed) -> Self {
		value.0
	}
}

impl PyStubType for PyDfStubbed {
	fn type_output() -> TypeInfo {
		TypeInfo::with_module("polars.DataFrame", "polars".into())
	}
}

define_stub_info_gatherer!(stub_info);
