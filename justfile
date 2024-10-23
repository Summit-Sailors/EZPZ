set shell := ["bash", "-uc"]
set export
set dotenv-load

default:
  @just --choose --justfile {{justfile()}}

help:
  #!/usr/bin/env bash
  set -euo pipefail
  echo DEFAULT:
  just --list

zoom *args:
  #!/usr/bin/env bash
  set -euo pipefail
  source "$HOME/.rye/env"
  rye run python painlezz-spiderz/painlezz_spiderz/zoom_zoom.py {{args}}

be:
  #!/usr/bin/env bash
  set -euo pipefail
  source "$HOME/.rye/env"
  rye run uvicorn pysilo_backend.main:create_app --reload --factory --port 8080

be-prod:
  #!/usr/bin/env bash
  set -euo pipefail
  gunicorn pysilo_backend.main:create_app -w 4 -k uvicorn.workers.UvicornWorker --factory --port 8080

fe:
  #!/usr/bin/env bash
  set -euo pipefail
  source "$HOME/.rye/env"
  cd ./site
  pnpm run dev


services:
  #!/usr/bin/env bash
  set -euo pipefail
  podman-compose down
  podman-compose up --build postgres redis

gen-orval:
  #!/usr/bin/env bash
  set -euo pipefail
  source "$HOME/.rye/env"
  cd ./site
  pnpm run orval

spider:
  #!/usr/bin/env bash
  set -euo pipefail
  cd src/py_spider/py_spider/spiders
  scrapy crawl pypi_spider

db-gen message:
  #!/usr/bin/env bash
  set -euo pipefail
  cd ./pysilo-migrations/pysilo_migrations
  rye run alembic revision --autogenerate -m "{{message}}"

db-bump:
  #!/usr/bin/env bash
  set -euo pipefail
  cd ./pysilo-migrations/pysilo_migrations
  rye run alembic upgrade head

ruzty-ai-web:
  #!/usr/bin/env bash
  set -euo pipefail
  trunk serve --config ./ruzty-ai/Trunk.toml

ruzty-ai-native:
  #!/usr/bin/env bash
  set -euo pipefail
  cargo run -p ruzty-ai

pysilo-rs:
  #!/usr/bin/env bash
  set -euo pipefail
  cd "pysilo-rs/pysilo-app"
  dx serve --platform web --skip-assets --hot-reload=true --open

prompt-rs:
  #!/usr/bin/env bash
  set -euo pipefail
  cd "prompt-rs"
  dx serve

