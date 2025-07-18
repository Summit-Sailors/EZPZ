# Painlezz Macroz

A lightweight Python macro system for code transformation and metadata collection, designed to work seamlessly with LibCST for static analysis and code generation. This system powers the plugin discovery mechanism in EZPZ-Pluginz.

## Overview

Painlezz Macroz provides a foundation for creating decorator-based macros that can collect metadata during static analysis without affecting runtime behavior. It's particularly useful for plugin systems, code generators, and tools that need to extract information from decorated classes and functions.

## Features

- **No-op Macros**: Decorators that preserve original functionality while enabling metadata collection
- **LibCST Integration**: Built-in visitor patterns for AST traversal and metadata extraction
- **Type-Safe**: Full type hints and generic support for robust macro definitions
- **Minimal Runtime Impact**: Macros are designed to be lightweight and non-intrusive
- **Flexible Callback System**: Support for custom metadata extraction logic

## Installation

```bash
pip install macroz
```

## Core Components

### 1. No-op Macros (`macroz/noop.py`)

The foundation of the macro system - decorators that don't change behavior but enable metadata collection:

```python
from painlezz_macroz.macroz.noop import class_macro, func_macro

# Class macro - preserves the class unchanged (identity function)
@class_macro
class MyClass:
    pass

# Function macro - preserves function behavior with proper wrapping
@func_macro
def my_function():
    return "unchanged"
```

#### Available Macros

- **`class_macro[T](cls: T) -> T`**: Identity decorator for classes - returns the class unchanged
- **`func_macro[**P, R](func: Callable[P, R]) -> Callable[P, R]`**: Wrapper decorator for functions that preserves signature and behavior using `@wraps`

**Important**: The `class_macro` is a true identity function that returns the class unchanged, while `func_macro` creates a wrapper using `functools.wraps` to preserve metadata.

### 2. Metadata Collection (`visitorz/macro_metadata_collector.py`)

A powerful LibCST visitor that extracts metadata from macro-decorated code using pattern matching:

```python
from painlezz_macroz.visitorz.macro_metadata_collector import MacroMetadataCollector
from pydantic import BaseModel
import libcst as cst

# Define your metadata model
class MyMacroData(BaseModel):
    name: str
    value: int

# Create a collector
collector = MacroMetadataCollector[MyMacroData, dict](
    macro_name="my_macro",
    callback=lambda args, kwargs: MyMacroData(
        name=kwargs["name"],
        value=kwargs["value"]
    )
)

# Parse and visit code
module = cst.parse_module(source_code)
module.visit(collector)

# Access collected metadata
for data in collector.macro_data:
    print(f"Found: {data.name} = {data.value}")
```

## How It Works with EZPZ-Pluginz

The macro system integrates seamlessly with EZPZ-Pluginz to enable plugin discovery:

### 1. Plugin Definition

```python
from ezpz_pluginz.register_plugin_macro import ezpz_plugin_collect

# The ezpz_plugin_collect function returns class_macro internally
@ezpz_plugin_collect(
    polars_ns="LazyFrame",
    attr_name="my_plugin",
    import_="from my_package import MyPlugin",
    type_hint="MyPlugin"
)
class MyPlugin:
    def custom_method(self):
        pass
```

### 2. Metadata Extraction

The `PolarsPluginCollector` extends `MacroMetadataCollector` to extract plugin information:

```python
class PolarsPluginCollector(MacroMetadataCollector[PolarsPluginMacroMetadataPD, PolarsPluginMacroKwargs]):
    def __init__(self) -> None:
        super().__init__(
            ezpz_plugin_collect.__name__,  # "ezpz_plugin_collect"
            lambda _args, kwargs: PolarsPluginMacroMetadataPD(
                import_=kwargs["import_"],
                type_hint=kwargs["type_hint"],
                attr_name=kwargs["attr_name"],
                polars_ns=EPolarsNS(kwargs["polars_ns"]),
            ),
        )
```

### 3. Function Call Support

The collector also handles function call syntax:

```python
# This syntax is also supported
ezpz_plugin_collect(
    polars_ns="DataFrame",
    attr_name="my_plugin",
    import_="from my_package import MyPlugin",
    type_hint="MyPlugin"
)(MyPluginClass)
```

## Technical Implementation Details

### Metadata Collection Process

The `MacroMetadataCollector` uses LibCST's matcher system to identify decorators:

