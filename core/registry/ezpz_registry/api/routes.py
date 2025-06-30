import json
import logging
from typing import TYPE_CHECKING, Any
from datetime import datetime, timezone

from fastapi import Query, Depends, APIRouter, HTTPException
from sqlalchemy.exc import IntegrityError

from ezpz_registry.api.deps import verify_api_key, get_database_session
from ezpz_registry.api.schema import HealthResponse, PluginResponse, WebhookResponse, PluginListResponse, PluginSearchResponse
from ezpz_registry.db.connection import db_manager
from ezpz_registry.services.pypi import PyPIService
from ezpz_registry.services.plugins import PluginService

if TYPE_CHECKING:
  from uuid import UUID

  from fastapi import Request, BackgroundTasks

  from ezpz_registry.api.deps import ApiKeyVerified, DatabaseSession, WebhookVerified
  from ezpz_registry.api.schema import PluginRegistrationRequest

logger = logging.getLogger(__name__)
router = APIRouter()


@router.get("/health", response_model=HealthResponse)
async def health_check(session: "DatabaseSession" = Depends(get_database_session)) -> HealthResponse:
  return HealthResponse(status="healthy", timestamp=datetime.now(timezone.utc), version="1.0.0", database="connected")


@router.get("/plugins", response_model=PluginListResponse)
async def list_plugins(
  session: "DatabaseSession" = Depends(get_database_session),
  page: int = Query(1, ge=1, description="Page number"),
  page_size: int = Query(50, ge=1, le=100, description="Items per page"),
  *,
  verified_only: bool = Query(default=False, description="Show only verified plugins"),
) -> PluginListResponse:
  try:
    plugins, total = await PluginService.list_plugins(session, page=page, page_size=page_size, verified_only=verified_only)

    total_pages = (total + page_size - 1) // page_size

    return PluginListResponse(
      plugins=[PluginResponse.model_validate(plugin) for plugin in plugins], total=total, page=page, page_size=page_size, total_pages=total_pages
    )
  except Exception as e:
    logger.exception(f"Error listing plugins: {e}")
    raise HTTPException(status_code=500, detail="Failed to retrieve plugins") from None


@router.get("/plugins/search", response_model=PluginSearchResponse)
async def search_plugins(
  session: "DatabaseSession" = Depends(get_database_session),
  q: str = Query(..., min_length=1, description="Search query"),
  page: int = Query(1, ge=1, description="Page number"),
  page_size: int = Query(50, ge=1, le=100, description="Items per page"),
) -> PluginSearchResponse:
  try:
    plugins, total = await PluginService.search_plugins(
      session,
      query_text=q,
      page=page,
      page_size=page_size,
    )

    return PluginSearchResponse(plugins=[PluginResponse.model_validate(plugin) for plugin in plugins], query=q, total=total)
  except Exception as e:
    logger.exception(f"Error searching plugins: {e}")
    raise HTTPException(status_code=500, detail="Failed to search plugins") from None


@router.get("/plugins/{plugin_id}", response_model=PluginResponse)
async def get_plugin(
  plugin_id: "UUID",
  session: "DatabaseSession" = Depends(get_database_session),
) -> PluginResponse:
  try:
    plugin = await PluginService.get_plugin_by_id(session, plugin_id)

    if not plugin:
      raise HTTPException(status_code=404, detail="Plugin not found")

    return PluginResponse.model_validate(plugin)
  except HTTPException:
    raise
  except Exception as e:
    logger.exception(f"Error retrieving plugin {plugin_id}: {e}")
    raise HTTPException(status_code=500, detail="Failed to retrieve plugin") from None


@router.post("/plugins/register", response_model=dict[str, Any])
async def register_plugin(
  request: "PluginRegistrationRequest",
  background_tasks: "BackgroundTasks",
  api_key: "ApiKeyVerified" = Depends(verify_api_key),
  session: "DatabaseSession" = Depends(get_database_session),
) -> dict[str, Any]:
  try:
    plugin = await PluginService.create_plugin(session, request.plugin, submitted_by="api")
    logger.info(f"here is the plugin we just created: {plugin}")
    # Start background verification
    background_tasks.add_task(verify_plugin_background, plugin.package_name)

    logger.info(f"here is the plugin generated: {plugin}")

    return {
      "success": True,
      "message": f"Plugin '{request.plugin.name}' registered successfully",
      "plugin_id": str(plugin.id),
      "note": "Plugin will be verified automatically when published to PyPI",
    }

  except IntegrityError:
    await session.rollback()
    raise HTTPException(status_code=400, detail="Plugin with this name or package name already exists") from None
  except Exception as e:
    await session.rollback()
    logger.exception(f"Error registering plugin: {e}")
    raise HTTPException(status_code=500, detail="Internal server error") from None


@router.post("/admin/plugins/{plugin_id}/verify", response_model=dict[str, str])
async def admin_verify_plugin(
  plugin_id: "UUID",
  api_key: "ApiKeyVerified" = Depends(verify_api_key),
  session: "DatabaseSession" = Depends(get_database_session),
) -> dict[str, str]:
  """Manually verify a plugin (admin only)."""
  try:
    plugin = await PluginService.get_plugin_by_id(session, plugin_id)

    if not plugin:
      raise HTTPException(status_code=404, detail="Plugin not found")

    success = await PluginService.verify_plugin(session, plugin.package_name)

    if success:
      return {"success": "true", "message": f"Plugin '{plugin.name}' verified successfully"}
    raise HTTPException(status_code=400, detail="Failed to verify plugin")
  except HTTPException:
    raise
  except Exception as e:
    logger.exception(f"Error verifying plugin {plugin_id}: {e}")
    raise HTTPException(status_code=500, detail="Failed to verify plugin") from None


