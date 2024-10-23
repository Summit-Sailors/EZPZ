const LINUX_DISTROS = [
    "ubuntu", "debian", "fedora", "centos", "redhat", "arch", "manjaro",
    "opensuse", "mint", "elementary", "kali", "gentoo", "slackware"
]
const WINDOWS_PACKAGE_MANAGERS = ["winget", "choco", "scoop"]
const MACOS_PACKAGE_MANAGERS = ["brew", "port", "mas"]

export def get_distro_or_package_manager [os: string] {
    match $os {
        "Windows" => {
            $WINDOWS_PACKAGE_MANAGERS
            | where { (which $it | is-empty) == false }
            | first
            | default "_"
        },
        "Darwin" => {
            $MACOS_PACKAGE_MANAGERS
            | where { (which $it | is-empty) == false }
            | first
            | default { error make {msg: "No supported package manager found for macOS"} }
        },
        "Linux" => {
            $LINUX_DISTROS
            | where { $it.file | path exists }
            | get name
            | first
            | default { error make {msg: "Unsupported distribution"} }
        },
        _ => (error make {msg: $"Unsupported operating system: ($os)"})
    }
}