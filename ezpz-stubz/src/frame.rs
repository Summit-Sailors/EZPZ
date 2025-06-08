use {
	pyo3::prelude::*,
	pyo3_polars::PyDataFrame,
	pyo3_stub_gen::{PyStubType, TypeInfo, define_stub_info_gatherer},
};

#[derive(Clone)]
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

impl<'a> FromPyObject<'a> for PyDfStubbed {
	fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
		Ok(PyDfStubbed(PyDataFrame::extract_bound(ob)?))
	}
}

impl<'py> IntoPyObject<'py> for PyDfStubbed {
	type Error = PyErr;
	type Output = Bound<'py, Self::Target>;
	type Target = PyAny;

	fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
		self.0.into_pyobject(py)
	}
}

define_stub_info_gatherer!(stub_info);
