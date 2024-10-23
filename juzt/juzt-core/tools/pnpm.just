mod system "../mods/system.just"

[windows]
install:
    system::install pnpm

[windows]
uninstall:
    system::uninstall pnpm

[unix]
install:
    #!/usr/bin/env bash
    set -euo pipefail
    curl -fsSL https://get.pnpm.io/install.sh | sh -

[unix]
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    pnpm uninstall -g pnpm