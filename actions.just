set shell := ["bash", "-uc"]
set export
set dotenv-load := true

analyze-plugins:
  #!/usr/bin/env bash
  set -euo pipefail
  python3 .github/scripts/plugins/plugin_manager.py analyze

test-plugin-pipeline package_name plugin_path:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "🔍 Testing plugin: {{package_name}} at {{plugin_path}}"
  
  nu .github/scripts/plugins/plugin_ops.nu test-pipeline "{{package_name}}" "{{plugin_path}}"

register-update-plugins plugins_to_register plugins_to_update dry_run:
  #!/usr/bin/env bash
  set -euo pipefail
  
  if [[ "{{plugins_to_register}}" != "[]" ]]; then
    echo "📝 Registering new plugins..."
    python3 .github/scripts/plugins/plugin_manager.py register \
      --plugins='{{plugins_to_register}}' \
      --dry-run='{{dry_run}}'
  fi
  
  if [[ "{{plugins_to_update}}" != "[]" ]]; then
    echo "🔄 Updating existing plugins..."
    python3 .github/scripts/plugins/plugin_manager.py update \
      --plugins='{{plugins_to_update}}' \
      --dry-run='{{dry_run}}'
  fi

publish-plugin package_name plugin_path dry_run plugins_to_register plugins_to_update:
  #!/usr/bin/env bash
  set -euo pipefail
  
  if ! python3 .github/scripts/plugins/plugin_manager.py check-publish \
    --package-name="{{package_name}}" \
    --plugins-to-register='{{plugins_to_register}}' \
    --plugins-to-update='{{plugins_to_update}}'; then
    echo "ℹ️  Plugin {{package_name}} does not need publishing"
    exit 0
  fi
  
  echo "📦 Publishing plugin: {{package_name}}"
  
  nu .github/scripts/plugins/plugin_ops.nu publish "{{package_name}}" "{{plugin_path}}" --dry-run='{{dry_run}}'

validate-plugin package_name plugin_path:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/plugin_ops.nu validate "{{package_name}}" "{{plugin_path}}"

build-plugin package_name plugin_path:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/plugin_ops.nu build "{{package_name}}" "{{plugin_path}}"

test-plugin package_name plugin_path:
  #!/usr/bin/env bash
  set -euo pipefail
  nu .github/scripts/plugins/plugin_ops.nu test "{{package_name}}" "{{plugin_path}}"

validate-all:
  #!/usr/bin/env bash
  set -euo pipefail
  
  for plugin_dir in plugins/*/; do
    if [[ -d "$plugin_dir" ]]; then
      plugin_name=$(basename "$plugin_dir")
      echo "🔍 Validating $plugin_name..."
      just validate-plugin "$plugin_name" "$plugin_dir"
    fi
  done

test-all:
  #!/usr/bin/env bash
  set -euo pipefail
  
  for plugin_dir in plugins/*/; do
    if [[ -d "$plugin_dir" ]]; then
      plugin_name=$(basename "$plugin_dir")
      echo "🧪 Testing $plugin_name..."
      just test-plugin-pipeline "$plugin_name" "$plugin_dir"
    fi
  done

clean:
  #!/usr/bin/env bash
  set -euo pipefail
  echo "🧹 Cleaning build artifacts..."
  
  find . -name "*.pyc" -delete
  find . -name "__pycache__" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "dist" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "build" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "*.egg-info" -type d -exec rm -rf {} + 2>/dev/null || true
  find . -name "target" -type d -exec rm -rf {} + 2>/dev/null || true
  
  echo "✅ Clean completed"

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
  