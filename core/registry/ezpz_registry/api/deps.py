# type: ignore[B008]

import os
import hmac
import hashlib
import logging
from typing import TYPE_CHECKING, Annotated, AsyncGenerator

import structlog
from fastapi import Header, Depends, HTTPException
from fastapi.security import HTTPBearer
from sqlalchemy.ext.asyncio import AsyncSession

from ezpz_registry.config import settings
from ezpz_registry.db.connection import db_manager

if TYPE_CHECKING:
  from fastapi import Request


logging.basicConfig(level=getattr(logging, settings.log_level.upper()), format="%(message)s")
logger = structlog.get_logger()

security = HTTPBearer()

EXPECTED_GITHUB_PAT = os.getenv("GITHUB_PAT", "")


async def get_database_session() -> AsyncGenerator[AsyncSession, None]:
  async with db_manager.aget_sa_session() as session:
    yield session


def verify_github_pat(authorization: str = Header(None)) -> bool:
  if not authorization:
    raise HTTPException(status_code=401, detail="Authorization header required")

  if not EXPECTED_GITHUB_PAT:
    raise HTTPException(status_code=500, detail="Server configuration error: GitHub PAT not configured")

  try:
    scheme, token = authorization.split(" ", 1)
    if scheme.lower() != "bearer":
      raise HTTPException(status_code=401, detail="Invalid authorization scheme")
  except ValueError:
    raise HTTPException(status_code=401, detail="Invalid authorization header format") from None

  if token != EXPECTED_GITHUB_PAT:
    raise HTTPException(status_code=403, detail="Invalid GitHub PAT")

  return True


async def verify_webhook_signature(request: "Request", x_hub_signature_256: str = Header(None)) -> bytes:
  if not settings.github_webhook_secret:
    raise HTTPException(status_code=501, detail="GitHub webhooks not configured")

  if not x_hub_signature_256:
    raise HTTPException(status_code=401, detail="Missing webhook signature")

  body = await request.body()

  expected_signature = "sha256=" + hmac.new(settings.github_webhook_secret.encode(), body, hashlib.sha256).hexdigest()

  if not hmac.compare_digest(x_hub_signature_256, expected_signature):
    raise HTTPException(status_code=401, detail="Invalid webhook signature")

  return body


# Type aliases for dependency injection
DatabaseSession = Annotated[AsyncSession, Depends(get_database_session)]
WebhookVerified = Annotated[bytes, Depends(verify_webhook_signature)]
