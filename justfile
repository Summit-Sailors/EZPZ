set shell := ["bash", "-uc"]
set export
set dotenv-load := true

# =============================================================================
# ENVIRONMENT VARIABLES (For Workflows)
# =============================================================================

# Plugin-specific environment variables
PACKAGE_NAME := env_var("PACKAGE_NAME")
PLUGIN_PATH := env_var("PLUGIN_PATH")

# Workflow environment variables
OPERATION := env_var("OPERATION")
DRY_RUN := env_var("DRY_RUN")
EVENT_NAME := env_var("EVENT_NAME")
DISCOVER_RESULT := env_var("DISCOVER_RESULT")
TEST_RESULT := env_var("TEST_RESULT")
REGISTER_RESULT := env_var("REGISTER_RESULT")
PUBLISH_RESULT := env_var("PUBLISH_RESULT")
HAS_CHANGES := env_var("HAS_CHANGES")
PLUGINS_TO_REGISTER := env_var("PLUGINS_TO_REGISTER")
PLUGINS_TO_UPDATE := env_var("PLUGINS_TO_UPDATE")

default:
  @just --choose --justfile {{justfile()}}

web:
  #!/usr/bin/env bash
  set -euo pipefail
  dx serve --platform web -p app

desktop:
  #!/usr/bin/env bash
  set -euo pipefail
  dx serve --platform desktop -p app

mobile:
  #!/usr/bin/env bash
  set -euo pipefail
  dx serve --platform mobile -p app

clear:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo clean
  rm *.lock
  rm -rf .venv

stub-gen:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo run -p ezpz-rust-ti stub_gen

examples:
  #!/usr/bin/env bash
  set -euo pipefail
  rye run python3 examples/ezpz_ta/volatility.py


# EZPZ Plugin Management and Security Recipes

# =============================================================================
# SETUP AND CLEANUP COMMANDS
# =============================================================================

# Install security and maintenance tools
install-tools:
  #!/usr/bin/env bash
  set -euo pipefail
  print "Installing security and maintenance tools..."
  rye install bandit
  rye install semgrep
  rye install pip-audit
  cargo install cargo-audit cargo-outdated

# Clean build artifacts and reports
clean:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Cleaning build artifacts..."
  find . -name "*.pyc" -delete
  find . -name "__pycache__" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "dist" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "build" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "target" -type d -exec rm -rf {} + 2>/dev/null || true
  echo "Clean complete!"

# Clean up generated reports
clean-reports:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Cleaning up generated reports..."
  rm -f **/*_report.json
  rm -f **/audit.json
  rm -f **/cargo_outdated*.json
  rm -f **/rust_audit*.json
  rm -f main_deps.json
  rm -f dependency_summary.md
  echo "Reports cleaned up!"

# =============================================================================
# PLUGIN DISCOVERY AND ANALYSIS
# =============================================================================

# Analyze plugins and generate lists for workflows
analyze-plugins:
  #!/usr/bin/env bash
  set -euo pipefail
  python3 .github/scripts/plugins/analyze_plugins.py

# Register new plugins with registry
register-plugins:
  #!/usr/bin/env bash
  set -euo pipefail
  python3 .github/scripts/plugins/register_plugins.py

# Update existing plugins in registry
update-plugins:
  #!/usr/bin/env bash
  set -euo pipefail
  python3 .github/scripts/plugins/update_plugins.py

# Check if plugin needs publishing
check-publish:
  #!/usr/bin/env bash
  set -euo pipefail
  python3 .github/scripts/plugins/check_publish.py

# =============================================================================
# PLUGIN VALIDATION AND TESTING
# =============================================================================

# Validate plugin structure
validate-plugin:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/validate-plugin.nu {{PACKAGE_NAME}} {{PLUGIN_PATH}}

# Build Rust components
build-rust:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/build-rust.nu {{PLUGIN_PATH}}