@router.post("/webhooks/github", response_model=WebhookResponse)
async def github_webhook(request: "Request", background_tasks: "BackgroundTasks", body: "WebhookVerified") -> WebhookResponse:
  try:
    body_str = body.decode("utf-8") if isinstance(body, bytes) else str(body)

    webhook_data: dict[str, Any] = json.loads(body_str)
  except (json.JSONDecodeError, UnicodeDecodeError) as e:
    logger.exception(f"Invalid webhook payload: {e}")
    raise HTTPException(status_code=400, detail="Invalid JSON payload") from None

  try:
    # Handle release events
    if webhook_data.get("action") == "published" and "release" in webhook_data:
      background_tasks.add_task(handle_release_webhook, webhook_data)
      return WebhookResponse(status="received", message="Release webhook processed")

    # Handle push events to main branch
    if webhook_data.get("ref") == "refs/heads/main" and "commits" in webhook_data:
      background_tasks.add_task(handle_push_webhook, webhook_data)
      return WebhookResponse(status="received", message="Push webhook processed")

    return WebhookResponse(status="ignored", message="Webhook event not handled")
  except Exception as e:
    logger.exception(f"Error processing webhook: {e}")
    raise HTTPException(status_code=500, detail="Failed to process webhook") from None


@router.delete("/admin/plugins/{plugin_id}", response_model=dict[str, str])
async def admin_delete_plugin(
  plugin_id: "UUID",
  api_key: "ApiKeyVerified" = Depends(verify_api_key),
  session: "DatabaseSession" = Depends(get_database_session),
  *,
  confirm: bool = Query(default=False, description="Confirmation flag to prevent accidental deletion"),
) -> dict[str, str]:
  # Require explicit confirmation
  if not confirm:
    raise HTTPException(status_code=400, detail="Deletion requires confirmation. Add ?confirm=true to the request.")

  plugin = await PluginService.get_plugin_by_id(session, plugin_id)

  if not plugin:
    raise HTTPException(status_code=404, detail="Plugin not found")

  logger.warning(f"Admin deletion requested for plugin: {plugin.name} (package: {plugin.package_name}) by API key: {api_key.key_id}")

  success = await PluginService.delete_plugin(session, plugin.id)

  if success:
    logger.info(f"Plugin '{plugin.name}' successfully deleted by admin")
    return {"success": "true", "message": f"Plugin '{plugin.name}' deleted successfully", "deleted_plugin": plugin.name, "deleted_package": plugin.package_name}

  raise HTTPException(status_code=500, detail="Failed to delete plugin")


async def verify_plugin_background(package_name: str) -> None:
  """Background task to verify a plugin package."""
  if not package_name or not isinstance(package_name, str):
    logger.error("Invalid package name provided for background verification")
    return

  try:
    async with db_manager.aget_sa_session() as session, PyPIService() as pypi_service:
      await pypi_service.verify_single_plugin(session, package_name)
      logger.info(f"Successfully verified plugin: {package_name}")
  except Exception as e:
    logger.exception(f"Background verification failed for {package_name}: {e}")


async def handle_release_webhook(webhook_data: dict[str, Any]) -> None:
  """Handle GitHub release webhook."""
  if not webhook_data or not isinstance(webhook_data, dict):
    logger.error("Invalid webhook data provided to handle_release_webhook")
    return

  try:
    # Safely extract release and repository data with proper None checks
    release: dict[str, Any] = webhook_data.get("release") or {}
    repository: dict[str, Any] = webhook_data.get("repository") or {}

    repo_name: str = repository.get("name", "")
    tag_name: str = release.get("tag_name", "")

    if not repo_name or not tag_name:
      logger.warning("Missing repository name or tag name in release webhook")
      return

    # Try to find plugin by repository name pattern
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
            # Update version from tag
            version = tag_name.lstrip("v")  # Remove 'v' prefix
            await PluginService.update_plugin_version(session, package_name, version)

            # Verify the plugin
            async with PyPIService() as pypi_service:
              await pypi_service.verify_single_plugin(session, package_name)

            logger.info(f"Updated plugin {package_name} to version {version}")
            break
        except Exception as plugin_error:
          logger.exception(f"Error processing plugin {package_name}: {plugin_error}")
          continue
      else:
        logger.info(f"No plugin found for repository {repo_name}")

  except Exception as e:
    logger.exception(f"Error handling release webhook: {e}")


async def handle_push_webhook(webhook_data: dict[str, Any]) -> None:
  """Handle GitHub push webhook."""
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

    # Extract commit information for analysis
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

      # Ensure we're working with lists
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

              # re-verify the plugin
              try:
                async with PyPIService() as pypi_service:
                  await pypi_service.verify_single_plugin(session, package_name)

                logger.info(f"Successfully re-verified plugin {package_name}")
              except Exception as verify_error:
                logger.exception(f"Failed to re-verify plugin {package_name}: {verify_error}")
            else:
              logger.info(f"No re-verification needed for plugin {package_name}")

            break
        except Exception as plugin_error:
          logger.exception(f"Error processing plugin {package_name}: {plugin_error}")
          continue

      if not plugin_found:
        logger.info(f"No plugin found for repository {repo_name}")

        if plugin_files_modified:
          logger.info(f"Repository {repo_name} has plugin-related files but no registered plugin. Consider checking if this should be registered.")

  except Exception as e:
    logger.exception(f"Error handling push webhook: {e}")
