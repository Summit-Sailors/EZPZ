# EZPZ

A toolkit for extending Polars with custom plugins and type safety. EZPZ is also tailored to bridge the gap between Rust performance and Python developer experience in the Polars Ecosystem.

## 📦 Core Components

### 🔌 [EZPZ-Pluginz](./pluginz/)

_The foundation of the EZPZ ecosystem_

A powerful tool that provides comprehensive type hinting and IDE support for Polars plugins, dramatically enhancing the development experience for custom Polars extensions.

**Key Features:**

- Full type safety for Polars plugins
- Hot reloading with automatic type hint updates pointing directly to plugin implementations
- **Site-packages integration**: Seamlessly load and manage plugins from installed packages
- **IDE support**: Autocompletion, inline documentation and error detection
- **Multiple syntax support**: Decorator and function call patterns for plugin discovery
- Support for DataFrame, LazyFrame, Series, and Expression plugins
- Reversible modifications with safe backups

```bash
pip install ezpz_pluginz
ezplugins mount  # Enable plugin support
```

### 🦀 [EZPZ Stubz](./stubz/)

_Type-safe PyO3-Polars wrappers_

Provides wrapper types that enable PyO3 extensions to work seamlessly with Polars objects while maintaining proper type information.

**Key Features:**

- Transparent wrappers for Polars types
- Automatic stub generation with `pyo3_stub_gen`
- Zero-runtime cost abstractions
- Full IDE support

```toml
[dependencies]
ezpz-stubz = "*"
```

### 📈 [EZPZ Rust Technical Analysis](./ezpz-rust-ti/)

_Production-ready technical analysis plugin_

A comprehensive technical analysis library showcasing the EZPZ plugin system with 70+ indicators powered by Rust.

**Key Features:**

- 70+ technical indicators
- Polars native integration
- Rust-powered performance
- Full type safety

```bash
pip install ezpz-rust-ti
ezplugins mount
```

## 📦 Suppoting Libraries

### 🔧 [Painlezz Macroz](./macroz/)

_Lightweight Python macro system powering plugin discovery_

A lightweight Python macro system for code transformation and metadata collection, built on LibCST for static analysis and code generation.

**Note**: This component is experimental and may evolve significantly as the Python static analysis ecosystem develops, particularly with upcoming tools like Astral.

**Key Features:**

- No-op macros that preserve runtime behavior
- LibCST integration for AST analysis
- Type-safe metadata collection
- Flexible callback system

```bash
pip install painlezz-macroz
```

## 🏗️ Architecture Overview

EZPZ follows a modular architecture designed aroung the Polars ecosystem:

```table
┌──────────────────────────────────────────────┐
│                EZPZ Ecosystem                │
├──────────────────────────────────────────────┤
│            Plugin Development Layer          │
│  ┌─────────────────┐  ┌─────────────────┐    │
│  │   EZPZ-Pluginz  │  │ Painlezz-Macroz │    │
│  │  (Type System)  │  │ (Macro System)  │    │
│  └─────────────────┘  └─────────────────┘    │
├──────────────────────────────────────────────┤
│            Runtime Integration Layer         │
│  ┌─────────────────┐  ┌─────────────────┐    │
│  │   EZPZ-Stubz    │  │  Plugin Runtime │    │
│  │ (PyO3 Wrappers) │  │   Integration   │    │
│  └─────────────────┘  └─────────────────┘    │
├──────────────────────────────────────────────┤
│               Application Layer              │
│  ┌─────────────────┐  ┌─────────────────┐    │
│  │ EZPZ-Rust-TI    │  │  Custom Plugins │    │
│  │(Tech Analysis)  │  │  (User-defined) │    │
│  └─────────────────┘  └─────────────────┘    │
├──────────────────────────────────────────────┤
│                  Polars Core                 │
└──────────────────────────────────────────────┘
```

## 🚀 Quick Start

### 1. Install the Plugin System

```bash
pip install ezpz_pluginz
```

### 2. Create Your First Plugin

```python
# my_plugin.py
from ezpz_pluginz.register_plugin_macro import ezpz_plugin_collect

@ezpz_plugin_collect(
    polars_ns="DataFrame",
    attr_name="my_operations",
    import_="from my_plugin import MyDataFramePlugin",
    type_hint="MyDataFramePlugin"
)
class MyDataFramePlugin:
    def custom_transform(self, multiplier: float):
        """Custom transformation with full type safety"""
        return self._df.with_columns(
            [pl.col(col) * multiplier for col in self._df.columns]
        )
```

