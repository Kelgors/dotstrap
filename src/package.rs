use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, PickFirst};
use std::fmt::{self, Display};
use std::str::FromStr;
use std::{collections::HashMap, fs, path::Path};

fn dependency_default_source() -> String {
    return "os".to_string();
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct DependencyDefinition {
    #[serde(default = "dependency_default_source")]
    pub source: String,
    pub name: String,
}

impl Display for DependencyDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.source, self.name)
    }
}

impl FromStr for DependencyDefinition {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        let splitted: Vec<&str> = value.split(":").collect();
        return Ok(DependencyDefinition {
            source: if splitted.len() == 1 {
                "os"
            } else {
                splitted[0]
            }
            .to_string(),
            name: splitted[splitted.len() - 1].to_string(),
        });
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct LinkFileDefinition {
    pub src: String,
    pub dest: String,
    #[serde(default)]
    pub copy: bool,
}

impl Display for LinkFileDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.src, self.dest)
    }
}

impl FromStr for LinkFileDefinition {
    type Err = String;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        let splitted: Vec<&str> = value.split(":").collect();
        if splitted.len() < 2 {
            return Err("Wrong format, needing {src}:{dest}".to_string());
        }
        return Ok(LinkFileDefinition {
            src: splitted[0].to_string(),
            dest: splitted[1].to_string(),
            copy: false,
        });
    }
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct PackageDefinition {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    #[serde_as(as = "Vec<PickFirst<(_, DisplayFromStr)>>")]
    pub dependencies: Vec<DependencyDefinition>,
    pub post_install: Option<String>,
    #[serde(default)]
    #[serde_as(as = "Vec<PickFirst<(_, DisplayFromStr)>>")]
    pub links: Vec<LinkFileDefinition>,
}

impl PackageDefinition {
    pub fn load(pathname: &Path) -> Result<PackageDefinition> {
        let file_content = fs::read_to_string(pathname).expect(&format!(
            "Unable to find file {}",
            pathname.to_string_lossy()
        ));
        let parentdir = pathname
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let mut package_definition: PackageDefinition = serde_yaml::from_str(&file_content)?;
        package_definition.name = parentdir;
        return Ok(package_definition);
    }
}

// pub type PackageDefinition = AbsPackageDefinition<DependencyDefinition>;
pub type PackageCollection = HashMap<String, PackageDefinition>;

pub const DEFAULT_PACKAGE_EXAMPLE_FLATPAK: &str = "description: Flatpak is a system for building, distributing and running sandboxed desktop applications on Linux.
source: https://github.com/flatpak/flatpak
dependencies:
  - flatpak
post_install: |
  flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
";

pub const DEFAULT_PACKAGE_EXAMPLE_TMUX: &str = "description: \"tmux is a terminal multiplexer\"
source: https://github.com/tmux/tmux
dependencies:
  - tmux
links:
  - src: tmux.conf
    dest: ~/.tmux.conf
";
pub const DEFAULT_PACKAGE_EXAMPLE_TMUX_CONFIG: &str = "## split panes using | and -
bind | split-window -h
bind - split-window -v
unbind \'\"\'
unbind %
## switch panes using Alt-arrow without prefix
bind -n M-Left select-pane -L
bind -n M-Right select-pane -R
bind -n M-Up select-pane -U
bind -n M-Down select-pane -D
## switch window using Alt-Page(Up|Down) without prefix
bind -n M-PageUp previous-window
bind -n M-PageDown next-window
";
