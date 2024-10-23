from sqlmodel import SQLModel


class PaginatedParamsMixin(SQLModel):
  page: int = 1
  limit: int = 10
