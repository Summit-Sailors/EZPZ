import sqlalchemy
from sqlmodel import Field, SQLModel


class EnabledMixin(SQLModel):
  enabled: bool = Field(default=True, nullable=False, sa_column_kwargs={"server_default": sqlalchemy.true()})


class EnabledNoMixin(SQLModel):
  enabled: bool = Field(default=True, nullable=False, sa_column_kwargs={"server_default": sqlalchemy.false()})
