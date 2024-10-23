mod system "../mods/system.just"

[windows]
install:
    #!/usr/bin/env bash
    set -euo pipefail
    if system::command-exists rustup; then
        echo "rustup is already installed. Version: $(rustup --version)"
        exit 0
    fi
    powershell -Command "Invoke-WebRequest https://win.rustup.rs -OutFile rustup-init.exe; .\rustup-init.exe -y"

[unix]
install:
    #!/usr/bin/env bash
    set -euo pipefail
    if system::command-exists rustup; then
        echo "rustup is already installed. Version: $(rustup --version)"
        exit 0
    fi
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"

[windows]
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! system::command-exists rustup; then
        echo "rustup is not installed."
        exit 0
    fi
    rustup self uninstall -y

[unix]
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! system::command-exists rustup; then
        echo "rustup is not installed."
        exit 0
    fi
    rustup self uninstall -y