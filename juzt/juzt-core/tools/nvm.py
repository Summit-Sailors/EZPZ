from enum import Enum, auto
from typing import Protocol, runtime_checkable
from dataclasses import dataclass

from pyinfra import host
from pyinfra.operations import apk, apt, dnf, pkg, yum, brew, files, pacman, server, zypper, chocolatey
from pyinfra.facts.server import WindowsVersion, LinuxDistribution
from pyinfra.facts.hardware import Arch


class PackageManager(Enum):
  APT = auto()
  BREW = auto()
  CHOCOLATEY = auto()
  PACMAN = auto()
  PIP = auto()
  SCOOP = auto()
  WINGET = auto()
  YUM = auto()
  DNF = auto()
  ZYPPER = auto()
  PKG = auto()
  APK = auto()
  NPM = auto()
  GEM = auto()


@dataclass
class SystemInfo:
  os: LinuxDistribution | WindowsVersion
  package_manager: PackageManager
  arch: Arch


@runtime_checkable
class SoftwareManagement(Protocol):
  def install(self, system_info: SystemInfo) -> None: ...

  def uninstall(self, system_info: SystemInfo) -> None: ...

  def upgrade(self, system_info: SystemInfo) -> None: ...

  def is_installed(self, system_info: SystemInfo) -> bool: ...


