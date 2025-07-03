class PluginRegistryError(Exception):
  def __init__(self, message: str) -> None:
    super().__init__(message)


class PluginRegistryConnectionError(Exception):
  def __init__(self, base_url: str, reason: str = "connection failed") -> None:
    super().__init__(f"Unable to connect to registry at {base_url}: {reason}")
    self.base_url = base_url
    self.reason = reason


class PluginRegistryAuthError(Exception):
  def __init__(self, message: str = "Authentication failed - invalid or expired token") -> None:
    super().__init__(message)


class PluginNotFoundError(Exception):
  def __init__(self, resource: str) -> None:
    super().__init__(f"Resource not found: {resource}")
    self.resource = resource


class PluginOperationError(Exception):
  def __init__(self, operation: str, plugin_name: str, reason: str) -> None:
    super().__init__(f"Failed to {operation} plugin '{plugin_name}': {reason}")
    self.operation = operation
    self.plugin_name = plugin_name
    self.reason = reason


class PluginValidationError(Exception):
  def __init__(self, message: str) -> None:
    super().__init__(message)
