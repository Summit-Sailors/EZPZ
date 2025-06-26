from typing import TYPE_CHECKING, Literal, overload
from contextvars import ContextVar

if TYPE_CHECKING:
  from contextvars import Token

  from sqlalchemy.ext.asyncio import AsyncSession

_session = ContextVar["AsyncSession | None"]("_session", default=None)


@overload
def get_session(*, strict: Literal[True] = True) -> "AsyncSession": ...


@overload
def get_session(*, strict: Literal[False]) -> "AsyncSession | None": ...


def get_session(*, strict: bool = True) -> "AsyncSession | None":
  if (session := _session.get()) is None and strict:
    raise RuntimeError("PANIC")
  return session


def set_session(session: "AsyncSession") -> "Token[AsyncSession | None]":
  return _session.set(session)


def reset_session(token: "Token[AsyncSession | None]") -> None:
  _session.reset(token)
