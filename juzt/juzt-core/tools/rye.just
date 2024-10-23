mod system "../mods/system.just"

install:
    curl -sSf https://rye-up.com/get | bash
    echo "Please add Rye to your PATH manually if needed."

[unix]
uninstall:
    rm -rf ~/.rye
    echo "Please remove any Rye-related lines from your shell configuration files manually."

[windows]
uninstall:
    @echo "Please remove the Rye installation directory manually and remove any Rye-related lines from your environment variables."