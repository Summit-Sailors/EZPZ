from __future__ import annotations

import inspect
import logging
from typing import TYPE_CHECKING, Literal, ClassVar
from pathlib import Path
from datetime import datetime

import structlog

if TYPE_CHECKING:
  from structlog.types import EventDict, WrappedLogger, FilteringBoundLogger

LogLevel = Literal["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]

ColorKey = Literal["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL", "RESET"]

LogEventDict = dict[str, str | int | datetime | None]


class ColoredFormatter:
  COLORS: ClassVar[dict[ColorKey, str]] = {
    "DEBUG": "\033[36m",  # Cyan
    "INFO": "\033[32m",  # Green
    "WARNING": "\033[33m",  # Yellow
    "ERROR": "\033[31m",  # Red
    "CRITICAL": "\033[35m",  # Magenta
    "RESET": "\033[0m",  # Reset
  }

  def __call__(self, logger: WrappedLogger, method_name: str, event_dict: EventDict) -> str:
    """Format log event with colors and aligned fields."""
    level: str = event_dict.get("level", method_name).upper()
    log_color: str = self.COLORS.get(level, "")  # type: ignore[dict-item]
    reset_color: str = self.COLORS["RESET"]

    # caller info with defaults
    filename: str = Path(event_dict.get("pathname", "unknown")).stem
    lineno: str = str(event_dict.get("lineno", 0))

    # the main message
    event: str = str(event_dict.get("event", ""))

    # format
    formatted: str = f"{log_color}[{level:8}]{reset_color} {filename}:{lineno:<4} - {event}"

    # structured data addition, excluding core fields
    extra_data = {k: v for k, v in event_dict.items() if k not in ("event", "level", "pathname", "lineno", "timestamp", "logger")}
    if extra_data:
      formatted += f" {extra_data!r}"

    return formatted


def add_caller_info(_: WrappedLogger, __: str, event_dict: EventDict) -> EventDict:
  frame = inspect.currentframe()
  try:
    # Walk up the stack to find the actual caller
    caller_frame = frame
    while caller_frame:
      caller_frame = caller_frame.f_back
      if caller_frame and not any(path in caller_frame.f_code.co_filename for path in ["structlog", "logging", "_log.py"]):
        break

    if caller_frame:
      event_dict["pathname"] = caller_frame.f_code.co_filename
      event_dict["lineno"] = caller_frame.f_lineno
  finally:
    del frame
  return event_dict


def setup_logger(
  name: str = "app",
  level: int | LogLevel = logging.INFO,
) -> FilteringBoundLogger:
  if isinstance(level, str):
    level = getattr(logging, level.upper())

  if not structlog.is_configured():
    structlog.configure(
      processors=[
        add_caller_info,
        structlog.processors.TimeStamper(fmt="iso"),
        structlog.processors.add_log_level,
        ColoredFormatter(),
      ],
      wrapper_class=structlog.make_filtering_bound_logger(level),
      logger_factory=structlog.PrintLoggerFactory(),
      cache_logger_on_first_use=True,
    )

  return structlog.get_logger(name)


class LoggerFactory:
  _default_logger: FilteringBoundLogger | None = None

  @classmethod
  def get_logger(cls, name: str | None = None) -> FilteringBoundLogger:
    if name is None:
      if cls._default_logger is None:
        cls._default_logger = setup_logger()
      return cls._default_logger
    return setup_logger(name)

  @classmethod
  def reset(cls) -> None:
    structlog.reset_defaults()
    cls._default_logger = None


if __name__ == "__main__":
  logger = LoggerFactory.get_logger("test")

  logger.debug("Debug message")
  logger.info("Info message")
  logger.warning("Warning message")
  logger.error("Error message")
  logger.critical("Critical message")

  # Structured logging example
  logger.info("User action", user_id=12345, action="login")

  # Different logger instance
  other_logger = LoggerFactory.get_logger("other")
  other_logger.info("Message from another module")

__all__ = ["LoggerFactory", "setup_logger"]
