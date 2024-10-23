import uuid
import datetime
from enum import Enum
from typing import TYPE_CHECKING, get_args, get_origin

import numpy as np
import polars as pl

if TYPE_CHECKING:
  from polars._typing import PolarsDataType

PRIMITIVE_TYPE_MAP = {
  str: pl.Utf8(),
  int: pl.Int64(),
  float: pl.Float64(),
  bool: pl.Boolean(),
  dict: pl.__dict__,
  datetime.datetime: pl.Datetime(),
  datetime.date: pl.Date(),
  datetime.time: pl.Time(),
  uuid.UUID: pl.Utf8(),
}

NP_TYPE_MAP = {
  np.int8: pl.Int8(),
  np.int16: pl.Int16(),
  np.int32: pl.Int32(),
  np.int64: pl.Int64(),
  np.uint8: pl.UInt8(),
  np.uint16: pl.UInt16(),
  np.uint32: pl.UInt32(),
  np.uint64: pl.UInt64(),
  np.float32: pl.Float32(),
  np.float64: pl.Float64(),
  np.bool_: pl.Boolean(),
}


def py_type_to_dtype(py_type: type) -> "PolarsDataType":
  if py_type in PRIMITIVE_TYPE_MAP:
    return PRIMITIVE_TYPE_MAP[py_type]
  origin = get_origin(py_type)
  args = get_args(py_type)
  if origin is list or py_type is list:
    if args:
      return pl.List(py_type_to_dtype(args[0]))
    raise Exception("missing type hints")
  if py_type is np.ndarray or (isinstance(py_type, type) and issubclass(py_type, np.ndarray)):
    if args and args[0] in NP_TYPE_MAP:
      return NP_TYPE_MAP[args[0]]
    raise Exception("missing type hints")
  if issubclass(py_type, Enum):
    return pl.Enum([e.value for e in py_type])
  raise Exception("What type is this?")
