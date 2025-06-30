from typing import TYPE_CHECKING

from sqlalchemy import String, TypeDecorator
from pydantic_core import Url

if TYPE_CHECKING:
  from pydantic import HttpUrl
  from sqlalchemy import Dialect


class HttpUrlType(TypeDecorator[Url]):
  impl = String
  cache_ok = True

  def process_bind_param(self, value: "HttpUrl | None", dialect: "Dialect") -> str | None:
    if value is not None:
      return str(value)
    return None

  def process_result_value(self, value: str | None, dialect: "Dialect") -> "HttpUrl | None":
    if value is not None:
      return Url(value)
    return None
