class TaskManagerError(Exception):
  """Base exception for all TaskManager related errors."""


class AlreadyEnteredError(TaskManagerError):
  """Raised when attempting to enter an TaskManager that has already been entered."""


class NotEnteredError(TaskManagerError):
  """Raised when attempting to use an TaskManager that has not been entered."""


class GroupFinishedError(TaskManagerError):
  """Raised when attempting to add a task to a finished TaskManager."""


class GroupShuttingDownError(TaskManagerError):
  """Raised when attempting to add a task to an TaskManager that is shutting down."""


class PendingTasksError(TaskManagerError):
  """Raised when an TaskManager exits with pending tasks."""


class NoLoopFound(TaskManagerError):
  """Raised when an TaskManager can find the loop."""


class ParentTaskError(TaskManagerError):
  """Raised when the TaskManager cannot determine the parent task."""


class InvalidExceptionTypeError(TaskManagerError):
  """Raised when an invalid exception type is passed to _is_base_error."""
