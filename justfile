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


registry-gen message:
  #!/usr/bin/env bash
  set -euo pipefail
  cd core/registry/ezpz_registry/migrations
  alembic revision --autogenerate -m "{{message}}"

registry-bump:
  #!/usr/bin/env bash
  set -euo pipefail
  cd core/registry/ezpz_registry/migrations
  alembic upgrade head

registry-run-dev:
  #!/usr/bin/env bash
  set -euo pipefail
  cd core/registry
  rye run uvicorn ezpz_registry.main:app --host 0.0.0.0 --port 8000 --reload

registry-run-prod:
  #!/usr/bin/env bash
  set -euo pipefail
  cd core/registry
  rye run gunicorn ezpz_registry.main:app -w 4 -k uvicorn.workers.UvicornWorker --bind 0.0.0.0:8000