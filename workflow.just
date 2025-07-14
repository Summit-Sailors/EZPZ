set shell := ["bash", "-uc"]
set export
set dotenv-load := true

PACKAGE_NAME := env_var("PACKAGE_NAME")
PLUGIN_PATH := env_var("PLUGIN_PATH")

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

# Install security and maintenance tools
install-tools:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Installing security and maintenance tools..."
  rye install bandit
  rye install semgrep
  rye install pip-audit
  cargo install cargo-audit cargo-outdated

uninstall-tools:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Uninstalling security and maintenance tools..."
  rye uninstall bandit
  rye uninstall semgrep
  rye uninstall pip-audit
  cargo uninstall cargo-audit cargo-outdated


# Clean build artifacts
clean:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Cleaning build artifacts..."
  find . -name "*.pyc" -delete
  find . -name "__pycache__" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "dist" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "build" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "target" -type d -exec rm -rf {} + 2>/dev/null || true
  uninstall-tools
  echo "Clean complete!"


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

# Convert security reports to SARIF format
convert-sarif:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "Converting security reports to SARIF format..."
  python3 .github/scripts/security/convert_sarif.py


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
