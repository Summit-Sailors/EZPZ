mod system "../mods/system.just"

[unix]
install:
    #!/usr/bin/env bash
    set -euo pipefail
    @just system::ensure-command pip
    pip install pyre-check

[windows]
install:
    @echo "Pyre is not officially supported on Windows. Please use WSL or a virtual machine to run Pyre on Windows."

[unix]
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    pip uninstall -y pyre-check

[windows]
uninstall:
    @echo "If Pyre was installed in WSL, please run the uninstall command within the WSL environment."