### 3. Configure Plugin Discovery

```toml
# ezpz.toml
[ezpz_pluginz]
name = "my-polars-project"
include = [
    "src/plugins/",
    "my_plugin.py"
]
site_customize = true
```

### 4. Mount and Use

```bash
ezplugins mount  # Enable the plugin system
```

```python
import polars as pl

df = pl.DataFrame({"a": [1, 2, 3], "b": [4, 5, 6]})
result = df.my_operations.custom_transform(2.0)  # Full IDE support!
```

## 🎯 Use Cases

### For Plugin Developers

- **Type-Safe Development**: Build Polars plugins with type checking
- **Amazing IDE Experience**: Enjoy autocompletion and error detection
- **Easy Distribution**: Publish plugins that integrate seamlessly with the ecosystem

### For Data Scientists

- **Extended Functionality**: Access powerful extensions like technical analysis
- **Familiar Interface**: Work with enhanced Polars using the same API patterns
- **Performance**: Benefit from Rust-powered implementations

### For Library Authors

- **Integration Framework**: Build upon EZPZ's plugin architecture
- **Type Safety**: Leverage PyO3 wrappers for robust Rust-Python integration
- **Ecosystem Compatibility**: Ensure your extensions work with existing tools

## 📋 Installation Matrix

| Component           | Purpose            | Installation                  |
| ------------------- | ------------------ | ----------------------------- |
| **EZPZ-Pluginz**    | Core plugin system | `pip install ezpz_pluginz`    |
| **EZPZ-Rust-TI**    | Technical analysis | `pip install ezpz-rust-ti`    |
| **EZPZ-Stubz**      | PyO3 type wrappers | `cargo add ezpz-stubz`        |
| **Painlezz-Macroz** | Macro system       | `pip install painlezz-macroz` |

## 🔧 Development Setup

```bash
# Clone the repository
git clone https://github.com/Summit-Sailors/EZPZ.git
cd EZPZ

# Install development dependencies
pip install -e ./pluginz[dev]
pip install -e ./macroz[dev]

# Install Rust components
cargo build --workspace

# Run tests
pytest pluginz/tests/
cargo test --workspace
```

### Component-Specific Guidelines

- **Pluginz**: Focus on type safety and IDE integration
- **Rust-TI**: Maintain performance while expanding indicator coverage
- **Stubz**: Ensure zero-cost abstractions and complete type coverage
- **Macroz**: Consider future static analysis tool compatibility

### 🎯 Roadmap

- Official Polars team blessing ([tracking issue](https://github.com/pola-rs/polars/issues/14475))
- Plugin marketplace and discovery
- More showcase plugins
- Advanced debugging tools

## 🤝 Contributing

We welcome contributions to any part of the EZPZ ecosystem! Each component has its own contribution guidelines:

- **Plugin System**: Focus on type safety and developer experience
- **Macro System**: Maintain lightweight, LibCST-based approach
- **Stubz**: Ensure zero-cost abstractions and proper stub generation
- **Showcase Plugins**: Demonstrate best practices and real-world usage

## 📚 Documentation

- [EZPZ-Pluginz Documentation](./pluginz/README.md)
- [Painlezz Macroz Documentation](./macroz/README.md)
- [EZPZ Stubz Documentation](./stubz/README.md)
- [Technical Analysis Plugin](./ezpz-rust-ti/README.md)
- [Examples and Tutorials](./examples/README.md)

## 🙏 Acknowledgments

- **[Polars](https://pola.rs/)** - The amazing DataFrame library that makes this all possible
- **[PyO3](https://pyo3.rs/)** - Rust bindings for Python enabling seamless integration
- **[LibCST](https://libcst.readthedocs.io/)** - Concrete syntax trees for Python code transformation
- **[rust_ti](https://crates.io/crates/rust_ti)** - Technical analysis algorithms powering our indicators

## 💖 Support

For support and sponsorship opportunities, visit our Polar page:

<a href="https://polar.sh/summitsailors/subscriptions">
<picture>
<source media="(prefers-color-scheme: dark)" srcset="https://polar.sh/embed/tiers.svg?org=summitsailors&darkmode"/>
<img alt="Subscription Tiers on Polar" src="https://polar.sh/embed/tiers.svg?org=summitsailors"/>
</picture>
</a>

## 📄 License

This project is licensed under the MIT License. See LICENSE file for details.

---

**EZPZ** - Making Polars plugin development EZPZ! 🚀
