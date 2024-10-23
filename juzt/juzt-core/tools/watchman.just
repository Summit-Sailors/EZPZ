mod system '../mods/system.just'

TOOL := "watchman"

install:
    system::install {{TOOL}}

uninstall:
    system::uninstall {{TOOL}}