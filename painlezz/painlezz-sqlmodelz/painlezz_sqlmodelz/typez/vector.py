from struct import pack, unpack
from typing import TYPE_CHECKING, Literal, Callable, override

import numpy
import numpy as np
from sqlalchemy import Float, Column, String
from sqlalchemy.types import UserDefinedType
from sqlalchemy.dialects.postgresql.base import ischema_names

if TYPE_CHECKING:
  from sqlalchemy import Dialect, Operators, ColumnElement

EMBEDDING_DIMENSION = 1024
HNSW_INDEX_NAME = "hnsw_index"

type Embedding = numpy.ndarray[tuple[int], numpy.dtype[numpy.float32]]
type NpArrayLike = Embedding | list[int] | list[float] | None

type TVecFunc = Literal["cosine_similarity", "inner_product", "l2_distance"]


class Vector(UserDefinedType[Column["Vector"]]):
  cache_ok = True
  _string = String()

  @staticmethod
  def from_db(value: Embedding | None | str) -> Embedding | None:
    # could be ndarray if already cast by lower-level driver
    if value is None or isinstance(value, np.ndarray):
      return value
    return np.array(value[1:-1].split(","), dtype=np.float32)

  @staticmethod
  def from_db_binary(value: Embedding | None) -> Embedding | None:
    if value is None:
      return value
    (dim, unused) = unpack(">HH", value[:4])
    return np.frombuffer(value, dtype=">f", count=dim, offset=4).astype(dtype=np.float32)

  @staticmethod
  def to_db(value: NpArrayLike, dim: int | None = None) -> None | str:
    if value is None:
      return value
    if isinstance(value, np.ndarray):
      if value.ndim != 1:
        raise ValueError("expected ndim to be 1")
      if not np.issubdtype(value.dtype, np.integer) and not np.issubdtype(value.dtype, np.floating):
        raise ValueError("dtype must be numeric")
      value = value.tolist()
    if isinstance(value, list):
      if not value:
        raise ValueError("expected non empty list")
      if not (isinstance(value[0], (float, int))):
        raise ValueError("must be numeric")
    if dim is not None and value is not None and len(value) != dim:
      raise ValueError("expected %d dimensions, not %d" % (dim, len(value)))
    return "[" + ",".join([str(float(v)) for v in value or []]) + "]"

  @staticmethod
  def to_db_binary(value: NpArrayLike) -> None | bytes:
    if value is None:
      return value
    value = np.asarray(value, dtype=">f")
    if value.ndim != 1:
      raise ValueError("expected ndim to be 1")
    return pack(">HH", value.shape[0], 0) + value.tobytes()

  @override
  def __init__(self, dim: int) -> None:
    self.dim = dim

  def get_col_spec(self) -> str:
    return "VECTOR(%d)" % self.dim

  @override
  def bind_processor(self, dialect: "Dialect") -> Callable[[NpArrayLike], str | None]:
    def process(value: NpArrayLike) -> str | None:
      return self.to_db(value, dim=self.dim)

    return process

  @override
  def literal_processor(self, dialect: "Dialect") -> Callable[[NpArrayLike], str]:
    string_literal_processor = self._string._cached_literal_processor(dialect)  # noqa: SLF001
    assert string_literal_processor

    def process(value: NpArrayLike) -> str:
      return string_literal_processor(self.to_db(value, dim=self.dim))

    return process

  @override
  def result_processor(self, dialect: "Dialect", coltype: Embedding | None) -> Callable[[str | None | Embedding], list[float] | None]:
    def process(value: str | None | Embedding) -> list[float] | None:
      if value is None:
        return value
      if isinstance(value, numpy.ndarray):
        return value.tolist()
      return np_arr.tolist() if (np_arr := self.from_db(value)) is not None else None

    return process

  @override
  class comparator_factory(UserDefinedType.Comparator["Vector"]):
    def l2_distance(self, other: "ColumnElement[Vector]") -> "Operators":
      return self.op("<->", return_type=Float[float])(other)

    def max_inner_product(self, other: "ColumnElement[Vector]") -> "Operators":
      return self.op("<#>", return_type=Float[float])(other)

    def cosine_distance(self, other: "ColumnElement[Vector]") -> "Operators":
      return self.op("<=>", return_type=Float[float])(other)


# for reflection
ischema_names["vector"] = Vector  # type: ignore
