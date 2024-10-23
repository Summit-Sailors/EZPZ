mod system "../mods/system.just"

# Stage 1: Initial setup and critical updates
stage1:
    just update-system
    just install-rpmfusion
    just install-nvidia-drivers
    @echo "Please reboot your system now and then run 'just stage2'"

# Stage 2: Install core tools and development environment
stage2:
    just install-core-tools
    just install-programming-languages
    just install-utilities
    just install-multimedia
    just install-communication-apps
    just install-vpn
    just install-gaming
    just install-bluetooth
    just install-sensors
    just install-vscode
    @echo "Stage 2 complete. Please reboot your system and then run 'just stage3' for final configurations."

# Stage 3: Final configurations and updates
stage3:
    just configure-sensors
    just configure-bluetooth
    just update
    @echo "Setup complete. You may want to reboot to ensure all changes take effect."

# Update and upgrade the system
update-system:
    #!/usr/bin/env bash
    set -euo pipefail
    just system::update
    just system::upgrade
    sudo dnf install -y dnf-plugins-core

# Install RPM Fusion repositories
install-rpmfusion:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo dnf install -y https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm
    sudo dnf install -y https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm

# Install NVIDIA drivers and CUDA
install-nvidia-drivers:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo dnf install -y akmod-nvidia xorg-x11-drv-nvidia-cuda xorg-x11-drv-nvidia-cuda-libs nvidia-vaapi-driver libva-utils vdpauinfo
    sudo dnf config-manager --add-repo https://developer.download.nvidia.com/compute/cuda/repos/fedora39/x86_64/cuda-fedora39.repo
    sudo dnf clean all
    sudo dnf module disable -y nvidia-driver
    sudo dnf install -y cuda

# Install core tools and development packages
install-core-tools:
    #!/usr/bin/env bash
    set -euo pipefail
    just system::group-install "Development Tools"
    sudo dnf install -y git gcc-c++ clang llvm fzf bzip2-devel openssl-devel systemd-devel ncurses-devel libffi-devel readline-devel sqlite-devel tk-devel webkit2gtk4.0 pkg-config snapd podman toolbox patchelf kernel-devel kernel-headers java-latest-openjdk-devel
    sudo ln -s /var/lib/snapd/snap /snap

# Install programming languages and package managers
install-programming-languages:
    #!/usr/bin/env bash
    set -euo pipefail
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    curl -sSf https://rye.astral.sh/get | bash
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
    curl -fsSL https://get.pnpm.io/install.sh | sh -
    curl https://pyenv.run | bash

# Install various utilities
install-utilities:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo dnf install -y shellcheck just btop graphviz-devel zlib-devel SDL2-devel p7zip p7zip-plugins unrar flatpak lm_sensors psensor fwupd
    flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# Install multimedia software
install-multimedia:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo dnf install -y gstreamer1-plugins-{bad-*,good-*,base} gstreamer1-plugin-openh264 gstreamer1-libav --exclude=gstreamer1-plugins-bad-free-devel
    sudo dnf install -y lame* --exclude=lame-devel
    just system::group-upgrade Multimedia --with-optional
    sudo dnf install -y vlc

# Install communication apps
install-communication-apps:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo dnf install -y telegram-desktop
    flatpak install -y flathub com.slack.Slack
    flatpak install -y flathub com.discordapp.Discord
    flatpak install -y flathub org.signal.Signal

# Install VPN
install-vpn:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo dnf config-manager --add-repo https://repository.mullvad.net/rpm/stable/mullvad.repo
    sudo dnf install -y mullvad-vpn

# Install gaming software
install-gaming:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo dnf install -y steam

# Install and configure Bluetooth
install-bluetooth:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo dnf install -y bluez bluez-tools blueman

# Install sensors
install-sensors:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo dnf copr enable -y shdwchn10/zenpower3
    sudo dnf install -y zenpower3 zenmonitor3 kernel-modules-extra

# Configure sensors
configure-sensors:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo modprobe zenpower
    sudo modprobe amd_energy

# Install VSCode
install-vscode:
    #!/usr/bin/env bash
    set -euo pipefail
    sudo rpm --import https://packages.microsoft.com/keys/microsoft.asc
    echo -e "[code]\nname=Visual Studio Code\nbaseurl=https://packages.microsoft.com/yumrepos/vscode\nenabled=1\ngpgcheck=1\ngpgkey=https://packages.microsoft.com/keys/microsoft.asc" | sudo tee /etc/yum.repos.d/vscode.repo > /dev/null
    sudo dnf install -y code

# Update all installed packages
update:
    #!/usr/bin/env bash
    set -euo pipefail
    just update-system
    rustup update
    rye self update
    nvm upgrade
    pnpm update -g
    flatpak update -y
    sudo snap refresh

