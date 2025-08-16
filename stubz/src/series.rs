use {
	pyo3::prelude::*,
	pyo3_polars::PySeries,
	pyo3_stub_gen::{PyStubType, TypeInfo, define_stub_info_gatherer},
};

#[derive(Clone, Debug)]
pub struct PySeriesStubbed(pub PySeries);

impl From<PySeries> for PySeriesStubbed {
	fn from(series: PySeries) -> Self {
		PySeriesStubbed(series)
	}
}

impl From<PySeriesStubbed> for PySeries {
	fn from(value: PySeriesStubbed) -> Self {
		value.0
	}
}

impl PyStubType for PySeriesStubbed {
	fn type_output() -> TypeInfo {
		TypeInfo::with_module("polars.Series", "polars".into())
	}
}

impl<'a> FromPyObject<'a> for PySeriesStubbed {
	fn extract_bound(ob: &Bound<'a, PyAny>) -> PyResult<Self> {
		Ok(PySeriesStubbed(PySeries::extract_bound(ob)?))
	}
}

impl<'py> IntoPyObject<'py> for PySeriesStubbed {
	type Error = PyErr;
	type Output = Bound<'py, Self::Target>;
	type Target = PyAny;

	fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
		self.0.into_pyobject(py)
	}
}

define_stub_info_gatherer!(stub_info);
