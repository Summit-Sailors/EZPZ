use {
	pyo3::prelude::*,
	pyo3_polars::PyLazyFrame,
	pyo3_stub_gen::{PyStubType, TypeInfo, define_stub_info_gatherer},
};

#[derive(Clone)]
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

impl<'a> FromPyObject<'a> for PyLfStubbed {
	fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
		Ok(PyLfStubbed(PyLazyFrame::extract_bound(ob)?))
	}
}

impl<'py> IntoPyObject<'py> for PyLfStubbed {
	type Error = PyErr;
	type Output = Bound<'py, Self::Target>;
	type Target = PyAny;

	fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
		self.0.into_pyobject(py)
	}
}

define_stub_info_gatherer!(stub_info);