# Run plugin tests
run-tests:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/run-tests.nu {{PACKAGE_NAME}} {{PLUGIN_PATH}}

# Validate all plugins in the repository
validate-all:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Validating all plugins..."
  for plugin in plugins/*/; do \
    if [ -d "$plugin" ]; then \
        plugin_name=$(basename "$plugin"); \
        echo "Validating $plugin_name..."; \
        PACKAGE_NAME="$plugin_name" PLUGIN_PATH="$plugin" just validate-plugin; \
    fi; \
  done

# Run tests for all plugins
test-all:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Testing all plugins..."
  for plugin in plugins/*/; do \
    if [ -d "$plugin" ]; then \
        plugin_name=$(basename "$plugin"); \
        echo "Testing $plugin_name..."; \
        PACKAGE_NAME="$plugin_name" PLUGIN_PATH="$plugin" just run-tests; \
    fi; \
  done

# =============================================================================
# PLUGIN BUILDING AND PUBLISHING
# =============================================================================

# Build plugin for distribution
build-plugin:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/build-plugin.nu {{PACKAGE_NAME}} {{PLUGIN_PATH}}

# Validate built package
validate-package:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/validate-package.nu {{PLUGIN_PATH}}

# Publish plugin to PyPI
publish-pypi:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/publish-pypi.nu {{PACKAGE_NAME}} {{PLUGIN_PATH}}

# Publish Rust crate to crates.io
publish-cargo:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/publish-cargo.nu {{PACKAGE_NAME}} {{PLUGIN_PATH}}


convert-sarif:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Converting security reports to SARIF format..."
  python3 .github/scripts/security/convert_sarif.py

# =============================================================================
# REPORTING AND STATUS
# =============================================================================

# Generate workflow report
generate-report:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/generate-report.nu \
    "{{OPERATION}}" \
    "{{DRY_RUN}}" \
    "{{EVENT_NAME}}" \
    "{{DISCOVER_RESULT}}" \
    "{{HAS_CHANGES}}" \
    "{{PLUGINS_TO_REGISTER}}" \
    "{{PLUGINS_TO_UPDATE}}" \
    "{{TEST_RESULT}}" \
    "{{REGISTER_RESULT}}" \
    "{{PUBLISH_RESULT}}"

# Show plugin information
plugin-info:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Current plugin: {{PACKAGE_NAME}}"
  echo "Plugin path: {{PLUGIN_PATH}}"
  echo "Available plugins:"
  ls -la plugins/ | grep "^d" | awk '{print "  - " $9}' | grep -v "^  - \.$" | grep -v "^  - \.\.$"

# Show project status
status:
  #!/usr/bin/env nu
  print "EZPZ Project Security and Maintenance Status"
  print "============================================"
  print ""
  print "Lock files:"
  print $"  ezpz-lock.yaml: (if ('ezpz-lock.yaml' | path exists) { '✅' } else { '❌' })"
  print ""
  print "Components:"
  let components = [
      "core/pluginz"
      "core/macroz"
      "examples"
      "plugins/ezpz-rust-ti"
      "stubz"
  ]
  for component in $components {
      let has_pyproject = ($component | path join "pyproject.toml" | path exists)
      let has_cargo = ($component | path join "Cargo.toml" | path exists)
      let type = if $has_pyproject and $has_cargo {
          "Python+Rust"
      } else if $has_pyproject {
          "Python"
      } else if $has_cargo {
          "Rust"
      } else {
          "Unknown"
      }
      print $"  ($component): ($type)"
  }
  print ""
  print "Available commands:"
  print "  just setup             - Set up development environment"
  print "  just security-audit    - Run full security audit"
  print "  just dependency-check  - Check for dependency updates"
  print "  just code-quality      - Run code quality checks"
  print "  just all-checks        - Run all checks"
  print "  just validate-all      - Validate all plugins"
  print "  just test-all          - Test all plugins"
  print "  just clean             - Clean build artifacts"
  print "  just status            - Show this status"

