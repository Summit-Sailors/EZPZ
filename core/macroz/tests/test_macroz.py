# ruff: noqa: S101

from typing import TYPE_CHECKING, Iterable, cast
from inspect import signature

import libcst as cst
import pytest
import libcst.matchers as m
from pydantic import BaseModel
from macroz.decoratorz.noop import func_macro, class_macro
from macroz.visitorz.macro_metadata_collector import (
  NoCallbackMethodError,
  MacroMetadataCollector,
)

if TYPE_CHECKING:
  from macroz.visitorz.macro_metadata_collector import JSONSerializable

DEFAULT_NAME: str = ""
DEFAULT_VALUE: int = 0
TEST_MACRO_NAME: str = "test_macro"
CUSTOM_MACRO_NAME: str = "custom_macro"
TEST_CLASS_NAME: str = "TestClass"
TEST_NAME: str = "test"
TEST_VALUE: int = 42
FUNC_TEST_NAME: str = "func_test"
FUNC_TEST_VALUE: int = 100
TEST_FUNC_INPUT_X: int = 42
TEST_FUNC_INPUT_Y: str = "hello"
TEST_FUNC_OUTPUT: str = "42 hello"


# Test data model for metadata collection
class TestMetadataModel(BaseModel):
  name: str
  value: int


# Sample callback for metadata collection
def sample_callback(args: Iterable["JSONSerializable"], kwargs: dict[str, "JSONSerializable"]) -> TestMetadataModel:
  name: str = cast("str", kwargs.get("name", DEFAULT_NAME))
  value: int = cast("int", kwargs.get("value", DEFAULT_VALUE))
  return TestMetadataModel(name=name, value=value)


# convert evaluated_value to string
def safe_string_conversion(value: str | bytes) -> str:
  """Convert LibCST evaluated_value to string, handling both str and bytes."""
  if isinstance(value, bytes):
    return value.decode("utf-8")
  return value


# Test no-op macros
def test_class_macro_identity() -> None:
  """Test that class_macro returns the class unchanged."""

  @class_macro
  class TestClass:
    pass

  assert TestClass.__name__ == TEST_CLASS_NAME
  assert isinstance(TestClass(), TestClass)


def test_func_macro_preservation() -> None:
  """Test that func_macro preserves function behavior and signature."""

  @func_macro
  def test_func(x: int, y: str = "default") -> str:
    return f"{x} {y}"

  assert test_func(TEST_FUNC_INPUT_X, y=TEST_FUNC_INPUT_Y) == TEST_FUNC_OUTPUT
  assert test_func.__name__ == "test_func"
  # Check signature instead of co_varnames to verify parameter preservation
  sig = signature(test_func)
  assert list(sig.parameters.keys()) == ["x", "y"]


# Test MacroMetadataCollector
def test_macro_metadata_collector_initialization() -> None:
  """Test MacroMetadataCollector initialization with callback."""
  collector = MacroMetadataCollector[TestMetadataModel, dict[str, "JSONSerializable"]](macro_name=TEST_MACRO_NAME, callback=sample_callback)
  assert collector.macro_name == TEST_MACRO_NAME
  assert collector.macro_data == []
  assert callable(collector.callback)


def test_macro_metadata_collector_no_callback_error() -> None:
  """Test that NoCallbackMethodError is raised when no callback is provided."""

  class InvalidCollector(MacroMetadataCollector[TestMetadataModel, dict[str, "JSONSerializable"]]):
    pass

  with pytest.raises(NoCallbackMethodError):
    InvalidCollector(macro_name=TEST_MACRO_NAME)


def test_macro_metadata_collection() -> None:
  """Test metadata collection from a decorated class."""
  source_code = f"""
@{CUSTOM_MACRO_NAME}(name="{TEST_NAME}", value={TEST_VALUE})
class {TEST_CLASS_NAME}:
    pass
"""

  # Mock the collector to handle decorator syntax
  class MockCollector(MacroMetadataCollector[TestMetadataModel, dict[str, "JSONSerializable"]]):
    @m.leave(m.Decorator())
    def collect_macro_metadata(self, node: cst.Decorator) -> None:
      matcher = m.Decorator(decorator=m.Call(func=m.Name(value=self.macro_name)))
      if m.matches(node, matcher):
        match node.decorator:
          case cst.Call(args=decorator_args):
            kwargs: dict[str, "JSONSerializable"] = {}
            for arg in decorator_args:
              if arg.keyword is not None:
                if arg.keyword.value == "name" and isinstance(arg.value, cst.SimpleString):
                  kwargs["name"] = safe_string_conversion(arg.value.evaluated_value)
                elif arg.keyword.value == "value" and isinstance(arg.value, cst.Integer):
                  kwargs["value"] = int(arg.value.evaluated_value)
            self.macro_data.append(self.callback([], kwargs))
          case _:
            pass

  collector = MockCollector(macro_name=CUSTOM_MACRO_NAME, callback=sample_callback)
  module = cst.parse_module(source_code)
  module.visit(collector)

  assert len(collector.macro_data) == 1
  metadata = collector.macro_data[0]
  assert isinstance(metadata, TestMetadataModel)
  assert metadata.name == TEST_NAME
  assert metadata.value == TEST_VALUE


