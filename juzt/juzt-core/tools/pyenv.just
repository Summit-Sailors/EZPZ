mod? system

[windows]
install:
    #!/usr/bin/env bash
    set -euo pipefail
    case {{PACKAGE_MANAGER}} in
        choco)  system::install pyenv-win ;;
        scoop)  system::install pyenv ;;
        winget) echo "pyenv is not available in winget. Please install manually." && exit 1 ;;
        *) echo "Unsupported package manager for pyenv on Windows" && exit 1 ;;
    esac

[windows]
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    case {{PACKAGE_MANAGER}} in
        choco)  system::uninstall pyenv-win ;;
        scoop)  system::uninstall pyenv ;;
        winget) echo "pyenv was not installed via winget. Please uninstall manually." && exit 1 ;;
        *) echo "Unsupported package manager for pyenv on Windows" && exit 1 ;;
    esac

[unix]
install:
    #!/usr/bin/env bash
    set -euo pipefail
    curl https://pyenv.run | bash
    echo "Please add pyenv to your PATH and shell configuration manually."

[unix]
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    rm -rf ~/.pyenv
    echo "Please remove pyenv-related lines from your shell configuration files manually."