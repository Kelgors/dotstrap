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
            package: PackageDefinition::from_path(&pathbuf![pathbuf, "package.yml"])?,
            config: HostConfiguration::from_path(&pathbuf![pathbuf, "config.yml"])?,
        });
    }
}
