import logging

from pydantic import Field, field_validator
from pydantic_settings import BaseSettings, SettingsConfigDict

logger = logging.getLogger(__name__)


class Settings(BaseSettings):
  model_config = SettingsConfigDict(
    env_file=".env",
    case_sensitive=False,
    env_prefix="EZPZ_",
  )

  database_url: str = Field(default="", description="Database connection URL for the EZPZ registry.")

  db_host: str = Field(default="localhost", description="Database host")
  db_port: int = Field(default=5432, description="Database port")
  db_user: str = Field(default="postgres", description="Database user")
  db_password: str = Field(default="postgres", description="Database password")
  db_name: str = Field(default="postgres", description="Database name")

  admin_api_key: str = Field(default="", description="API key for administrative operations.")
  github_webhook_secret: str = Field(default="", description="Secret for GitHub webhook verification.")

  pypi_check_interval: int = Field(
    default=3600,
    description="Interval (in seconds) to check PyPI for new plugin versions.",
  )

  host: str = Field(default="127.0.0.1", description="Host address for the server to listen on.")
  port: int = Field(default=8000, description="Port for the server to listen on.")
  debug: bool = Field(default=False, description="Enable debug mode for the server.")
  secret_key: str = Field(default="", description="Secret key for application security (e.g., session management).")
  cors_origins: list[str] = Field(default=["*"], description="List of allowed CORS origins. Use '*' for all.")
  log_level: str = Field(default="INFO", description="Logging level (e.g., INFO, DEBUG, WARNING, ERROR).")

  @field_validator("cors_origins", mode="before")
  @classmethod
  def parse_cors_origins(cls, v: str | list[str]) -> list[str]:
    if isinstance(v, str):
      return [origin.strip() for origin in v.split(",") if origin.strip()]
    return v

  @field_validator("secret_key", mode="before")
  @classmethod
  def validate_secret_key(cls, v: str) -> str:
    if not v:
      import secrets

      generated_key = secrets.token_urlsafe(32)
      logger.warning("SECRET_KEY environment variable not set. Generating a random key.")
      return generated_key
    return v


settings = Settings()
