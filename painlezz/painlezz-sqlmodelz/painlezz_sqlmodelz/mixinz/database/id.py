from uuid import UUID, uuid4

from sqlmodel import Field, SQLModel
from sqlalchemy import text


class UuidPKMixin(SQLModel):
  id: UUID = Field(
    default_factory=uuid4, primary_key=True, index=True, nullable=False, sa_column_kwargs={"server_default": text("gen_random_uuid()"), "unique": True}
  )
