# EZPZ Stubz

Type-safe wrappers for PyO3-Polars integration, providing seamless conversion between Rust and Python Polars objects with proper type stub generation.

## Overview

EZPZ Stubz provides wrapper types that enable PyO3 extensions to work seamlessly with Polars objects while maintaining proper type information for Python static analysis tools. It bridges the gap between Rust's type system and Python's type hints, ensuring that your PyO3-based Polars extensions have excellent IDE support and type safety.

## Features

- **Type-Safe Wrappers**: Transparent wrappers for a few Polars types
- **Automatic Stub Generation**: Integration with `pyo3_stub_gen` for type hints
- **Zero-Runtime Cost**: Wrapper types compile away, leaving only the original Polars objects
- **Seamless Conversion**: Automatic conversion between wrapped and unwrapped types
- **IDE Support**: Full type completion and error detection in IDEs

## Installation

```toml
 cargo add ezpz-stubz
```

## Available Wrappers

EZPZ Stubz provides wrappers for major Polars types:

- `PyDfStubbed` - DataFrame wrapper
- `PyLfStubbed` - LazyFrame wrapper
- `PySeriesStubbed` - Series wrapper
- `PyExprStubbed` - Expression wrapper

## Type Stub Generation

When you use EZPZ Stubz wrappers, the generated `.pyi` files will have proper Polars type hints:

## Contributing

EZPZ Stubz is part of the EZPZ ecosystem. When contributing:

1. Maintain wrapper consistency across all Polars types
2. Ensure zero-cost abstraction principles
3. Test stub generation output
4. Update documentation for new wrapper types

## License

Part of the EZPZ project - see main repository for licensing information.