class NVMManager:
  def _get_package_name(self, system_info: SystemInfo) -> str:
    if isinstance(system_info.os, WindowsVersion):
      return "nvm" if system_info.package_manager == PackageManager.SCOOP else "nvm-windows"
    return "nvm"

  def _run_nvm_install_script(self) -> None:
    server.shell(
      name="Install NVM using official script",
      commands=[
        "curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash",
      ],
    )

  def install(self, system_info: SystemInfo) -> None:
    package_name = self._get_package_name(system_info)

    if isinstance(system_info.os, WindowsVersion):
      if system_info.package_manager == PackageManager.CHOCOLATEY:
        chocolatey.packages(name=f"Install {package_name}", packages=[package_name])
      elif system_info.package_manager == PackageManager.SCOOP:
        server.shell(name=f"Install {package_name}", commands=[f"scoop install {package_name}"])
      elif system_info.package_manager == PackageManager.WINGET:
        server.winget(name=f"Install {package_name}", packages=[package_name])
    elif isinstance(system_info.os, LinuxDistribution):
      if system_info.package_manager == PackageManager.APT:
        apt.packages(name=f"Install {package_name}", packages=[package_name])
      elif system_info.package_manager == PackageManager.BREW:
        brew.packages(name=f"Install {package_name}", packages=[package_name])
      elif system_info.package_manager == PackageManager.YUM:
        yum.packages(name=f"Install {package_name}", packages=[package_name])
      elif system_info.package_manager == PackageManager.DNF:
        dnf.packages(name=f"Install {package_name}", packages=[package_name])
      elif system_info.package_manager == PackageManager.ZYPPER:
        zypper.packages(name=f"Install {package_name}", packages=[package_name])
      elif system_info.package_manager == PackageManager.PACMAN:
        pacman.packages(name=f"Install {package_name}", packages=[package_name])
      elif system_info.package_manager == PackageManager.APK:
        apk.packages(name=f"Install {package_name}", packages=[package_name])
      elif system_info.package_manager == PackageManager.PKG:
        pkg.packages(name=f"Install {package_name}", packages=[package_name])
      else:
        self._run_nvm_install_script()
    else:
      self._run_nvm_install_script()

  def uninstall(self, system_info: SystemInfo) -> None:
    package_name = self._get_package_name(system_info)

    if isinstance(system_info.os, WindowsVersion):
      if system_info.package_manager == PackageManager.CHOCOLATEY:
        chocolatey.packages(name=f"Uninstall {package_name}", packages=[package_name], present=False)
      elif system_info.package_manager == PackageManager.SCOOP:
        server.shell(name=f"Uninstall {package_name}", commands=[f"scoop uninstall {package_name}"])
      elif system_info.package_manager == PackageManager.WINGET:
        server.winget(name=f"Uninstall {package_name}", packages=[package_name], present=False)
    elif isinstance(system_info.os, LinuxDistribution):
      if system_info.package_manager == PackageManager.APT:
        apt.packages(name=f"Uninstall {package_name}", packages=[package_name], present=False)
      elif system_info.package_manager == PackageManager.BREW:
        brew.packages(name=f"Uninstall {package_name}", packages=[package_name], present=False)
      elif system_info.package_manager == PackageManager.YUM:
        yum.packages(name=f"Uninstall {package_name}", packages=[package_name], present=False)
      elif system_info.package_manager == PackageManager.DNF:
        dnf.packages(name=f"Uninstall {package_name}", packages=[package_name], present=False)
      elif system_info.package_manager == PackageManager.ZYPPER:
        zypper.packages(name=f"Uninstall {package_name}", packages=[package_name], present=False)
      elif system_info.package_manager == PackageManager.PACMAN:
        pacman.packages(name=f"Uninstall {package_name}", packages=[package_name], present=False)
      elif system_info.package_manager == PackageManager.APK:
        apk.packages(name=f"Uninstall {package_name}", packages=[package_name], present=False)
      elif system_info.package_manager == PackageManager.PKG:
        pkg.packages(name=f"Uninstall {package_name}", packages=[package_name], present=False)
      else:
        server.shell(
          name="Uninstall NVM",
          commands=[
            "rm -rf $HOME/.nvm",
            'echo "Please remove NVM-related lines from your shell configuration files manually."',
          ],
        )
    else:
      server.shell(
        name="Uninstall NVM",
        commands=[
          "rm -rf $HOME/.nvm",
          'echo "Please remove NVM-related lines from your shell configuration files manually."',
        ],
      )

  def upgrade(self, system_info: SystemInfo) -> None:
    package_name = self._get_package_name(system_info)

    if isinstance(system_info.os, WindowsVersion):
      if system_info.package_manager == PackageManager.CHOCOLATEY:
        chocolatey.packages(name=f"Upgrade {package_name}", packages=[package_name], latest=True)
      elif system_info.package_manager == PackageManager.SCOOP:
        server.shell(name=f"Upgrade {package_name}", commands=[f"scoop update {package_name}"])
      elif system_info.package_manager == PackageManager.WINGET:
        server.winget(name=f"Upgrade {package_name}", packages=[package_name], latest=True)
    elif isinstance(system_info.os, LinuxDistribution):
      if system_info.package_manager == PackageManager.APT:
        apt.packages(name=f"Upgrade {package_name}", packages=[package_name], latest=True)
      elif system_info.package_manager == PackageManager.BREW:
        brew.packages(name=f"Upgrade {package_name}", packages=[package_name], latest=True)
      elif system_info.package_manager == PackageManager.YUM:
        yum.packages(name=f"Upgrade {package_name}", packages=[package_name], latest=True)
      elif system_info.package_manager == PackageManager.DNF:
        dnf.packages(name=f"Upgrade {package_name}", packages=[package_name], latest=True)
      elif system_info.package_manager == PackageManager.ZYPPER:
        zypper.packages(name=f"Upgrade {package_name}", packages=[package_name], latest=True)
      elif system_info.package_manager == PackageManager.PACMAN:
        pacman.packages(name=f"Upgrade {package_name}", packages=[package_name], latest=True)
      elif system_info.package_manager == PackageManager.APK:
        apk.packages(name=f"Upgrade {package_name}", packages=[package_name], latest=True)
      elif system_info.package_manager == PackageManager.PKG:
        pkg.packages(name=f"Upgrade {package_name}", packages=[package_name], latest=True)
      else:
        server.shell(
          name="Upgrade NVM",
          commands=[
            'cd $HOME/.nvm && git fetch --tags origin && git checkout `git describe --abbrev=0 --tags --match "v[0-9]*" $(git rev-list --tags --max-count=1)`',
          ],
        )
    else:
      server.shell(
        name="Upgrade NVM",
        commands=[
          'cd $HOME/.nvm && git fetch --tags origin && git checkout `git describe --abbrev=0 --tags --match "v[0-9]*" $(git rev-list --tags --max-count=1)`',
        ],
      )

  def is_installed(self, system_info: SystemInfo) -> bool:
    package_name = self._get_package_name(system_info)

    if isinstance(system_info.os, WindowsVersion):
      if system_info.package_manager == PackageManager.CHOCOLATEY:
        return host.get_fact(chocolatey.chocolatey_packages)[package_name] is not None
      if system_info.package_manager == PackageManager.SCOOP:
        return package_name in host.get_fact(server.command, "scoop list")
      if system_info.package_manager == PackageManager.WINGET:
        return package_name in host.get_fact(server.command, "winget list")
    elif isinstance(system_info.os, LinuxDistribution):
      if system_info.package_manager in [
        PackageManager.APT,
        PackageManager.YUM,
        PackageManager.DNF,
        PackageManager.ZYPPER,
        PackageManager.PACMAN,
        PackageManager.APK,
        PackageManager.PKG,
      ]:
        return package_name in host.get_fact(server.packages)
      if system_info.package_manager == PackageManager.BREW:
        return package_name in host.get_fact(brew.brew_packages)

    # For systems where we use the NVM install script
    return host.get_fact(files.directory, "$HOME/.nvm")


# Usage example:
nvm_manager = NVMManager()
system_info = SystemInfo(os=LinuxDistribution.UBUNTU, package_manager=PackageManager.APT, arch=Arch.X86_64)

nvm_manager.install(system_info)
nvm_manager.upgrade(system_info)
nvm_manager.uninstall(system_info)
