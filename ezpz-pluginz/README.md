# EZPZ-Pluginz

This package provides type hinting and IDE support for plugins to the Polars package, enhancing the development experience.

## Installation

```bash
pip install polar-patch
```

## Problem It Solves

Polars is a fast DataFrame library for Python, but it lacks a way to provide type hints with type checker and IDE support for custom plugins. The polars maintainers have no plans to fill this gap from within polars itself. So Summit Sailors is stepping in to help.

## Motivation

With this package, developers can:

- Write more robust and maintainable polars plugins.
- Utilize IDE Type Checker features such as autocompletion and inline documentation.
- Extend the polars ecosystem with more incentive to create new plugins

## How does it work?

1. PP parses your ezpz_pluginz.toml
2. scans files and folders you listed in ur toml
3. uses [libCST](https://libcst.readthedocs.io/en/latest/) to extract the needed info about your plugins.
4. generates a lockfile for all the plugin data it extracted
5. creates a backup of the files to be modified
6. uses a copy of the backup fresh each run
7. applies the libCST transformer to add the attribute with type hint onto the corresponding Polars class
8. adds the corresponding import for your plugin into polars in a type checking block

![Lockfile](images/lockfile.png)

![Added Import](images/attr_type_hint_import.png)

![Added Attribute](images/attr_type_hint_added.png)

## Notes

- It is important to note that while this is minimally invasive, it is monkey patching the executing interpreters polars package.
- libCST uses concrete syntax trees, thus the polars file is well preserved.

## Beta Blockers

- ~~callable form of `pl.api`~~
- ~~install plugins from site-packages~~
- ~~basic logging~~
- inital functional hypothesis testing setup
- basic exception handling
- ~~unpin 3.12.4 to ^3.12~~

## Stable Blockers

- some maturity
- The blessing of the polars team for the approach on [issue](https://github.com/pola-rs/polars/issues/14475)

## Features

- automatic "hot reloading" since the type hint points directly to the implementation
- loads plugins from site-packages and generates a lockfile

## Configuration

To specify paths to be scanned for plugins, create a ezpz_pluginz.toml file in your project root.
(VSC IDE Support in Development)

```toml
[ezpz_pluginz]
include = ["path/to/your/plugin1.py", "path/to/your/polars/plugin/folder"]
```

## Usage

To use the CLI tool provided by this package, run the following command:

```bash
pp mount
```

## Undoing Changes

If you need to undo the changes made by this package, simply:

```bash
pp unmount
```

---

<a href="https://polar.sh/summitsailors/subscriptions">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://polar.sh/embed/tiers.svg?org=summitsailors&darkmode"/>
    <img alt="Subscription Tiers on Polar" src="https://polar.sh/embed/tiers.svg?org=summitsailors"/>
  </picture>
</a>
