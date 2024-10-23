use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::PyDataFrame;
use pyo3_stub_gen::{define_stub_info_gatherer, PyStubType, TypeInfo};

#[derive(Clone, FromPyObject)]
pub struct PyDfStubbed(pub PyDataFrame);

impl From<DataFrame> for PyDfStubbed {
  fn from(df: DataFrame) -> Self {
    PyDfStubbed(PyDataFrame(df))
  }
}

impl From<PyDfStubbed> for DataFrame {
  fn from(value: PyDfStubbed) -> Self {
    value.0 .0
  }
}

impl PyStubType for PyDfStubbed {
  fn type_output() -> TypeInfo {
    TypeInfo::with_module("polars.DataFrame", "polars".into())
  }
}

define_stub_info_gatherer!(stub_info);
