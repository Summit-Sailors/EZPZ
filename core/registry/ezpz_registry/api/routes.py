# type: ignore[B008]
# ruff: noqa: B008
import json
import logging
from typing import TYPE_CHECKING, Any
from datetime import datetime, timezone

from fastapi import Query, Depends, APIRouter, HTTPException
from sqlalchemy.exc import IntegrityError

from ezpz_registry.api.deps import verify_github_pat, get_database_session
from ezpz_registry.api.schema import HealthResponse, PluginResponse, WebhookResponse, PluginListResponse, PluginSearchResponse
from ezpz_registry.db.connection import db_manager
from ezpz_registry.services.pypi import PyPIService
from ezpz_registry.services.plugins import PluginService

if TYPE_CHECKING:
  from uuid import UUID

  from fastapi import Request, BackgroundTasks

  from ezpz_registry.api.deps import DatabaseSession
  from ezpz_registry.api.schema import PluginUpdate, PluginRegistrationRequest

logger = logging.getLogger(__name__)
router = APIRouter()


@router.get("/health", response_model=HealthResponse)
async def health_check(session: "DatabaseSession" = Depends(get_database_session)) -> HealthResponse:
  return HealthResponse(status="healthy", timestamp=datetime.now(timezone.utc), version="1.0.0", database="connected")


@router.get("/plugins", response_model=PluginListResponse)
async def list_plugins(
  session: "DatabaseSession" = Depends(get_database_session),
  page: int = Query(1, ge=1, description="Page number"),
  page_size: int = Query(100, ge=1, le=1000, description="Items per page"),
  *,
  verified_only: bool = Query(default=False, description="Show only verified plugins"),
) -> PluginListResponse:
  try:
    plugins, total = await PluginService.list_plugins(session, page=page, page_size=page_size, verified_only=verified_only)
    total_pages = (total + page_size - 1) // page_size
    return PluginListResponse(
      plugins=[PluginResponse.model_validate(plugin) for plugin in plugins], total=total, page=page, page_size=page_size, total_pages=total_pages
    )
  except Exception:
    logger.exception("Error listing plugins")
    raise HTTPException(status_code=500, detail="Failed to retrieve plugins") from None


@router.get("/plugins/search", response_model=PluginSearchResponse)
async def search_plugins(
  session: "DatabaseSession" = Depends(get_database_session),
  q: str = Query(..., min_length=1, description="Search query"),
) -> PluginSearchResponse:
  try:
    plugins, total = await PluginService.search_plugins(session, query_text=q)
    return PluginSearchResponse(plugins=[PluginResponse.model_validate(plugin) for plugin in plugins], query=q, total=total)
  except Exception:
    logger.exception("Error searching plugins")
    raise HTTPException(status_code=500, detail="Failed to search plugins") from None


@router.get("/plugins/{plugin_id}", response_model=PluginResponse)
async def get_plugin(
  plugin_id: "UUID",
  session: "DatabaseSession" = Depends(get_database_session),
) -> PluginResponse:
  try:
    plugin = await PluginService.get_plugin_by_id(session, plugin_id)
    return validate_plugin_exists(plugin)
  except HTTPException:
    raise
  except Exception:
    logger.exception(f"Error retrieving plugin {plugin_id}")
    raise HTTPException(status_code=500, detail="Failed to retrieve plugin") from None


def validate_plugin_exists(plugin: "PluginResponse | None") -> PluginResponse:
  if not plugin:
    raise HTTPException(status_code=404, detail="Plugin not found")
  return PluginResponse.model_validate(plugin)


@router.put("/plugins/{plugin_id}", response_model=dict[str, Any])
async def update_plugin(
  plugin_id: "UUID",
  update_data: "PluginUpdate",
  session: "DatabaseSession" = Depends(get_database_session),
  *,
  verified: bool = Depends(verify_github_pat),
) -> dict[str, Any]:
  try:
    existing_plugin = await PluginService.get_plugin_by_id(session, plugin_id)
    validate_existing_plugin(existing_plugin)

    updated_plugin = await PluginService.update_plugin(session, plugin_id, update_data)
    validate_update_success(updated_plugin)

    logger.info(f"Plugin '{existing_plugin.name}' (ID: {plugin_id}) updated successfully")
    return {
      "success": True,
      "message": f"Plugin '{existing_plugin.name}' updated successfully",
      "plugin_id": str(plugin_id),
      "updated_fields": [field for field, value in update_data.model_dump(exclude_unset=True).items() if value is not None],
    }
  except HTTPException:
    raise
  except IntegrityError:
    await session.rollback()
    logger.exception(f"Integrity error updating plugin {plugin_id}")
    raise HTTPException(status_code=400, detail="Plugin with this name or package name already exists") from None
  except Exception:
    await session.rollback()
    logger.exception(f"Error updating plugin {plugin_id}")
    raise HTTPException(status_code=500, detail="Failed to update plugin") from None


