set shell := ["bash", "-uc"]
set export
set dotenv-load

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
  cargo run -p plugins/ezpz-rust-ti stub_gen

examples:
  #!/usr/bin/env bash
  set -euo pipefail
  rye run python3 examples/ezpz_ta/ezpz_rust_ti.py


reg-gen message:
  #!/usr/bin/env bash
  set -euo pipefail
  cd core/registry/ezpz_registry/migrations
  alembic revision --autogenerate -m "{{message}}"

reg-bump:
  #!/usr/bin/env bash
  set -euo pipefail
  cd core/registry/ezpz_registry/migrations
  alembic upgrade head

reg-dev:
  #!/usr/bin/env bash
  set -euo pipefail
  cd core/registry
  rye run uvicorn ezpz_registry.main:app --host 0.0.0.0 --port 8000 --reload

reg-prod:
  #!/usr/bin/env bash
  set -euo pipefail
  cd core/registry
  rye run gunicorn ezpz_registry.main:app -w 4 -k uvicorn.workers.UvicornWorker --bind 0.0.0.0:8000






# Workflow commands:

# Environment variables with defaults
PACKAGE_NAME := env_var_or_default("PACKAGE_NAME", "")
PLUGIN_PATH := env_var_or_default("PLUGIN_PATH", "")

# plugin structure
validate-plugin:
  #!/usr/bin/env nu
  nu .github/scripts/plugins/validate-plugin.nu {{PACKAGE_NAME}} {{PLUGIN_PATH}}

build-rust:
  #!/usr/bin/env nu
  nu .github/scripts/plugins/build-rust.nu {{PLUGIN_PATH}}

# plugin tests
run-tests:
  #!/usr/bin/env nu
  nu .github/scripts/plugins/run-tests.nu {{PACKAGE_NAME}} {{PLUGIN_PATH}}

# Build plugin for publishing
build-plugin:
  #!/usr/bin/env nu
  nu .github/scripts/plugins/build-plugin.nu {{PACKAGE_NAME}} {{PLUGIN_PATH}}

validate-package:
  #!/usr/bin/env nu
  nu .github/scripts/plugins/validate-package.nu {{PLUGIN_PATH}}

publish-pypi:
  #!/usr/bin/env nu
  nu .github/scripts/plugins/publish-pypi.nu {{PACKAGE_NAME}} {{PLUGIN_PATH}}

publish-cargo:
  #!/usr/bin/env nu
  nu .github/scripts/plugins/publish-cargo.nu {{PACKAGE_NAME}} {{PLUGIN_PATH}}

# Generate workflow report
generate-report:
  #!/usr/bin/env nu
  nu .github/scripts/plugins/generate-report.nu \
    "{{env_var_or_default('OPERATION', 'automatic')}}" \
    "{{env_var_or_default('DRY_RUN', 'false')}}" \
    "{{env_var_or_default('EVENT_NAME', 'unknown')}}" \
    "{{env_var_or_default('DISCOVER_RESULT', 'unknown')}}" \
    "{{env_var_or_default('HAS_CHANGES', 'unknown')}}" \
    "{{env_var_or_default('PLUGINS_TO_REGISTER', '[]')}}" \
    "{{env_var_or_default('PLUGINS_TO_UPDATE', '[]')}}" \
    "{{env_var_or_default('TEST_RESULT', 'unknown')}}" \
    "{{env_var_or_default('REGISTER_RESULT', 'unknown')}}" \
    "{{env_var_or_default('PUBLISH_RESULT', 'unknown')}}"

# Python script recipes
analyze-plugins:
  #!/usr/bin/env python
  python .github/scripts/plugins/analyze_plugins.py

register-plugins:
  #!/usr/bin/env python
  python .github/scripts/plugins/register_plugins.py

update-plugins:
  #!/usr/bin/env python
  python .github/scripts/plugins/update_plugins.py

check-publish:
  #!/usr/bin/env python
  python .github/scripts/plugins/check_publish.py

# Dev recipes
dev-setup:
  #!/usr/bin/env bash
  set -euo pipefail
  @echo "Setting up development environment..."
  rye sync
  @echo "Development setup complete!"

clean:
  #!/usr/bin/env bash
  set -euo pipefail
  @echo "Cleaning build artifacts..."
  find . -name "*.pyc" -delete
  find . -name "__pycache__" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "dist" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "build" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "target" -type d -exec rm -rf {} + 2>/dev/null || true
  @echo "Clean complete!"

# Test a specific plugin locally
test-plugin PLUGIN_NAME:
  #!/usr/bin/env bash
  set -euo pipefail
  @echo "Testing plugin: {{PLUGIN_NAME}}"
  PACKAGE_NAME={{PLUGIN_NAME}} PLUGIN_PATH=plugins/{{PLUGIN_NAME}} just validate-plugin
  PACKAGE_NAME={{PLUGIN_NAME}} PLUGIN_PATH=plugins/{{PLUGIN_NAME}} just run-tests

# Build a specific plugin locally
build-plugin-local PLUGIN_NAME:
  #!/usr/bin/env bash
  set -euo pipefail
  @echo "Building plugin: {{PLUGIN_NAME}}"
  PACKAGE_NAME={{PLUGIN_NAME}} PLUGIN_PATH=plugins/{{PLUGIN_NAME}} just build-plugin