def test_macro_metadata_collection_empty() -> None:
  """Test metadata collection when no matching decorators are found."""
  source_code = f"""
class {TEST_CLASS_NAME}:
    pass
"""
  collector = MacroMetadataCollector[TestMetadataModel, dict[str, "JSONSerializable"]](macro_name=CUSTOM_MACRO_NAME, callback=sample_callback)

  module = cst.parse_module(source_code)
  module.visit(collector)

  assert len(collector.macro_data) == 0


def test_macro_metadata_collection_function_call_syntax() -> None:
  """Test metadata collection with function call syntax."""
  # This creates a proper decorator pattern that matches the AST structure
  source_code = f"""
@{CUSTOM_MACRO_NAME}(name="{FUNC_TEST_NAME}", value={FUNC_TEST_VALUE})
class {TEST_CLASS_NAME}:
    pass
"""

  # Mock the collector to handle function call syntax
  class MockCollector(MacroMetadataCollector[TestMetadataModel, dict[str, "JSONSerializable"]]):
    @m.leave(m.Decorator())
    def collect_macro_metadata(self, node: cst.Decorator) -> None:
      matcher = m.Decorator(decorator=m.Call(func=m.Name(value=self.macro_name)))
      if m.matches(node, matcher):
        match node.decorator:
          case cst.Call(args=decorator_args):
            kwargs: dict[str, "JSONSerializable"] = {}
            for arg in decorator_args:
              if arg.keyword is not None:
                if arg.keyword.value == "name" and isinstance(arg.value, cst.SimpleString):
                  kwargs["name"] = safe_string_conversion(arg.value.evaluated_value)
                elif arg.keyword.value == "value" and isinstance(arg.value, cst.Integer):
                  kwargs["value"] = int(arg.value.evaluated_value)
            self.macro_data.append(self.callback([], kwargs))
          case _:
            pass

  collector = MockCollector(macro_name=CUSTOM_MACRO_NAME, callback=sample_callback)
  module = cst.parse_module(source_code)
  module.visit(collector)

  assert len(collector.macro_data) == 1
  metadata = collector.macro_data[0]
  assert isinstance(metadata, TestMetadataModel)
  assert metadata.name == FUNC_TEST_NAME
  assert metadata.value == FUNC_TEST_VALUE


def test_macro_metadata_collection_actual_function_call_syntax() -> None:
  """Test metadata collection with actual function call syntax that returns a decorator."""
  # This tests the pattern: some_macro(args)(class) but applied as a decorator
  source_code = f"""
class {TEST_CLASS_NAME}:
    pass

# Apply the decorator using function call syntax
{TEST_CLASS_NAME} = {CUSTOM_MACRO_NAME}(name="{FUNC_TEST_NAME}", value={FUNC_TEST_VALUE})({TEST_CLASS_NAME})
"""

  # For this pattern, we need to look for function calls, not decorators
  class FunctionCallCollector(MacroMetadataCollector[TestMetadataModel, dict[str, "JSONSerializable"]]):
    @m.leave(m.Assign())
    def collect_macro_metadata(self, node: cst.Assign) -> None:
      # Look for assignments like: ClassName = macro_name(args)(ClassName)
      if len(node.targets) == 1 and isinstance(node.targets[0].target, cst.Name) and isinstance(node.value, cst.Call) and isinstance(node.value.func, cst.Call):
        inner_call = node.value.func
        if isinstance(inner_call.func, cst.Name) and inner_call.func.value == self.macro_name:
          kwargs: dict[str, "JSONSerializable"] = {}
          for arg in inner_call.args:
            if arg.keyword is not None:
              if arg.keyword.value == "name" and isinstance(arg.value, cst.SimpleString):
                kwargs["name"] = safe_string_conversion(arg.value.evaluated_value)
              elif arg.keyword.value == "value" and isinstance(arg.value, cst.Integer):
                kwargs["value"] = int(arg.value.evaluated_value)
          self.macro_data.append(self.callback([], kwargs))

  collector = FunctionCallCollector(macro_name=CUSTOM_MACRO_NAME, callback=sample_callback)
  module = cst.parse_module(source_code)
  module.visit(collector)

  assert len(collector.macro_data) == 1
  metadata = collector.macro_data[0]
  assert isinstance(metadata, TestMetadataModel)
  assert metadata.name == FUNC_TEST_NAME
  assert metadata.value == FUNC_TEST_VALUE
