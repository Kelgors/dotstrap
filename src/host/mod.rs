use pathbuf::pathbuf;
use std::path::PathBuf;

use self::config::HostConfiguration;
use crate::package::PackageDefinition;
use anyhow::Result;

pub mod config;

#[derive(Debug)]
pub struct HostDefinition {
    pub package: PackageDefinition,
    pub config: HostConfiguration,
}

impl HostDefinition {
    pub fn from_path(pathbuf: &PathBuf) -> Result<HostDefinition> {
        return Ok(HostDefinition {
            package: PackageDefinition::load(&pathbuf![pathbuf, "package.yml"])?,
            config: HostConfiguration::load(&pathbuf![pathbuf, "config.yml"])?,
        });
    }
}

pub const DEFAULT_HOST_PACKAGE_CONTENT: &str = "dependencies:
  # You can add host dependencies
  # by default, it is prefixed by \"os\" which means this dependency
  # will be added with the OS package manager described
  # in hosts/<hostname>/config.yml
  - firefox # same as os:firefox
  - dot:flatpak # install flatpak from packages directory
  - flatpak:net.lutris.Lutris # Install lutris from \"flatpak\" package manager described in config.yml
post_install: |
  # You can add custom shell commands
  sudo systemctl enable wallpapers.timer
  echo \"Hello\"
links:
  # You can label symlinks as a string source:destination
  # - init.vim:~/.config/nvim/init.vim
  # or via an object (you can specify also if you want to copy it or link it)
  # - src: init.vim
  #   dest: ~/.config/nvim/init.vim
  #   copy: false
";

pub const DEFAULT_HOST_CONFIG_CONTENT: &str = "package_managers:
  os:
    # indicate that the package manager supports multiple packages at once
    multiple: true
    commands:
      install: paru --needed -S <package>
      uninstall: paru -Runs <package>
  flatpak:
    multiple: false
    commands:
      install: sudo flatpak install <package>
      uninstall: sudo flatpak uninstall <package>
      clean: sudo flatpak uninstall --unused
";
