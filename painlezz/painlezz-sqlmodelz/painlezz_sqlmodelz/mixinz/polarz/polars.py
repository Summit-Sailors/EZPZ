import dataclasses
from uuid import UUID
from typing import TYPE_CHECKING, Any, Literal, cast
from itertools import chain

import polars as pl
from sqlmodel import SQLModel
from sqlalchemy import (
  select,
  tuple_,
  update,
  inspect as sa_inspect,
)
from sqlalchemy.dialects.postgresql import psycopg

from pysilo_env.db_env import DB_ENV
from painlezz_sqlmodelz.py_type_to_pl_type import py_type_to_dtype

if TYPE_CHECKING:
  from sqlalchemy import Select, Selectable
  from polars._typing import SchemaDict, DbReadEngine

  from pysilo_env.db_env import DbEnv


class PolarsMixin(SQLModel):
  @classmethod
  def get_pl_schema(cls, *, extras: "SchemaDict|None" = None, exclude: list[str] | None = None) -> "SchemaDict":
    schema: SchemaDict = {name: py_type_to_dtype(col.type.python_type) for name, col in sa_inspect(cls).columns.items() if name not in (exclude or [])}
    if extras:
      schema.update(extras)
    return schema

  @classmethod
  def read_df_uri(
    cls,
    selectable: "Select[Any]",
    *,
    env: "DbEnv" = DB_ENV,
    partition_on: str | None = None,
    partition_range: tuple[int, int] | None = None,
    partition_num: int | None = None,
    protocol: str | None = None,
    engine: "DbReadEngine | None" = None,
    schema_overrides: "SchemaDict | None" = None,
    execute_options: dict[str, Any] | None = None,
  ) -> pl.DataFrame:
    return pl.read_database_uri(
      selectable.compile(dialect=psycopg.dialect(), compile_kwargs={"literal_binds": True}).string,
      env.get_db_url(),
      partition_on=partition_on,
      partition_range=partition_range,
      partition_num=partition_num,
      protocol=protocol,
      engine=engine,
      schema_overrides=schema_overrides,
      execute_options=execute_options,
    )

  @classmethod
  def read_df(
    cls,
    selectable: "Selectable",
    *,
    env: "DbEnv" = DB_ENV,
    iter_batches: Literal[False] = False,
    batch_size: int | None = None,
    schema_overrides: "SchemaDict | None" = None,
    infer_schema_length: int | None = None,
    execute_options: dict[str, Any] | None = None,
  ) -> pl.DataFrame:
    return pl.read_database(
      selectable,
      env.engine,
      iter_batches=iter_batches,
      batch_size=batch_size,
      schema_overrides=schema_overrides,
      infer_schema_length=infer_schema_length,
      execute_options=execute_options,
    )

  @classmethod
  def write_df(
    cls,
    raw_df: pl.DataFrame | pl.LazyFrame,
    env: "DbEnv" = DB_ENV,
    if_table_exists: Literal["replace", "append", "fail", "upsert"] = "upsert",
  ) -> None:
    raw_df = raw_df.lazy()
    if raw_df.collect().is_empty():
      return
    inspection = sa_inspect(cls)
    dc_fields = dataclasses.fields(cls)
    pk_cols = inspection.mapper.primary_key
    pk_names = [column.name for column in inspection.mapper.primary_key]
    if inspection.attrs.keys():
      defaults = {field.name: field.default for field in dc_fields if field.default is not dataclasses.MISSING}
      default_factories = {field.name: field.default_factory for field in dc_fields if field.default_factory is not dataclasses.MISSING}
      raw_df = raw_df.with_columns(
        (
          pl.col(name).fill_null(pl.lit(str(val) if isinstance(val := default_val_fac(), UUID) else val))
          if callable(default_val_fac)
          else pl.col(name).fill_null(pl.lit(default_val_fac))
        ).name.keep()
        for name, default_val_fac in chain.from_iterable([defaults.items(), default_factories.items()])
      )
    to_drop = list(set(raw_df.columns).difference(set(inspection.columns.keys())))
    to_dump_df = raw_df.drop(to_drop).unique(pk_names)
    if to_dump_df.collect().is_empty():
      return
    if if_table_exists == "upsert":
      to_update_df = cls.read_df(select(*pk_cols).filter(tuple_(*pk_cols).in_(to_dump_df.select(pk_names).collect().iter_rows())), env=env)
      if not to_update_df.is_empty():
        with env.get_sa_session() as session, session.begin():
          session.execute(update(cls), to_update_df.to_dicts())  # type: ignore
        to_dump_df = to_dump_df.join(to_update_df.lazy(), on=pk_names, how="anti")
      if_table_exists = "append"
    if not (to_insert_df := to_dump_df.collect()).is_empty():
      to_insert_df.write_database(cast(str, cls.__tablename__), env.engine.url.render_as_string(hide_password=False), if_table_exists=if_table_exists)
