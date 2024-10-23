mod system "../mods/system.just"

[windows]
install:
    #!/usr/bin/env bash
    set -euo pipefail
    case {{PACKAGE_MANAGER}} in
        choco)  system::install nvm.portable ;;
        scoop)  system::install nvm ;;
        winget) system::install CoreyButler.NVMforWindows ;;
        *) echo "Unsupported package manager for NVM on Windows" && exit 1 ;;
    esac

[windows]
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    case {{PACKAGE_MANAGER}} in
        choco)  system::uninstall nvm.portable ;;
        scoop)  system::uninstall nvm ;;
        winget) system::uninstall CoreyButler.NVMforWindows ;;
        *) echo "Unsupported package manager for NVM on Windows" && exit 1 ;;
    esac

[unix]
install:
    #!/usr/bin/env bash
    set -euo pipefail
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash

[unix]
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    rm -rf "$HOME/.nvm"
    echo "Please remove NVM-related lines from your shell configuration files manually."