# Validate all plugins
validate-all:
  @echo "Validating all plugins..."
  @for plugin in plugins/*/; do \
    if [ -d "$plugin" ]; then \
        plugin_name=$(basename "$plugin"); \
        echo "Validating $plugin_name..."; \
        PACKAGE_NAME="$plugin_name" PLUGIN_PATH="$plugin" just validate-plugin; \
    fi; \
  done

# Run tests for all plugins
test-all:
  @echo "Testing all plugins..."
  @for plugin in plugins/*/; do \
    if [ -d "$plugin" ]; then \
        plugin_name=$(basename "$plugin"); \
        echo "Testing $plugin_name..."; \
        PACKAGE_NAME="$plugin_name" PLUGIN_PATH="$plugin" just run-tests; \
    fi; \
  done

# Show plugin information
info:
  @echo "Current plugin: {{PACKAGE_NAME}}"
  @echo "Plugin path: {{PLUGIN_PATH}}"
  @echo "Available plugins:"
  @ls -la plugins/ | grep "^d" | awk '{print "  - " $9}' | grep -v "^  - \.$" | grep -v "^  - \.\.$"





# Security scripts

install-tools:
  #!/usr/bin/env nu
  print "Installing security and maintenance tools..."
  rye install bandit
  rye install semgrep
  rye install pip-audit
  cargo install cargo-audit cargo-outdated

# full security audit (Python + Rust + Semgrep)
security-audit:
  #!/usr/bin/env nu
  nu .github/scripts/security/python-security.nu
  nu .github/scripts/security/rust-security.nu  
  nu .github/scripts/security/semgrep.nu

python-security:
  #!/usr/bin/env nu
  nu .github/scripts/security/python-security.nu

rust-security:
  #!/usr/bin/env nu
  nu .github/scripts/security/rust-security.nu

semgrep-scan:
  #!/usr/bin/env nu
  nu .github/scripts/security/semgrep.nu

# full dependency check (Python + Rust)
dependency-check:
  #!/usr/bin/env nu
  nu .github/scripts/security/py-deps.nu
  nu .github/scripts/security/rust-deps.nu

python-deps:
  #!/usr/bin/env nu
  nu .github/scripts/security/py-deps.nu

rust-deps:
  #!/usr/bin/env nu
  nu .github/scripts/security/rust-deps.nu

dependency-summary:
  #!/usr/bin/env nu
  nu .github/scripts/security/dep-summary.nu

# Run full code quality checks (Python + Rust)
code-quality:
  #!/usr/bin/env nu
  nu .github/scripts/security/py-quality.nu
  nu .github/scripts/security/rust-quality.nu

python-quality:
  #!/usr/bin/env nu
  nu .github/scripts/security/py-quality.nu

rust-quality:
  #!/usr/bin/env nu
  nu .github/scripts/plugins/rust-quality.nu

# Run all checks (equivalent to GitHub Actions workflow)
all-checks: security-audit dependency-check code-quality
  #!/usr/bin/env nu
  print "All security and maintenance checks completed!"

# Clean up generated reports
clean-reports:
  #!/usr/bin/env nu
  print "Cleaning up generated reports..."
  rye uninstall bandit
  rye uninstall semgrep
  rye uninstall pip-audit
  rm -f **/*_report.json
  rm -f **/audit.json
  rm -f **/cargo_outdated*.json
  rm -f **/rust_audit*.json
  rm -f main_deps.json
  rm -f dependency_summary.md
  print "Reports cleaned up!"

setup:
    #!/usr/bin/env nu
    print "Setting up project for development..."
    rye sync --all-features
    print "Project setup complete!"

# security checks suitable for CI
ci-security:
  #!/usr/bin/env nu 
  nu .github/scripts/security/python-security.nu
  nu .github/scripts/security/rust-security.nu
  nu .github/scripts/security/semgrep.nu

# dependency checks suitable for CI  
ci-deps:
  #!/usr/bin/env nu
  nu .github/scripts/security/py-deps.nu
  nu .github/scripts/security/rust-deps.nu

# code quality checks suitable for CI
ci-quality:
  #!/usr/bin/env nu
  nu .github/scripts/security/py-quality.nu
  nu .github/scripts/security/rust-quality.nu

# minimal checks for pre-commit
pre-commit:
  #!/usr/bin/env nu
  nu .github/scripts/security/py-quality.nu
  nu .github/scripts/security/rust-quality.nu

# project status
status:
  #!/usr/bin/env nu
  print "Project Security and Maintenance Status"
  print "======================================"
  print ""
  print "Lock files:"
  print $"  ezpz-lock.yaml: (if ('ezpz-lock.yaml' | path exists) { '✅' } else { '❌' })"
  print ""
  print "Components:"
  let components = [
      "core/pluginz"
      "core/macroz" 
      "core/registry"
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
  print "  just security-audit    - Run full security audit"
  print "  just dependency-check  - Check for dependency updates"
  print "  just code-quality      - Run code quality checks"
  print "  just all-checks        - Run all checks"