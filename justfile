set shell := ["bash", "-uc"]
set export
set dotenv-load := true

mod actions

default:
  @just --choose --justfile {{justfile()}}

stub-gen:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo run -p ezpz-rust-ti stub_gen

examples:
  #!/usr/bin/env bash
  set -euo pipefail
  uv run python3 examples/ezpz_tiz/src/volatility.py

clear:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo clean
  find . -type d -name "__pycache__" -exec rm -rf {} +
  rm -f *.lock
  rm -rf .venv
