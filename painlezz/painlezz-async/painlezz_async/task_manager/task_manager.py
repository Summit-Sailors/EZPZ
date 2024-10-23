from __future__ import annotations

import asyncio
from enum import Enum
from typing import TYPE_CHECKING, Any, Self, Literal, Coroutine
from collections import Counter
from contextvars import Context
from dataclasses import dataclass

from painlezz_async.task_manager.exceptions import (
  NoLoopFound,
  NotEnteredError,
  ParentTaskError,
  PendingTasksError,
  GroupFinishedError,
  AlreadyEnteredError,
  GroupShuttingDownError,
)

if TYPE_CHECKING:
  from types import TracebackType


class ETaskManagerState(Enum):
  NOT_ENTERED = "NOT_ENTERED"
  ABORTING = "ABORTING"
  ENTERED = "ENTERED"
  EXITING = "EXITING"


@dataclass
class TaskManager[ParentTaskReturnType, TaskGroupReturnType]:
  _loop: asyncio.AbstractEventLoop | None = None

  def __post_init__(self) -> None:
    self._entered = False
    self._exiting = False
    self._aborting = False
    self._parent_cancel_requested = False
    self._tasks = set[asyncio.Task[TaskGroupReturnType]]()
    self._errors = list[BaseException]()
    self._base_error: BaseException | None = None
    self._on_completed_fut: asyncio.Future[bool] | None = None
    self._task_status: Counter[str] = Counter()

  def get_loop(self) -> asyncio.AbstractEventLoop:
    if self._loop is None:
      raise NoLoopFound
    return self._loop

  def __repr__(self) -> str:
    state_str = "aborting" if self._aborting else "entered" if self._entered else "not entered"
    status_str = " ".join((f"{name}={count}" for name, count in self._task_status.most_common()))
    return f"<{self.__class__.__name__} state={state_str} {status_str}>"

  async def __aenter__(self) -> Self:
    if self._entered:
      raise AlreadyEnteredError
    if self._loop is None:
      self._loop = asyncio.get_running_loop()
    _parent_task: asyncio.Task[ParentTaskReturnType] | None = asyncio.current_task(self.get_loop())
    if _parent_task is None:
      raise ParentTaskError
    self._parent_task = _parent_task
    self._entered = True
    return self

  async def __aexit__(
    self, exc_type: type[BaseException] | type[Exception] | None, exc_val: BaseException | Exception | None, exc_tb: TracebackType | None
  ) -> bool:
    self._exiting = True
    if exc_val is not None and self._is_base_error(exc_val) and self._base_error is None:
      self._base_error = exc_val
    propagate_cancellation_error = exc_val if exc_type is asyncio.CancelledError else None
    if self._parent_cancel_requested and self._parent_task.uncancel() == 0:
      propagate_cancellation_error = None
    if exc_type is not None and not self._aborting:
      self._abort()
    while self._tasks:
      if self._on_completed_fut is None:
        self._on_completed_fut = self.get_loop().create_future()
      try:
        await self._on_completed_fut
      except asyncio.CancelledError as ex:
        if not self._aborting:
          propagate_cancellation_error = ex
          self._abort()
      self._on_completed_fut = None
    if self._tasks:
      raise PendingTasksError
    if self._base_error is not None:
      raise self._base_error
    if propagate_cancellation_error and not self._errors:
      raise propagate_cancellation_error
    if exc_type is not None and not issubclass(exc_type, asyncio.CancelledError) and exc_val is not None:
      self._errors.append(exc_val)
      self.task_status["errors"] + 1
    if self._errors:
      try:
        me = BaseExceptionGroup("unhandled errors in TaskManager", self._errors)
        raise me from None
      finally:
        self._errors = []
    return False

  def create_task(
    self, coro: Coroutine[Any, Any, TaskGroupReturnType], *, name: str | None = None, context: Literal["inherit"] | Context | None = "inherit"
  ) -> asyncio.Task[TaskGroupReturnType]:
    if not self._entered:
      raise NotEnteredError
    if self._exiting and not self._tasks:
      raise GroupFinishedError
    if self._aborting:
      raise GroupShuttingDownError
    match context:
      case str():
        if context == "inherit":
          context = self._parent_task.get_context()
      case Context():
        pass
      case None:
        pass
    task = self.get_loop().create_task(coro, context=context)
    if name is not None:
      task.set_name(name)
    if task.done():
      self._on_task_done(task)
    else:
      self._tasks.add(task)
      task.add_done_callback(self._on_task_done)
    self._task_status["total"] += 1
    self._task_status["pending"] += 1
    return task

  def _is_base_error(self, exc: BaseException | Exception) -> bool:
    return isinstance(exc, (SystemExit, KeyboardInterrupt))

  def _abort(self) -> None:
    self._aborting = True
    for t in self._tasks:
      if not t.done():
        t.cancel()

  def _on_task_done(self, task: asyncio.Task[TaskGroupReturnType]) -> None:
    self._tasks.discard(task)
    self._task_status["pending"] -= 1
    if self._on_completed_fut is not None and not self._tasks and not self._on_completed_fut.done():
      self._on_completed_fut.set_result(True)
    if task.cancelled():
      self._task_status["cancelled"] += 1
      return
    exc = task.exception()
    if exc is None:
      self._task_status["completed"] += 1
      return
    self._task_status["failed"] += 1
    self._errors.append(exc)
    if self._is_base_error(exc) and self._base_error is None:
      self._base_error = exc
    if self._parent_task.done():
      self.get_loop().call_exception_handler(
        {"message": f"Task {task!r} has errored out but its parent task {self._parent_task} is already completed", "exception": exc, "task": task}
      )
      return
    if not self._aborting and not self._parent_cancel_requested:
      self._abort()
      self._parent_cancel_requested = True
      self._parent_task.cancel()

  @property
  def task_status(self) -> dict[str, int]:
    return dict(self._task_status.most_common())
