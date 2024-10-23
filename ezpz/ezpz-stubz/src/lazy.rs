use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::PyLazyFrame;
use pyo3_stub_gen::{define_stub_info_gatherer, PyStubType, TypeInfo};

#[derive(Clone, FromPyObject)]
pub struct PyLfStubbed(pub PyLazyFrame);

impl From<LazyFrame> for PyLfStubbed {
  fn from(df: LazyFrame) -> Self {
    PyLfStubbed(PyLazyFrame(df))
  }
}

impl From<PyLfStubbed> for LazyFrame {
  fn from(value: PyLfStubbed) -> Self {
    value.0 .0
  }
}

impl PyStubType for PyLfStubbed {
  fn type_output() -> TypeInfo {
    TypeInfo::with_module("polars.LazyFrame", "polars".into())
  }
}

define_stub_info_gatherer!(stub_info);
