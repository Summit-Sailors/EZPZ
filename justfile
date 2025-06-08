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
  cargo run -p ezpz-guiz stub_gen
