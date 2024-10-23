import logging
from enum import Enum

logger = logging.getLogger(__name__)


class MacroPhase(Enum):
  GENERATE = 1
  BUILDTIME = 2
  RUNTIME = 3
