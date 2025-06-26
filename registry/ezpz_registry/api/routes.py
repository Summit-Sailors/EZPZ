import json
import logging
from typing import TYPE_CHECKING, Any
from datetime import datetime, timezone

from fastapi import Query, APIRouter, HTTPException
from sqlalchemy.exc import IntegrityError

from ezpz_registry.api.schema import HealthResponse, PluginResponse, WebhookResponse, PluginListResponse, PluginSearchResponse
from ezpz_registry.services.pypi import PyPIService
from ezpz_registry.services.plugins import PluginService

if TYPE_CHECKING:
  from fastapi import Request, BackgroundTasks

  from ezpz_registry.api.deps import ApiKeyVerified, DatabaseSession, WebhookVerified
  from ezpz_registry.api.schema import PluginRegistrationRequest

logger = logging.getLogger(__name__)
router = APIRouter()


@router.get("/health", response_model=HealthResponse)
async def health_check() -> HealthResponse:
  return HealthResponse(status="healthy", timestamp=datetime.now(timezone.utc), version="1.0.0", database="connected")


@router.get("/plugins", response_model=PluginListResponse)
async def list_plugins(
  session: "DatabaseSession",
  page: int = Query(1, ge=1, description="Page number"),
  page_size: int = Query(50, ge=1, le=100, description="Items per page"),
  verified_only: bool = Query(False, description="Show only verified plugins"),
) -> PluginListResponse:
  plugins, total = await PluginService.list_plugins(session, page=page, page_size=page_size, verified_only=verified_only)

  total_pages = (total + page_size - 1) // page_size

  return PluginListResponse(
    plugins=[PluginResponse.model_validate(plugin) for plugin in plugins], total=total, page=page, page_size=page_size, total_pages=total_pages
  )


@router.get("/plugins/search", response_model=PluginSearchResponse)
async def search_plugins(
  session: "DatabaseSession",
  q: str = Query(..., min_length=1, description="Search query"),
  page: int = Query(1, ge=1, description="Page number"),
  page_size: int = Query(50, ge=1, le=100, description="Items per page"),
) -> PluginSearchResponse:
  plugins, total = await PluginService.search_plugins(session, query=q, page=page, page_size=page_size)

  return PluginSearchResponse(plugins=[PluginResponse.model_validate(plugin) for plugin in plugins], query=q, total=total)


@router.get("/plugins/{plugin_id}", response_model=PluginResponse)
async def get_plugin(session: "DatabaseSession", plugin_id: int) -> PluginResponse:
  plugin = await PluginService.get_plugin_by_id(session, plugin_id)

  if not plugin:
    raise HTTPException(status_code=404, detail="Plugin not found")

  return PluginResponse.model_validate(plugin)


@router.post("/plugins/register", response_model=dict[str, str])
async def register_plugin(
  request: "PluginRegistrationRequest", session: "DatabaseSession", background_tasks: "BackgroundTasks", api_key: "ApiKeyVerified"
) -> dict[str, str]:
  try:
    plugin = await PluginService.create_plugin(session, request.plugin, submitted_by="api")

    # Start background verification
    background_tasks.add_task(verify_plugin_background, plugin.package_name)

    return {
      "success": "true",
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
async def admin_verify_plugin(plugin_id: int, session: "DatabaseSession", api_key: "ApiKeyVerified") -> dict[str, str]:
  """Manually verify a plugin (admin only)."""
  plugin = await PluginService.get_plugin_by_id(session, plugin_id)

  if not plugin:
    raise HTTPException(status_code=404, detail="Plugin not found")

  success = await PluginService.verify_plugin(session, plugin.package_name)

  if success:
    return {"success": "true", "message": f"Plugin '{plugin.name}' verified successfully"}
  raise HTTPException(status_code=400, detail="Failed to verify plugin")


@router.delete("/admin/plugins/{plugin_id}", response_model=dict[str, str])
async def admin_delete_plugin(plugin_id: int, session: "DatabaseSession", api_key: "ApiKeyVerified") -> dict[str, str]:
  """Delete a plugin (admin only)."""
  success = await PluginService.delete_plugin(session, plugin_id)

  if success:
    return {"success": "true", "message": "Plugin deleted successfully"}
  raise HTTPException(status_code=404, detail="Plugin not found")


@router.post("/webhooks/github", response_model=WebhookResponse)
async def github_webhook(request: "Request", background_tasks: "BackgroundTasks", body: "WebhookVerified") -> WebhookResponse:
  try:
    webhook_data: dict[str, Any] = json.loads(body.decode())
  except json.JSONDecodeError:
    raise HTTPException(status_code=400, detail="Invalid JSON payload") from None

  # Handle release events
  if webhook_data.get("action") == "published" and "release" in webhook_data:
    background_tasks.add_task(handle_release_webhook, webhook_data)
    return WebhookResponse(status="received", message="Release webhook processed")

  # Handle push events to main branch
  if webhook_data.get("ref") == "refs/heads/main" and "commits" in webhook_data:
    background_tasks.add_task(handle_push_webhook, webhook_data)
    return WebhookResponse(status="received", message="Push webhook processed")

  return WebhookResponse(status="ignored", message="Webhook event not handled")


async def verify_plugin_background(package_name: str) -> None:
  """Background task to verify a plugin package."""
  try:
    async with PyPIService() as pypi_service:
      from ezpz_registry.db.connection import db_manager

      async with db_manager.aget_sa_session() as session:
        await pypi_service.verify_single_plugin(session, package_name)
  except Exception as e:
    logger.exception(f"Background verification failed for {package_name}: {e}")


async def handle_release_webhook(webhook_data: dict[str, Any]) -> None:
  """Handle GitHub release webhook."""
  try:
    from ezpz_registry.db.connection import db_manager

    # Safely extract release and repository data
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
      else:
        logger.info(f"No plugin found for repository {repo_name}")

  except Exception as e:
    logger.exception(f"Error handling release webhook: {e}")


async def handle_push_webhook(webhook_data: dict[str, Any]) -> None:
  try:
    from ezpz_registry.db.connection import db_manager

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
      message: str = commit.get("message", "")
      if message:
        commit_messages.append(message)

      added_files: list[str] = commit.get("added", [])
      modified_files_in_commit: list[str] = commit.get("modified", [])
      modified_files.extend(added_files + modified_files_in_commit)

    plugin_files_modified = any(
      file_path
      for file_path in modified_files
      if any(pattern in file_path.lower() for pattern in ["setup.py", "pyproject.toml", "requirements.txt", "__init__.py", "plugin.py", "manifest.json"])
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

      if not plugin_found:
        logger.info(f"No plugin found for repository {repo_name}")

        if plugin_files_modified:
          logger.info(f"Repository {repo_name} has plugin-related files but no registered plugin. Consider checking if this should be registered.")

  except Exception as e:
    logger.exception(f"Error handling push webhook: {e}")
