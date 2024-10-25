from pyinfra.context import host
from pyinfra.operations import dnf, files, server
from pyinfra.facts.server import LinuxName, LinuxDistribution

FEDORA_VERSION = host.get_fact(LinuxDistribution).version


def stage1() -> None:
  update_system()
  install_rpmfusion()
  install_nvidia_drivers()
  print("Stage 1 complete. Please reboot your system and then run stage2.")


def stage2() -> None:
  install_core_tools()
  install_programming_languages()
  install_utilities()
  install_multimedia()
  install_communication_apps()
  install_vpn()
  install_gaming()
  install_bluetooth()
  install_sensors()
  install_vscode()
  print("Stage 2 complete. Please reboot your system and then run stage3.")


def stage3() -> None:
  configure_sensors()
  configure_bluetooth()
  update()
  print("Setup complete. You may want to reboot to ensure all changes take effect.")


def update_system() -> None:
  dnf.update()
  dnf.upgrade()
  dnf.packages(
    name="Install DNF core plugins",
    packages=["dnf-plugins-core"],
  )


def install_rpmfusion() -> None:
  dnf.rpm(
    name="Install RPM Fusion free",
    src=f"https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-{FEDORA_VERSION}.noarch.rpm",
  )
  dnf.rpm(
    name="Install RPM Fusion nonfree",
    src=f"https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-{FEDORA_VERSION}.noarch.rpm",
  )


def install_nvidia_drivers() -> None:
  dnf.packages(
    name="Install NVIDIA drivers and CUDA",
    packages=["akmod-nvidia", "xorg-x11-drv-nvidia-cuda", "xorg-x11-drv-nvidia-cuda-libs", "nvidia-vaapi-driver", "libva-utils", "vdpauinfo"],
  )
  files.download(
    name="Add NVIDIA CUDA repository",
    src=f"https://developer.download.nvidia.com/compute/cuda/repos/fedora{FEDORA_VERSION}/x86_64/cuda-fedora{FEDORA_VERSION}.repo",
    dest="/etc/yum.repos.d/cuda-fedora.repo",
  )
  dnf.repo(
    name="Disable NVIDIA driver module",
    repo="nvidia-driver",
    disabled=True,
  )
  dnf.packages(
    name="Install CUDA",
    packages=["cuda"],
  )


def install_core_tools() -> None:
  dnf.group(
    name="Install Development Tools",
    group="Development Tools",
  )
  dnf.packages(
    name="Install core development tools",
    packages=[
      "git",
      "gcc-c++",
      "clang",
      "llvm",
      "fzf",
      "bzip2-devel",
      "openssl-devel",
      "systemd-devel",
      "ncurses-devel",
      "libffi-devel",
      "readline-devel",
      "sqlite-devel",
      "tk-devel",
      "webkit2gtk4.0",
      "pkg-config",
      "snapd",
      "podman",
      "toolbox",
      "patchelf",
      "kernel-devel",
      "kernel-headers",
      "java-latest-openjdk-devel",
    ],
  )
  files.link(
    name="Link snap directory",
    path="/snap",
    target="/var/lib/snapd/snap",
  )


def install_programming_languages() -> None:
  server.shell(
    name="Install Rust",
    commands=["curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"],
  )
  server.shell(
    name="Install Rye",
    commands=["curl -sSf https://rye.astral.sh/get | bash"],
  )
  server.shell(
    name="Install NVM",
    commands=["curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash"],
  )
  server.shell(
    name="Install PNPM",
    commands=["curl -fsSL https://get.pnpm.io/install.sh | sh -"],
  )
  server.shell(
    name="Install pyenv",
    commands=["curl https://pyenv.run | bash"],
  )


def install_utilities() -> None:
  dnf.packages(
    name="Install utilities",
    packages=[
      "shellcheck",
      "just",
      "btop",
      "graphviz-devel",
      "zlib-devel",
      "SDL2-devel",
      "p7zip",
      "p7zip-plugins",
      "unrar",
      "flatpak",
      "lm_sensors",
      "psensor",
      "fwupd",
    ],
  )
  server.shell(
    name="Add Flathub repository",
    commands=["flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo"],
  )


def install_multimedia() -> None:
  dnf.packages(
    name="Install multimedia packages",
    packages=[
      "gstreamer1-plugins-bad-*",
      "gstreamer1-plugins-good-*",
      "gstreamer1-plugins-base",
      "gstreamer1-plugin-openh264",
      "gstreamer1-libav",
      "lame*",
      "vlc",
    ],
    exclude=["gstreamer1-plugins-bad-free-devel", "lame-devel"],
  )
  dnf.group(
    name="Upgrade Multimedia group",
    group="Multimedia",
    with_optional=True,
  )


def install_communication_apps() -> None:
  dnf.packages(
    name="Install Telegram",
    packages=["telegram-desktop"],
  )
  server.shell(
    name="Install Flatpak apps",
    commands=[
      "flatpak install -y flathub com.slack.Slack",
      "flatpak install -y flathub com.discordapp.Discord",
      "flatpak install -y flathub org.signal.Signal",
    ],
  )


def install_vpn() -> None:
  files.download(
    name="Add Mullvad VPN repository",
    src="https://repository.mullvad.net/rpm/stable/mullvad.repo",
    dest="/etc/yum.repos.d/mullvad.repo",
  )
  dnf.packages(
    name="Install Mullvad VPN",
    packages=["mullvad-vpn"],
  )


def install_gaming() -> None:
  dnf.packages(
    name="Install Steam",
    packages=["steam"],
  )


def install_bluetooth() -> None:
  dnf.packages(
    name="Install Bluetooth packages",
    packages=["bluez", "bluez-tools", "blueman"],
  )


def install_sensors() -> None:
  dnf.copr(
    name="Enable zenpower3 COPR repository",
    repo="shdwchn10/zenpower3",
  )
  dnf.packages(
    name="Install sensor packages",
    packages=["zenpower3", "zenmonitor3", "kernel-modules-extra"],
  )


def configure_sensors() -> None:
  server.shell(
    name="Load sensor modules",
    commands=[
      "modprobe zenpower",
      "modprobe amd_energy",
    ],
  )


def configure_bluetooth() -> None:
  # This function is empty in the original Justfile
  pass


def install_vscode() -> None:
  files.download(
    name="Add Microsoft GPG key",
    src="https://packages.microsoft.com/keys/microsoft.asc",
    dest="/etc/pki/rpm-gpg/microsoft.asc",
  )
  files.template(
    name="Add VSCode repository",
    src="vscode.repo.j2",
    dest="/etc/yum.repos.d/vscode.repo",
    mode="644",
    sudo=True,
  )
  dnf.packages(
    name="Install VSCode",
    packages=["code"],
  )


def update() -> None:
  update_system()
  server.shell(
    name="Update user-installed packages",
    commands=[
      "rustup update",
      "rye self update",
      "nvm upgrade",
      "pnpm update -g",
      "flatpak update -y",
      "snap refresh",
    ],
  )


if host.get_fact(LinuxName) == "Fedora":
  stage1()
  # Uncomment the following lines to run all stages at once
  # Remember to remove the print statements in each stage function if you do this
  # server.reboot(delay=30, name="Reboot after Stage 1")
  # stage2()
  # server.reboot(delay=30, name="Reboot after Stage 2")
  # stage3()
else:
  print("This script is designed for Fedora. Exiting.")
