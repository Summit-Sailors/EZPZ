from typing import Self, cast
from datetime import UTC, datetime

from sqlmodel import Field, DateTime, SQLModel
from sqlalchemy.orm import UserDefinedOption


class SoftDeleteMixin(SQLModel):
  __abstract__ = True
  deleted_at: datetime | None = Field(default_factory=None, nullable=True, sa_type=cast(type[DateTime], DateTime(timezone=True)))

  def soft_delete(self) -> Self:
    self.deleted_at = datetime.now(UTC)
    return self

  def restore(self) -> Self:
    self.deleted_at = None
    return self


class WithDeleted(UserDefinedOption):
  """
  if with_deleted:
      query = query.options(WithDeleted())
  """

  def __init__(self) -> None:
    self.include_deleted = True
