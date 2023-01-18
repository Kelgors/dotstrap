use anyhow::Result;
use std::{fs, path::Path};
use yaml_rust::YamlLoader;

#[derive(Default, Debug)]
pub enum PackageSource {
    #[default]
    System,
    Local,
}
impl From<&str> for PackageSource {
    fn from(value: &str) -> Self {
        if value == "dot" {
            return PackageSource::Local;
        }
        return PackageSource::System;
    }
}

#[derive(Default, Debug)]
pub struct DependencyDefinition {
    name: String,
    source: PackageSource,
}
impl From<&str> for DependencyDefinition {
    fn from(value: &str) -> Self {
        let splitted: Vec<&str> = value.split(":").collect();
        return DependencyDefinition {
            name: String::from(splitted[splitted.len() - 1]),
            source: PackageSource::from(splitted[0]),
        };
    }
}

#[derive(Debug)]
pub struct LinkFileDefinition {
    src: String,
    dest: String,
    copy: Option<bool>,
}

#[derive(Debug)]
pub struct PackageDefinition {
    name: String,
    dependencies: Vec<DependencyDefinition>,
    post_install: Vec<String>,
    links: Vec<LinkFileDefinition>,
}

impl PackageDefinition {
    pub fn from_path(pathname: &Path) -> Result<PackageDefinition> {
        println!("Load Package from {}", pathname.to_string_lossy());
        let file_content = fs::read_to_string(pathname)?;
        let docs = YamlLoader::load_from_str(&file_content)?;
        let doc = &docs[0];
        return Ok(PackageDefinition {
            name: String::from(
                doc["name"]
                    .as_str()
                    .unwrap_or(pathname.parent().unwrap().to_str().unwrap()),
            ),

            dependencies: doc["dependencies"]
                .as_vec()
                .unwrap_or(&Vec::new())
                .iter()
                .map(|row| DependencyDefinition::from(row.as_str().unwrap_or_default()))
                .collect::<Vec<DependencyDefinition>>(),

            post_install: doc["post_install"]
                .as_vec()
                .unwrap_or(&Vec::new())
                .iter()
                .map(|y| String::from(y.as_str().unwrap_or_default()))
                .collect::<Vec<String>>(),

            links: Vec::new(),
        });
    }
}
