from enum import StrEnum
from typing import Any, Generator


class EPolarsNS(StrEnum):
  Expr = "Expr"
  DataFrame = "DataFrame"
  LazyFrame = "LazyFrame"
  Series = "Series"

  @property
  def api_decorator(self) -> str:
    match self:
      case EPolarsNS.Expr:
        return "register_expr_namespace"
      case EPolarsNS.DataFrame:
        return "register_dataframe_namespace"
      case EPolarsNS.LazyFrame:
        return "register_lazyframe_namespace"
      case EPolarsNS.Series:
        return "register_series_namespace"

  @classmethod
  def get_api_decorators(cls) -> Generator[str, Any, None]:
    for e_pl_ns in EPolarsNS:
      yield f"register_{e_pl_ns.value.lower()}_namespace"