def validate_existing_plugin(existing_plugin: "PluginResponse | None") -> None:
  if not existing_plugin:
    raise HTTPException(status_code=404, detail="Plugin not found")


def validate_update_success(updated_plugin: "PluginUpdate | None") -> None:
  if not updated_plugin:
    raise HTTPException(status_code=500, detail="Failed to update plugin")


@router.post("/plugins/register", response_model=dict[str, Any])
async def register_plugin(
  request: "PluginRegistrationRequest",
  background_tasks: "BackgroundTasks",
  session: "DatabaseSession" = Depends(get_database_session),
  *,
  verified: bool = Depends(verify_github_pat),
) -> dict[str, Any]:
  try:
    plugin = await PluginService.create_plugin(session, request.plugin, submitted_by="api")
    background_tasks.add_task(verify_plugin_background, plugin.package_name)

    logger.info(f"Plugin '{request.plugin.name}' registered successfully with ID: {plugin.id}")
    return {
      "success": True,
      "message": f"Plugin '{request.plugin.name}' registered successfully",
      "plugin_id": str(plugin.id),
      "note": "Plugin will be verified automatically when published to PyPI",
    }
  except IntegrityError:
    await session.rollback()
    raise HTTPException(status_code=400, detail="Plugin with this name or package name already exists") from None
  except Exception:
    await session.rollback()
    logger.exception("Error registering plugin")
    raise HTTPException(status_code=500, detail="Internal server error") from None


@router.post("/webhooks/github", response_model=WebhookResponse)
async def github_webhook(request: "Request", background_tasks: "BackgroundTasks") -> WebhookResponse:
  try:
    body = await request.body()
    body_str = body.decode("utf-8")
    webhook_data: dict[str, Any] = json.loads(body_str)
  except (json.JSONDecodeError, UnicodeDecodeError):
    logger.exception("Invalid webhook payload")
    raise HTTPException(status_code=400, detail="Invalid JSON payload") from None

  try:
    if webhook_data.get("action") == "published" and "release" in webhook_data:
      background_tasks.add_task(handle_release_webhook, webhook_data)
      return WebhookResponse(status="received", message="Release webhook processed")

    if webhook_data.get("ref") == "refs/heads/main" and "commits" in webhook_data:
      background_tasks.add_task(handle_push_webhook, webhook_data)
      return WebhookResponse(status="received", message="Push webhook processed")

    return WebhookResponse(status="ignored", message="Webhook event not handled")
  except Exception:
    logger.exception("Error processing webhook")
    raise HTTPException(status_code=500, detail="Failed to process webhook") from None


@router.delete("/plugins/{plugin_id}", response_model=dict[str, Any])
async def delete_plugin(
  plugin_id: "UUID",
  session: "DatabaseSession" = Depends(get_database_session),
  *,
  verified: bool = Depends(verify_github_pat),
) -> dict[str, Any]:
  plugin = await PluginService.get_plugin_by_id(session, plugin_id)
  if not plugin:
    raise HTTPException(status_code=404, detail="Plugin not found")

  logger.warning(f"Admin deletion requested for plugin: {plugin.name} (package: {plugin.package_name})")
  success = await PluginService.delete_plugin(session, plugin.id)

  if success:
    logger.info(f"Plugin '{plugin.name}' successfully deleted")
    return {"success": True, "message": f"Plugin '{plugin.name}' deleted successfully", "deleted_plugin": plugin.name, "deleted_package": plugin.package_name}

  raise HTTPException(status_code=500, detail="Failed to delete plugin")


async def verify_plugin_background(package_name: str) -> None:
  if not package_name or not isinstance(package_name, str):
    logger.error("Invalid package name provided for background verification")
    return

  try:
    async with db_manager.aget_sa_session() as session, PyPIService() as pypi_service:
      await pypi_service.verify_single_plugin(session, package_name)
      logger.info(f"Successfully verified plugin: {package_name}")
  except Exception:
    logger.exception(f"Background verification failed for {package_name}")


