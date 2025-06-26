"""API dependencies for authentication and database access."""

import hmac
import hashlib
from typing import TYPE_CHECKING, Annotated

from fastapi import Header, Depends, HTTPException
from fastapi.security import HTTPBearer
from sqlalchemy.ext.asyncio import AsyncSession

from ezpz_registry.config import settings
from ezpz_registry.db.connection import db_manager

if TYPE_CHECKING:
  from fastapi import Request
  from fastapi.security import HTTPAuthorizationCredentials

security = HTTPBearer()


async def get_database_session():
  async with db_manager.aget_sa_session() as session:
    yield session


async def verify_api_key(credentials: "HTTPAuthorizationCredentials" = Depends(security)) -> str:
  if not settings.admin_api_key or credentials.credentials != settings.admin_api_key:
    raise HTTPException(status_code=401, detail="Invalid API key", headers={"WWW-Authenticate": "Bearer"})
  return credentials.credentials


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
ApiKeyVerified = Annotated[str, Depends(verify_api_key)]
WebhookVerified = Annotated[bytes, Depends(verify_webhook_signature)]