```python
@m.leave(m.Decorator(decorator=m.Call(func=m.Name())))
def collect_macro_metadata(self, node: cst.Decorator) -> None:
    match node.decorator:
        case cst.Call(func=cst.Name(decorator_name), args=decorator_args) if decorator_name == self.macro_name:
            args: list[JSONSerializable] = []
            kwargs = cast("TMacroKwargs", {})

            for arg in decorator_args:
                # Extract literal values using ast.literal_eval
                evaled = ast.literal_eval(arg.value.value) if isinstance(arg.value, cst.SimpleString) else ast.literal_eval(dump(arg.value))

                if arg.keyword is None:
                    args.append(evaled)
                else:
                    kwargs[arg.keyword.value] = evaled

            # Create metadata instance via callback
            self.macro_data.append(self.callback(args, kwargs))
```

### Type System

The library provides comprehensive generic type support:

```python
# JSON-serializable types for macro arguments
type JSONSerializable = str | int | float | bool | None | list[JSONSerializable] | dict[str, JSONSerializable]

# Generic callback type
type TMetadataCallback[T: BaseModel, TMacroKwargs: dict[str, JSONSerializable]] =
    Callable[[Iterable[JSONSerializable], TMacroKwargs], T]

# Generic collector class (TMacroKwargs bound to Any to allow TypedDict)
class MacroMetadataCollector[T: BaseModel, TMacroKwargs: Any](m.MatcherDecoratableVisitor):
```

## Usage Patterns

### Plugin Registration (EZPZ-Pluginz Pattern)

```python
from ezpz_pluginz.register_plugin_macro import ezpz_plugin_collect

# Decorator syntax
@ezpz_plugin_collect(
    polars_ns="DataFrame",
    attr_name="advanced_ops",
    import_="from my_plugins import DataFrameAdvanced",
    type_hint="DataFrameAdvanced"
)
class DataFrameAdvanced:
    def complex_operation(self):
        pass

# Function call syntax
class SeriesUtils:
    def utility_method(self):
        pass

ezpz_plugin_collect(
    polars_ns="Series",
    attr_name="utils",
    import_="from my_plugins import SeriesUtils",
    type_hint="SeriesUtils"
)(SeriesUtils)
```

### Custom Macro System

```python
from painlezz_macroz.macroz.noop import class_macro
from painlezz_macroz.visitorz.macro_metadata_collector import MacroMetadataCollector
from pydantic import BaseModel

# Define custom metadata
class ConfigMetadata(BaseModel):
    section: str
    priority: int = 0

# Create custom macro
def config_section(**kwargs):
    return class_macro

# Create collector
collector = MacroMetadataCollector[ConfigMetadata, dict](
    "config_section",
    lambda args, kwargs: ConfigMetadata(**kwargs)
)
```

## Integration Flow in EZPZ-Pluginz

1. **Plugin Definition**: Developers use `@ezpz_plugin_collect` to mark plugin classes
2. **Code Scanning**: EZPZ-Pluginz scans configured paths for Python files
3. **AST Parsing**: LibCST parses each file into a concrete syntax tree
4. **Metadata Collection**: `PolarsPluginCollector` visits the AST and extracts plugin metadata
5. **Lockfile Generation**: Collected metadata is serialized into a YAML lockfile
6. **Type Enhancement**: LibCST transformers inject type hints into Polars classes
7. **Import Management**: Required imports are added to `TYPE_CHECKING` blocks

## Error Handling

The collector includes robust error handling:

- Graceful handling of malformed decorator syntax
- Safe literal evaluation using `ast.literal_eval`
- Fallback to LibCST's `dump` function for complex expressions
- Optional callback validation

## Supported Argument Types

The macro system supports these JSON-serializable types:

- `str`, `int`, `float`, `bool`, `None`
- `list[JSONSerializable]` (nested lists)
- `dict[str, JSONSerializable]` (nested dictionaries)

## Advanced Features

- **Pattern Matching**: Uses LibCST matchers for precise AST node identification
- **Flexible Callbacks**: Support for custom metadata extraction logic
- **Type Safety**: Full generic support with proper type bounds
- **Multiple Syntax Support**: Handles both decorator and function call patterns

## Contributing

Painlezz Macroz is part of the EZPZ ecosystem. Contributions should maintain the lightweight, type-safe approach while expanding functionality for static analysis and code generation use cases.

## License

Part of the EZPZ project - see main repository for licensing information.
