import logging
from typing import TYPE_CHECKING, Callable, Awaitable, AsyncGenerator
from contextlib import asynccontextmanager

import structlog
from fastapi import FastAPI, HTTPException
from fastapi.responses import JSONResponse
from fastapi.middleware.cors import CORSMiddleware

from ezpz_registry.config import settings
from ezpz_registry.api.routes import router
from ezpz_registry.api.schema import ErrorResponse
from ezpz_registry.db.connection import db_manager
from ezpz_registry.services.pypi import verification_service

if TYPE_CHECKING:
  from fastapi import Request, Response

structlog.configure(
  processors=[
    structlog.stdlib.filter_by_level,
    structlog.stdlib.add_logger_name,
    structlog.stdlib.add_log_level,
    structlog.stdlib.PositionalArgumentsFormatter(),
    structlog.processors.TimeStamper(fmt="iso"),
    structlog.processors.StackInfoRenderer(),
    structlog.processors.format_exc_info,
    structlog.processors.UnicodeDecoder(),
    structlog.processors.JSONRenderer(),
  ],
  context_class=dict,
  logger_factory=structlog.stdlib.LoggerFactory(),
  wrapper_class=structlog.stdlib.BoundLogger,
  cache_logger_on_first_use=True,
)

logging.basicConfig(level=getattr(logging, settings.log_level.upper()), format="%(message)s")
logger = structlog.get_logger()


@asynccontextmanager
async def lifespan(app: FastAPI) -> AsyncGenerator[None, None]:
  logger.info("Starting EZPZ Plugin Registry")

  db_manager.initialize()
  logger.info("Database initialized")

  await verification_service.start()
  logger.info("PyPI verification service started")

  yield

  logger.info("Shutting down EZPZ Plugin Registry")

  await verification_service.stop()
  logger.info("PyPI verification service stopped")

  await db_manager.close()
  logger.info("Database connections closed")


app = FastAPI(
  title="EZPZ Plugin Registry",
  description="Central registry for EZPZ ecosystem plugins",
  version="1.0.0",
  lifespan=lifespan,
  docs_url="/docs" if settings.debug else None,
  redoc_url="/redoc" if settings.debug else None,
)

app.add_middleware(
  CORSMiddleware,
  allow_origins=settings.cors_origins,
  allow_credentials=True,
  allow_methods=["GET", "POST", "PUT", "DELETE"],
  allow_headers=["*"],
)


@app.exception_handler(HTTPException)
async def http_exception_handler(request: "Request", exc: HTTPException) -> JSONResponse:
  logger.error("HTTP exception occurred", status_code=exc.status_code, detail=exc.detail, path=request.url.path, method=request.method)

  return JSONResponse(status_code=exc.status_code, content=ErrorResponse(error=exc.detail, timestamp=datetime.now(timezone.utc)).model_dump())


@app.exception_handler(Exception)
async def general_exception_handler(request: "Request", exc: Exception) -> JSONResponse:
  logger.error("Unhandled exception occurred", error=str(exc), path=request.url.path, method=request.method, exc_info=True)

  return JSONResponse(
    status_code=500,
    content=ErrorResponse(error="Internal server error", detail=str(exc) if settings.debug else None, timestamp=datetime.now(timezone.utc)).model_dump(),
  )


# request logging middleware
@app.middleware("http")
async def log_requests(request: "Request", call_next: Callable[["Request"], Awaitable["Response"]]) -> "Response":
  start_time = time.time()

  response = await call_next(request)

  process_time = time.time() - start_time

  logger.info(
    "Request processed",
    method=request.method,
    path=request.url.path,
    status_code=response.status_code,
    process_time=round(process_time, 4),
    client_ip=request.client.host if request.client else None,
  )

  return response


app.include_router(router, prefix="/api/v1")


@app.get("/")
async def root() -> dict[str, str]:
  return {"name": "EZPZ Plugin Registry", "version": "1.0.0", "status": "running", "docs": "/docs" if settings.debug else "disabled"}


if __name__ == "__main__":
  import time
  from datetime import datetime, timezone

  import uvicorn

  uvicorn.run(
    "ezpz_registry.main:app",
    host=settings.host,
    port=settings.port,
    reload=settings.debug,
    log_level=settings.log_level.lower(),
  )
