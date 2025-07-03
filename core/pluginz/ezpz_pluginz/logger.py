import sys
import logging
from typing import ClassVar, Optional
from pathlib import Path


class ColoredFormatter(logging.Formatter):
  COLORS: ClassVar[dict[str, str]] = {
    "DEBUG": "\033[36m",  # Cyan
    "INFO": "\033[32m",  # Green
    "WARNING": "\033[33m",  # Yellow
    "ERROR": "\033[31m",  # Red
    "CRITICAL": "\033[35m",  # Magenta
    "RESET": "\033[0m",  # Reset
  }

  def format(self, record: logging.LogRecord) -> str:
    filename = Path(record.pathname).stem

    log_color = self.COLORS.get(record.levelname, "")
    reset_color = self.COLORS["RESET"]

    formatted = f"{log_color}[{record.levelname:8}]{reset_color} {filename}:{record.lineno:<4} - {record.getMessage()}"

    if record.exc_info:
      formatted += f"\n{self.formatException(record.exc_info)}"

    return formatted


def setup_logger(name: str = "app", level: int = logging.INFO) -> logging.Logger:
  logger = logging.getLogger(name)

  if logger.handlers:
    return logger

  logger.setLevel(level)

  console_handler = logging.StreamHandler(sys.stdout)
  console_handler.setLevel(level)

  formatter = ColoredFormatter()
  console_handler.setFormatter(formatter)

  logger.addHandler(console_handler)

  return logger


logger: logging.Logger = setup_logger()


def get_logger(name: Optional[str] = None) -> logging.Logger:
  if name is None:
    return logger
  return setup_logger(name)


if __name__ == "__main__":
  test_logger = get_logger("test")

  test_logger.debug("This is a debug message")
  test_logger.info("This is an info message")
  test_logger.warning("This is a warning message")
  test_logger.error("This is an error message")
  test_logger.critical("This is a critical message")

  import logging

  other_logger = logging.getLogger("test")
  other_logger.info("Message from another part of the code")


__all__ = ["setup_logger"]