async def handle_release_webhook(webhook_data: dict[str, Any]) -> None:
  if not webhook_data or not isinstance(webhook_data, dict):
    logger.error("Invalid webhook data provided to handle_release_webhook")
    return

  try:
    release: dict[str, Any] = webhook_data.get("release") or {}
    repository: dict[str, Any] = webhook_data.get("repository") or {}
    repo_name: str = repository.get("name", "")
    tag_name: str = release.get("tag_name", "")

    if not repo_name or not tag_name:
      logger.warning("Missing repository name or tag name in release webhook")
      return

    possible_package_names: list[str] = [
      repo_name,
      repo_name.replace("-", "_"),
      f"ezpz-{repo_name}",
      f"ezpz_{repo_name}",
    ]

    async with db_manager.aget_sa_session() as session:
      for package_name in possible_package_names:
        try:
          plugin = await PluginService.get_plugin_by_package_name(session, package_name)
          if plugin:
            version = tag_name.lstrip("v")
            await PluginService.update_plugin_version(session, package_name, version)

            async with PyPIService() as pypi_service:
              await pypi_service.verify_single_plugin(session, package_name)

            logger.info(f"Updated plugin {package_name} to version {version}")
            break
        except Exception:
          logger.exception(f"Error processing plugin {package_name}")
          continue
      else:
        logger.info(f"No plugin found for repository {repo_name}")
  except Exception:
    logger.exception("Error handling release webhook")


async def handle_push_webhook(webhook_data: dict[str, Any]) -> None:  # noqa: PLR0915
  if not webhook_data or not isinstance(webhook_data, dict):
    logger.error("Invalid webhook data provided to handle_push_webhook")
    return

  try:
    repository: dict[str, Any] = webhook_data.get("repository") or {}
    commits: list[dict[str, Any]] = webhook_data.get("commits") or []
    pusher: dict[str, Any] = webhook_data.get("pusher") or {}

    repo_name: str = repository.get("name", "")
    repo_full_name: str = repository.get("full_name", "")
    commit_count = len(commits)
    pusher_name: str = pusher.get("name", "unknown")

    if not repo_name:
      logger.warning("Missing repository name in push webhook")
      return

    logger.info(f"Received push webhook for {repo_full_name} with {commit_count} commits by {pusher_name}")

    commit_messages: list[str] = []
    modified_files: list[str] = []

    for commit in commits:
      if not isinstance(commit, dict):
        continue

      message: str = commit.get("message", "")
      if message:
        commit_messages.append(message)

      added_files: list[str] = commit.get("added", [])
      modified_files_in_commit: list[str] = commit.get("modified", [])

      if isinstance(added_files, list):
        modified_files.extend(added_files)
      if isinstance(modified_files_in_commit, list):
        modified_files.extend(modified_files_in_commit)

    plugin_files_modified = any(
      file_path
      for file_path in modified_files
      if isinstance(file_path, str)
      and any(pattern in file_path.lower() for pattern in ["setup.py", "pyproject.toml", "requirements.txt", "__init__.py", "plugin.py", "manifest.json"])
    )

    possible_package_names: list[str] = [
      repo_name,
      repo_name.replace("-", "_"),
      f"ezpz-{repo_name}",
      f"ezpz_{repo_name}",
    ]

    async with db_manager.aget_sa_session() as session:
      plugin_found = False
      for package_name in possible_package_names:
        try:
          plugin = await PluginService.get_plugin_by_package_name(session, package_name)
          if plugin:
            plugin_found = True
            logger.info(f"Found plugin {package_name} for repository {repo_name}")

            should_reverify = plugin_files_modified or any(
              keyword in " ".join(commit_messages).lower() for keyword in ["version", "release", "update", "fix", "plugin"]
            )

            if should_reverify:
              logger.info(f"Triggering re-verification for plugin {package_name} due to relevant changes")
              try:
                async with PyPIService() as pypi_service:
                  await pypi_service.verify_single_plugin(session, package_name)
                logger.info(f"Successfully re-verified plugin {package_name}")
              except Exception:
                logger.exception(f"Failed to re-verify plugin {package_name}")
            else:
              logger.info(f"No re-verification needed for plugin {package_name}")
            break
        except Exception:
          logger.exception(f"Error processing plugin {package_name}")
          continue

      if not plugin_found:
        logger.info(f"No plugin found for repository {repo_name}")
        if plugin_files_modified:
          logger.info(f"Repository {repo_name} has plugin-related files but no registered plugin. Consider checking if this should be registered.")
  except Exception:
    logger.exception("Error handling push webhook")
