{
  name: "devtool"
  author: "the og dev dude to PR"
  authors:["I PR-ed", "me too"]
  description:"just some dev tool"
  actions: ["install", "uninstall", "upgrade"]
  struct: {
      linux: {
          debian: {
              x86_64: {
                  install: {|| sudo apt update; sudo apt install -y devtool }
                  uninstall: {|| sudo apt remove -y devtool }
                  upgrade: {|| sudo apt update; sudo apt upgrade -y devtool }
              }
              aarch64: {
                  install: {|| sudo apt update; sudo apt install -y devtool }
                  uninstall: {|| sudo apt remove -y devtool }
                  upgrade: {|| sudo apt update; sudo apt upgrade -y devtool }
              }
          }
          ubuntu: {
              x86_64: {
                  install: {|| sudo apt update; sudo apt install -y devtool }
                  uninstall: {|| sudo apt remove -y devtool }
                  upgrade: {|| sudo apt update; sudo apt upgrade -y devtool }
              }
              aarch64: {
                  install: {|| sudo apt update; sudo apt install -y devtool }
                  uninstall: {|| sudo apt remove -y devtool }
                  upgrade: {|| sudo apt update; sudo apt upgrade -y devtool }
              }
          }
          fedora: {
              x86_64: {
                  install: {|| sudo dnf install -y devtool }
                  uninstall: {|| sudo dnf remove -y devtool }
                  upgrade: {|| sudo dnf upgrade -y devtool }
              }
              aarch64: {
                  install: {|| sudo dnf install -y devtool }
                  uninstall: {|| sudo dnf remove -y devtool }
                  upgrade: {|| sudo dnf upgrade -y devtool }
              }
          }
          centos: {
              x86_64: {
                  install: {|| sudo yum install -y devtool }
                  uninstall: {|| sudo yum remove -y devtool }
                  upgrade: {|| sudo yum upgrade -y devtool }
              }
          }
          rhel: {
              x86_64: {
                  install: {|| sudo yum install -y devtool }
                  uninstall: {|| sudo yum remove -y devtool }
                  upgrade: {|| sudo yum upgrade -y devtool }
              }
          }
          arch: {
              x86_64: {
                  install: {|| sudo pacman -S devtool }
                  uninstall: {|| sudo pacman -R devtool }
                  upgrade: {|| sudo pacman -Syu devtool }
              }
          }
          manjaro: {
              x86_64: {
                  install: {|| sudo pacman -S devtool }
                  uninstall: {|| sudo pacman -R devtool }
                  upgrade: {|| sudo pacman -Syu devtool }
              }
          }
          alpine: {
              x86_64: {
                  install: {|| sudo apk add devtool }
                  uninstall: {|| sudo apk del devtool }
                  upgrade: {|| sudo apk upgrade devtool }
              }
          }
          opensuse: {
              x86_64: {
                  install: {|| sudo zypper install -y devtool }
                  uninstall: {|| sudo zypper remove -y devtool }
                  upgrade: {|| sudo zypper update -y devtool }
              }
          }
          void: {
              x86_64: {
                  install: {|| sudo xbps-install -S devtool }
                  uninstall: {|| sudo xbps-remove -R devtool }
                  upgrade: {|| sudo xbps-install -u devtool }
              }
          }
          gentoo: {
              x86_64: {
                  install: {|| sudo emerge --ask devtool }
                  uninstall: {|| sudo emerge --unmerge devtool }
                  upgrade: {|| sudo emerge --update devtool }
              }
          }
          slackware: {
              x86_64: {
                  install: {|| sudo slackpkg install devtool }
                  uninstall: {|| sudo slackpkg remove devtool }
                  upgrade: {|| sudo slackpkg upgrade devtool }
              }
          }
      }
      macos: {
          homebrew: {
              x86_64: {
                  install: {|| brew install devtool }
                  uninstall: {|| brew uninstall devtool }
                  upgrade: {|| brew upgrade devtool }
              }
              aarch64: {
                  install: {|| brew install devtool }
                  uninstall: {|| brew uninstall devtool }
                  upgrade: {|| brew upgrade devtool }
              }
          }
          macports: {
              x86_64: {
                  install: {|| sudo port install devtool }
                  uninstall: {|| sudo port uninstall devtool }
                  upgrade: {|| sudo port upgrade devtool }
              }
          }
          _: {
              x86_64: {
                  install: {
                      deps: ["tar"]
                      command: {|| 
                          http get https://example.com/devtool/macos_x86_64.tar.gz | save devtool.tar.gz
                          tar xzf devtool.tar.gz -C /tmp
                          sudo /tmp/devtool_install.sh
                      }
                  }
                  uninstall: {|| sudo /opt/devtool/uninstall.sh }
                  upgrade: {
                      deps: ["tar"]
                      command: {|| 
                          http get https://example.com/devtool/macos_x86_64.tar.gz | save devtool.tar.gz
                          tar xzf devtool.tar.gz -C /tmp
                          sudo /tmp/devtool_upgrade.sh
                      }
                  }
              }
              aarch64: {
                  install: {
                      deps: ["tar"]
                      command: {|| 
                          http get https://example.com/devtool/macos_aarch64.tar.gz | save devtool.tar.gz
                          tar xzf devtool.tar.gz -C /tmp
                          sudo /tmp/devtool_install.sh
                      }
                  }
                  uninstall: {|| sudo /opt/devtool/uninstall.sh }
                  upgrade: {
                      deps: ["tar"]
                      command: {|| 
                          http get https://example.com/devtool/macos_aarch64.tar.gz | save devtool.tar.gz
                          tar xzf devtool.tar.gz -C /tmp
                          sudo /tmp/devtool_upgrade.sh
                      }
                  }
              }
          }
      }
      windows: {
          scoop: {
              x86_64: {
                  install: {|| scoop install devtool }
                  uninstall: {|| scoop uninstall devtool }
                  upgrade: {|| scoop update devtool }
              }
              aarch64: {
                  install: {|| scoop install devtool }
                  uninstall: {|| scoop uninstall devtool }
                  upgrade: {|| scoop update devtool }
              }
          }
          chocolatey: {
              x86_64: {
                  install: {|| choco install devtool -y }
                  uninstall: {|| choco uninstall devtool -y }
                  upgrade: {|| choco upgrade devtool -y }
              }
              aarch64: {
                  install: {|| choco install devtool -y }
                  uninstall: {|| choco uninstall devtool -y }
                  upgrade: {|| choco upgrade devtool -y }
              }
          }
          winget: {
              x86_64: {
                  install: {|| winget install -e --id ExampleCorp.DevTool }
                  uninstall: {|| winget uninstall -e --id ExampleCorp.DevTool }
                  upgrade: {|| winget upgrade -e --id ExampleCorp.DevTool }
              }
              aarch64: {
                  install: {|| winget install -e --id ExampleCorp.DevTool }
                  uninstall: {|| winget uninstall -e --id ExampleCorp.DevTool }
                  upgrade: {|| winget upgrade -e --id ExampleCorp.DevTool }
              }
          }
          _: {
              x86_64: {
                  install: {
                      deps: ["pwsh"]
                      command: {|| 
                          let temp_dir = (mktemp -d)
                          cd $temp_dir
                          http get https://example.com/devtool/windows_x86_64.zip | save devtool.zip
                          pwsh -Command "Expand-Archive -Path devtool.zip -DestinationPath ."
                          pwsh -File install.ps1
                          cd ..
                          rm -rf $temp_dir
                      }
                  }
                  uninstall: {|| pwsh -File 'C:\\Program Files\\DevTool\\uninstall.ps1' }
                  upgrade: {
                      deps: ["pwsh"]
                      command: {|| 
                          let temp_dir = (mktemp -d)
                          cd $temp_dir
                          http get https://example.com/devtool/windows_x86_64.zip | save devtool.zip
                          pwsh -Command "Expand-Archive -Path devtool.zip -DestinationPath ."
                          pwsh -File upgrade.ps1
                          cd ..
                          rm -rf $temp_dir
                      }
                  }
              }
              aarch64: {
                  install: {
                      deps: ["pwsh"]
                      command: {|| 
                          let temp_dir = (mktemp -d)
                          cd $temp_dir
                          http get https://example.com/devtool/windows_aarch64.zip | save devtool.zip
                          pwsh -Command "Expand-Archive -Path devtool.zip -DestinationPath ."
                          pwsh -File install.ps1
                          cd ..
                          rm -rf $temp_dir
                      }
                  }
                  uninstall: {|| pwsh -File 'C:\\Program Files\\DevTool\\uninstall.ps1' }
                  upgrade: {
                      deps: ["pwsh"]
                      command: {|| 
                          let temp_dir = (mktemp -d)
                          cd $temp_dir
                          http get https://example.com/devtool/windows_aarch64.zip | save devtool.zip
                          pwsh -Command "Expand-Archive -Path devtool.zip -DestinationPath ."
                          pwsh -File upgrade.ps1
                          cd ..
                          rm -rf $temp_dir
                      }
                  }
              }
          }
      }
      freebsd: {
          x86_64: {
              install: {|| sudo pkg install -y devtool }
              uninstall: {|| sudo pkg remove -y devtool }
              upgrade: {|| sudo pkg upgrade -y devtool }
          }
      }
      openbsd: {
          x86_64: {
              install: {|| doas pkg_add devtool }
              uninstall: {|| doas pkg_delete devtool }
              upgrade: {|| doas pkg_add -u devtool }
          }
      }
      netbsd: {
          x86_64: {
              install: {|| sudo pkgin install devtool }
              uninstall: {|| sudo pkgin remove devtool }
              upgrade: {|| sudo pkgin upgrade devtool }
          }
      }
      dragonflybsd: {
          x86_64: {
              install: {|| sudo pkg install -y devtool }
              uninstall: {|| sudo pkg remove -y devtool }
              upgrade: {|| sudo pkg upgrade -y devtool }
          }
      }
      solaris: {
          x86_64: {
              install: {|| sudo pkg install devtool }
              uninstall: {|| sudo pkg uninstall devtool }
              upgrade: {|| sudo pkg update devtool }
          }
      }
  }
}