from typing import Generic, TypeVar, Iterable

from painlezz_sqlmodelz.mixinz.response.base import BaseResponseModel

T = TypeVar("T")


class ItemResponse(BaseResponseModel, Generic[T]):
  success: bool = True
  item: T | None


class ItemsResponse(BaseResponseModel, Generic[T]):
  success: bool = True
  items: Iterable[T]
