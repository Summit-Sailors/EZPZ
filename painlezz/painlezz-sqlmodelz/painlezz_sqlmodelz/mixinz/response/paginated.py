from __future__ import annotations

import math
from typing import TYPE_CHECKING, Generic, TypeVar, Iterable
from functools import cached_property

# from pydantic.generics import GenericModel
from sqlmodel import SQLModel
from sqlmodel.ext.asyncio.session import AsyncSession

from pysilo_env.context.asession import get_session
from painlezz_sqlmodelz.mixinz.response.base import BaseResponseModel

if TYPE_CHECKING:
  from sqlmodel.sql.expression import SelectOfScalar
  from sqlmodel.ext.asyncio.session import AsyncSession

# Was error pydantic.errors.PydanticUndefinedAnnotation: name 'T' is not defined
T = TypeVar("T", bound=SQLModel)


class Paginated(BaseResponseModel, Generic[T]):
  has_more: bool
  items: Iterable[T]
  next_page: int
  total_pages: int

  @cached_property
  def session(self) -> "AsyncSession":
    return get_session()

  @classmethod
  async def from_query(cls, count_query: "SelectOfScalar[int]", items_query: "SelectOfScalar[T]", page: int, limit: int) -> Paginated[T]:
    session = get_session()
    _count = await session.exec(count_query)
    count = _count.one()
    items = list(await session.exec(items_query.offset((page - 1) * limit).limit(limit)))
    return Paginated[T](has_more=count > page * limit, items=items, next_page=page + 1, total_pages=math.ceil(count / limit), success=True)
