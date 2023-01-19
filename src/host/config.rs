use anyhow::{anyhow, Result};
use std::{collections::HashMap, fs, path::Path};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
pub struct PackageManagerCommands {
    pub install: String,
    pub uninstall: String,
    pub clean: Option<String>,
}
impl PackageManagerCommands {
    pub fn from_yaml(doc: &Yaml) -> Result<PackageManagerCommands> {
        return Ok(PackageManagerCommands {
            install: doc["install"].as_str().unwrap().to_string(),
            uninstall: doc["uninstall"].as_str().unwrap().to_string(),
            clean: if doc["clean"].as_str().is_some() {
                Some(doc["clean"].as_str().unwrap().to_string())
            } else {
                None
            },
        });
    }
}

#[derive(Debug)]
pub struct PackageManager {
    pub multiple: bool,
    pub commands: PackageManagerCommands,
}

impl PackageManager {
    pub fn from_yaml(doc: &Yaml) -> Result<PackageManager> {
        return Ok(PackageManager {
            multiple: doc["multiple"].as_bool().unwrap_or(false),
            commands: PackageManagerCommands::from_yaml(&doc["commands"])?,
        });
    }
}

#[derive(Debug)]
pub struct HostConfiguration {
    pub package_managers: HashMap<String, PackageManager>,
}

impl HostConfiguration {
    pub fn from_yaml(doc: &Yaml) -> Result<HostConfiguration> {
        let mut package_managers = HashMap::<String, PackageManager>::new();
        if doc["package_managers"].as_hash().is_none() {
            return Err(anyhow!["Missing package_managers definition in config.yml"]);
        }
        let pm_collection = doc["package_managers"].as_hash().unwrap();
        for (key_yml, pm_conf_yml) in pm_collection.into_iter() {
            let key = key_yml.as_str().unwrap().to_string();
            package_managers.insert(key, PackageManager::from_yaml(pm_conf_yml)?);
        }
        return Ok(HostConfiguration { package_managers });
    }

    pub fn from_path(pathname: &Path) -> Result<HostConfiguration> {
        println!("Load {}", pathname.to_string_lossy());
        let file_content = fs::read_to_string(pathname)?;
        let docs = YamlLoader::load_from_str(&file_content)?;
        return Self::from_yaml(&docs[0]);
    }
}
