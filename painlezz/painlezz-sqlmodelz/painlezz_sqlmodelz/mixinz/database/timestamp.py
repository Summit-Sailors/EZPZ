from datetime import datetime, timezone

from sqlmodel import Field, Column, DateTime, SQLModel, func


class CreatedAtMixin(SQLModel):
  created_at: datetime = Field(
    default_factory=lambda: datetime.now(timezone.utc),
    sa_column=Column(DateTime(timezone=True), nullable=False, server_default=func.now()),
  )


class UpdatedAtMixin:
  updated_at: datetime = Field(
    default_factory=lambda: datetime.now(timezone.utc),
    sa_column=Column(DateTime(timezone=True), nullable=False, server_default=func.now(), onupdate=func.now()),
  )


class TimeStampeMixin(CreatedAtMixin, UpdatedAtMixin): ...
