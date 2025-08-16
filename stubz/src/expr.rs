use {
	pyo3::prelude::*,
	pyo3_polars::PyExpr,
	pyo3_stub_gen::{PyStubType, TypeInfo, define_stub_info_gatherer},
};

#[derive(Clone)]
pub struct PyExprStubbed(pub PyExpr);

impl From<PyExpr> for PyExprStubbed {
	fn from(expr: PyExpr) -> Self {
		PyExprStubbed(expr)
	}
}

impl From<PyExprStubbed> for PyExpr {
	fn from(value: PyExprStubbed) -> Self {
		value.0
	}
}

impl PyStubType for PyExprStubbed {
	fn type_output() -> TypeInfo {
		TypeInfo::with_module("polars.Expr", "polars".into())
	}
}

impl<'a> FromPyObject<'a> for PyExprStubbed {
	fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
		Ok(PyExprStubbed(PyExpr::extract_bound(ob)?))
	}
}

impl<'py> IntoPyObject<'py> for PyExprStubbed {
	type Error = PyErr;
	type Output = Bound<'py, Self::Target>;
	type Target = PyAny;

	fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
		self.0.into_pyobject(py)
	}
}

define_stub_info_gatherer!(stub_info);
