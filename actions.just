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


# Clean artifacts
clean:
  #!/usr/bin/env bash
  set -euo pipefail
  find . -name "*.pyc" -delete
  find . -name "__pycache__" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "dist" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "build" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "target" -type d -exec rm -rf {} + 2>/dev/null || true
  just uninstall-tools

# Plugin operations
analyze-plugins:
  #!/usr/bin/env bash
  set -euo pipefail
  python3 .github/scripts/plugins/plugin_manager.py analyze

register-plugins:
  #!/usr/bin/env bash
  set -euo pipefail
  python3 .github/scripts/plugins/plugin_manager.py register --dry-run=$DRY_RUN

update-plugins:
  #!/usr/bin/env bash
  set -euo pipefail
  python3 .github/scripts/plugins/plugin_manager.py update --dry-run=$DRY_RUN

check-publish:
  #!/usr/bin/env bash
  set -euo pipefail
  python3 .github/scripts/plugins/plugin_manager.py check-publish --package-name=$PACKAGE_NAME

validate-plugin:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/plugin_ops.nu validate $PACKAGE_NAME $PLUGIN_PATH

build-plugin:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/plugin_ops.nu build $PACKAGE_NAME $PLUGIN_PATH

test-plugin:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/plugin_ops.nu test $PACKAGE_NAME $PLUGIN_PATH

publish-plugin:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/plugin_ops.nu publish $PACKAGE_NAME $PLUGIN_PATH --dry-run=$DRY_RUN

# Batch operations
validate-all:
  #!/usr/bin/env bash
  set -euo pipefail
  for plugin in plugins/*/; do
    if [ -d "$plugin" ]; then
      plugin_name=$(basename "$plugin")
      PACKAGE_NAME="$plugin_name" PLUGIN_PATH="$plugin" just validate-plugin
    fi
  done

test-all:
  #!/usr/bin/env bash
  set -euo pipefail
  for plugin in plugins/*/; do
    if [ -d "$plugin" ]; then
      plugin_name=$(basename "$plugin")
      PACKAGE_NAME="$plugin_name" PLUGIN_PATH="$plugin" just test-plugin
    fi
  done

# Generate report
generate-report:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/generate-report.nu \
    "$OPERATION" \
    "$DRY_RUN" \
    "$EVENT_NAME" \
    "$DISCOVER_RESULT" \
    "$HAS_CHANGES" \
    "$PLUGINS_TO_REGISTER" \
    "$PLUGINS_TO_UPDATE" \
    "$TEST_RESULT" \
    "$REGISTER_RESULT" \
    "$PUBLISH_RESULT"
    
