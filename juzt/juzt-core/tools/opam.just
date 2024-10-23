mod system "../mods/system.just"

[windows]
install:
    powershell -Command "Invoke-Expression \"& { $(Invoke-RestMethod https://raw.githubusercontent.com/ocaml/opam/master/shell/install.ps1) }\""

[unix]
install:
    sh <(curl -fsSL https://raw.githubusercontent.com/ocaml/opam/master/shell/install.sh)

[linux]
install:
    #!/usr/bin/env bash
    set -euo pipefail
    case {{PACKAGE_MANAGER}} in
        pacman) system::install opam ;;
        apt)    system::install opam ;;
        dnf)    system::install opam ;;
        urpmi)  system::install opam ;;
        pkg)    system::install ocaml-opam ;;  # For FreeBSD
        brew)   system::install opam ;;
        port)   system::install opam ;;
        guix)   system::install opam ;;
        *) sh <(curl -fsSL https://raw.githubusercontent.com/ocaml/opam/master/shell/install.sh) ;;
    esac

[unix]
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    rm -rf ~/.opam
    system::uninstall opam

[windows]
uninstall:
    #!/usr/bin/env bash
    set -euo pipefail
    powershell -Command "Remove-Item -Path $env:LOCALAPPDATA\Programs\opam -Recurse -Force"
    powershell -Command "Remove-Item -Path $env:ProgramFiles\opam -Recurse -Force"