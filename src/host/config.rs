use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PackageManagerCommands {
    pub install: String,
    pub uninstall: String,
    pub clean: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct PackageManager {
    pub multiple: bool,
    pub commands: PackageManagerCommands,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct HostConfiguration {
    pub package_managers: HashMap<String, PackageManager>,
}

impl HostConfiguration {
    pub fn load(pathname: &Path) -> Result<HostConfiguration> {
        let file_content = fs::read_to_string(pathname).expect(&format!(
            "Unable to find file {}",
            pathname.to_string_lossy()
        ));
        let host_configuration: HostConfiguration = serde_yaml::from_str(&file_content)?;
        return Ok(host_configuration);
    }
}
