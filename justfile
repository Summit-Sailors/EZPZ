set shell := ["bash", "-uc"]
set export
set dotenv-load := true

mod workflow

default:
  @just --choose --justfile {{justfile()}}

stub-gen:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo run -p ezpz-rust-ti stub_gen

examples:
  #!/usr/bin/env bash
  set -euo pipefail
  rye run python3 examples/ezpz_ta/volatility.py

clear:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo clean
  rm -f *.lock
  rm -rf .venv
