import json
from typing import Any, ClassVar
from dataclasses import asdict
from urllib.parse import urlencode

import httpx

from ezpz_pluginz.logger import setup_logger
from ezpz_pluginz.registry.config import (
  API_VERSION,
  REGISTRY_URL,
  HTTP_NOT_FOUND,
  REQUEST_TIMEOUT,
  HTTP_SERVER_ERROR,
  HTTP_UNAUTHORIZED,
  DEFAULT_BATCH_SIZE,
  DEFAULT_PAGE_START,
)
from ezpz_pluginz.registry.models import PluginCreate, PluginUpdate, PluginResponse, safe_deserialize_plugin  # noqa: TC001
from ezpz_pluginz.registry.exceptions import (
  PluginNotFoundError,
  PluginRegistryError,
  PluginOperationError,
  PluginRegistryAuthError,
  PluginRegistryConnectionError,
)

logger = setup_logger("Registry")


class PluginRegistryAPI:
  UNSUPPORTED_HTTP_METHOD_ERROR: ClassVar[str] = "Unsupported HTTP method: {method}"
  EMPTY_SEARCH_KEYWORD_ERROR: ClassVar[str] = "Search keyword cannot be empty"
  EMPTY_PLUGIN_ID_ERROR: ClassVar[str] = "Plugin ID cannot be empty"
  GITHUB_TOKEN_REQUIRED_ERROR: ClassVar[str] = "Authentication is required"  # noqa: S105

  def __init__(self, base_url: str = REGISTRY_URL) -> None:
    self.base_url = base_url.rstrip("/")
    self.timeout = REQUEST_TIMEOUT

  def invalid_method(self, method: str) -> None:
    raise ValueError(self.UNSUPPORTED_HTTP_METHOD_ERROR.format(method=method))

  def _make_request(
    self,
    endpoint: str,
    method: str = "POST",
    data: dict[str, Any] | None = None,
    headers: dict[str, str] | None = None,
    params: dict[str, Any] | None = None,
    *,
    use_json: bool = False,
  ) -> dict[str, Any]:
    url = f"{self.base_url}/api/{API_VERSION}{endpoint}"
    response = None
    headers = headers or {}
    request_data = data or {}

    if method == "POST" and request_data:
      if use_json:
        headers["Content-Type"] = "application/json"
      else:
        headers["Content-Type"] = "application/x-www-form-urlencoded"
        request_data = urlencode(request_data, doseq=True)

    try:
      with httpx.Client(timeout=self.timeout) as client:
        if method == "POST":
          response = client.post(url, json=data, headers=headers) if use_json else client.post(url, data=request_data, headers=headers)
        elif method == "GET":
          response = client.get(url, params=params, headers=headers)
        else:
          self.invalid_method(method)

        if response is not None:
          if response.status_code == HTTP_UNAUTHORIZED:
            raise PluginRegistryAuthError()
          if response.status_code == HTTP_NOT_FOUND:
            raise PluginNotFoundError(endpoint)
          if response.status_code >= HTTP_SERVER_ERROR:
            raise PluginRegistryError("Server_error")

          response.raise_for_status()

          if not response.content.strip():
            logger.debug(f"Empty response from {url}")
            return {}

        return response.json() if response is not None else {}

    except httpx.ConnectError as exc:
      raise PluginRegistryConnectionError(self.base_url, "connection refused") from exc
    except httpx.TimeoutException as exc:
      raise PluginRegistryConnectionError(self.base_url, f"timeout after {self.timeout}s") from exc
    except httpx.HTTPStatusError as exc:
      if exc.response.status_code not in [HTTP_UNAUTHORIZED, HTTP_NOT_FOUND]:
        raise PluginRegistryError(f"{exc.response.text}") from exc
      raise
    except (ValueError, json.JSONDecodeError) as exc:
      raise PluginRegistryError(f"{exc}") from exc

  def check_health(self) -> dict[str, Any]:
    logger.info("Checking registry health")
    return self._make_request("/health", method="POST")

  def fetch_plugins(self, *, verified_only: bool = False) -> list[PluginResponse]:
    all_plugins: list[PluginResponse] = []
    batch_size = DEFAULT_BATCH_SIZE
    page = DEFAULT_PAGE_START

    logger.info(f"Fetching plugins from registry (verified_only={verified_only})")

    while True:
      data = {
        "page": str(page),
        "page_size": str(batch_size),
        "verified_only": str(verified_only).lower(),
      }
      response = self._make_request("/plugins", data=data)

      plugins_data: list[dict[str, Any]] = response.get("plugins", [])
      if not plugins_data:
        break

      batch_plugins: list[PluginResponse] = []
      for plugin_data in plugins_data:
        plugin = safe_deserialize_plugin(plugin_data)
        if plugin:
          batch_plugins.append(plugin)

      all_plugins.extend(batch_plugins)
      logger.debug(f"Fetched page {page}: {len(batch_plugins)} plugins")

      total_pages = response.get("total_pages", DEFAULT_PAGE_START)
      if page >= total_pages:
        break

      page += 1

    logger.info(f"Successfully fetched {len(all_plugins)} plugins")
    return all_plugins

  def search_plugins(self, keyword: str) -> list[PluginResponse]:
    if not keyword.strip():
      raise ValueError(self.EMPTY_SEARCH_KEYWORD_ERROR)

    logger.info(f"Searching plugins for keyword: '{keyword}'")

    data = {"query_text": keyword}
    response = self._make_request("/plugins/search", data=data)

    plugins_data: list[dict[str, Any]] = response.get("plugins", [])
    plugins: list[PluginResponse] = []

    for plugin_data in plugins_data:
      plugin = safe_deserialize_plugin(plugin_data)
      if plugin:
        plugins.append(plugin)

    logger.info(f"Search returned {len(plugins)} plugins")
    return plugins

  def get_plugin(self, plugin_id: str) -> PluginResponse:
    if not plugin_id.strip():
      raise ValueError(self.EMPTY_PLUGIN_ID_ERROR)

    logger.info(f"Fetching plugin: {plugin_id}")

    response = self._make_request(f"/plugins/get/{plugin_id}")

    if not response:
      raise PluginNotFoundError(plugin_id)

    plugin = safe_deserialize_plugin(response)
    if not plugin:
      raise PluginRegistryError("Invalid_plugin_data")

    logger.info(f"Successfully retrieved plugin: {plugin.name}")
    return plugin

  def register_plugin(self, plugin_info: PluginCreate, github_token: str) -> PluginResponse | None:
    if not github_token.strip():
      raise ValueError(self.GITHUB_TOKEN_REQUIRED_ERROR)

    logger.info(f"Registering plugin: {plugin_info.name}")

    plugin_dict = asdict(plugin_info)
    data = {"request": {"plugin_data": plugin_dict}}
    headers = {"Authorization": f"Bearer {github_token}"}

    def _handle_registration_error(error_msg: str, plugin_name: str) -> None:
      raise PluginOperationError("register", plugin_name, error_msg)

    try:
      response = self._make_request("/plugins/register", data=data, headers=headers, use_json=True)
      plugin = safe_deserialize_plugin(response)

      if not plugin:
        error_msg = response.get("error", "Unknown registration error")
        _handle_registration_error(error_msg, plugin_info.name)

      logger.info("Successfully registered plugin")

    except Exception as e:
      error_message = (
        f"Failed to register plugin '{plugin_info.name}'.\n"
        f"Possible reasons:\n"
        f"1. Plugin name already exists (even if marked as deleted - wait for hard deletion),\n"
        f"2. Network/server error,\n "
        f"3. Invalid plugin data or authorization.\n "
        "\n"
        f"Error details: {e!s}\n"
        "\n"
      )
      logger.exception(error_message)

      return None
    return plugin

  def update_plugin(self, plugin_id: str, plugin_info: PluginUpdate, github_token: str) -> PluginResponse:
    if not plugin_id.strip():
      raise ValueError(self.EMPTY_PLUGIN_ID_ERROR)
    if not github_token.strip():
      raise ValueError(self.GITHUB_TOKEN_REQUIRED_ERROR)

    logger.info(f"Updating plugin: {plugin_id}")

    plugin_dict = {k: v for k, v in asdict(plugin_info).items() if v is not None}
    data = {"request": {"plugin_data": plugin_dict}}
    headers = {"Authorization": f"Bearer {github_token}"}

    response = self._make_request(f"/plugins/update/{plugin_id}", data=data, headers=headers, use_json=True)

    plugin = safe_deserialize_plugin(response)
    if not plugin:
      error_msg = response.get("error", "Unknown update error")
      raise PluginOperationError("update", plugin_id, error_msg)

    logger.info(f"Successfully updated plugin: {plugin_id}")
    return plugin

  def delete_plugin(self, plugin_id: str, github_token: str) -> PluginResponse:
    if not plugin_id.strip():
      raise ValueError(self.EMPTY_PLUGIN_ID_ERROR)
    if not github_token.strip():
      raise ValueError(self.GITHUB_TOKEN_REQUIRED_ERROR)

    logger.info(f"Deleting plugin: {plugin_id}")

    data: dict[str, Any] = {}
    headers = {"Authorization": f"Bearer {github_token}"}

    response = self._make_request(f"/plugins/delete/{plugin_id}", data=data, headers=headers, use_json=False)

    plugin = safe_deserialize_plugin(response)
    if not plugin:
      error_msg = response.get("error", "Unknown deletion error")
      raise PluginOperationError("delete", plugin_id, error_msg)
    return plugin
