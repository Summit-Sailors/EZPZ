use {
	pyo3::prelude::*,
	pyo3_polars::PyLazyFrame,
	pyo3_stub_gen::{define_stub_info_gatherer, PyStubType, TypeInfo},
};

#[derive(Clone, FromPyObject)]
pub struct PyLfStubbed(pub PyLazyFrame);

impl From<PyLazyFrame> for PyLfStubbed {
	fn from(df: PyLazyFrame) -> Self {
		PyLfStubbed(df)
	}
}

impl From<PyLfStubbed> for PyLazyFrame {
	fn from(value: PyLfStubbed) -> Self {
		value.0
	}
}

impl PyStubType for PyLfStubbed {
	fn type_output() -> TypeInfo {
		TypeInfo::with_module("polars.LazyFrame", "polars".into())
	}
}

define_stub_info_gatherer!(stub_info);
