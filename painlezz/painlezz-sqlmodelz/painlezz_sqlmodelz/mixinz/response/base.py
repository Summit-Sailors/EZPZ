from pydantic import BaseModel


class BaseResponseModel(BaseModel):
  success: bool = True

  class Config:
    read_with_orm_mode = True
    orm_mode = True
