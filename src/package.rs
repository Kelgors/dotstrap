use anyhow::Result;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, PartialEq, Eq)]
pub struct DependencyDefinition {
    pub source: String,
    pub name: Option<String>,
    pub tagged: Option<Vec<String>>,
}
impl From<&str> for DependencyDefinition {
    fn from(value: &str) -> Self {
        let splitted: Vec<&str> = value.split(":").collect();
        if splitted[0] == "tagged" {
            return DependencyDefinition {
                source: "dot".to_string(),
                name: None,
                tagged: Some(
                    splitted[1]
                        .split(",")
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>(),
                ),
            };
        }
        return DependencyDefinition {
            source: if splitted.len() == 1 {
                "os"
            } else {
                splitted[0]
            }
            .to_string(),
            name: Some(splitted[splitted.len() - 1].to_string()),
            tagged: None,
        };
    }
}
impl From<&Yaml> for DependencyDefinition {
    fn from(value: &Yaml) -> Self {
        let text = value.as_str();
        if text.is_some() {
            return DependencyDefinition::from(text.unwrap());
        }
        let mut source = value["source"].as_str().unwrap_or("os");
        // Parse tagged
        let tagged: Option<Vec<String>> = if value["tagged"].as_vec().is_some() {
            Some(
                value["tagged"]
                    .as_vec()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|y| y.as_str().unwrap_or_default().to_string())
                    .collect::<Vec<String>>(),
            )
        } else if value["tagged"].as_str().is_some() {
            Some(vec![value["tagged"].as_str().unwrap().to_string()])
        } else {
            None
        };
        if tagged.is_some() {
            source = "dot";
        }
        return DependencyDefinition {
            source: source.to_string(),
            name: if tagged.is_none() {
                Some(value["name"].as_str().unwrap_or_default().to_string())
            } else {
                None
            },
            tagged: tagged,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LinkFileDefinition {
    pub src: String,
    pub dest: String,
    pub copy: bool,
}

impl LinkFileDefinition {
    pub fn prepend_src(self, dirpath: &PathBuf) -> LinkFileDefinition {
        return LinkFileDefinition {
            src: dirpath.join(self.src).to_str().unwrap().to_string(),
            dest: self.dest,
            copy: self.copy.clone(),
        };
    }
}

impl From<&str> for LinkFileDefinition {
    fn from(value: &str) -> Self {
        let splitted: Vec<&str> = value.split(":").collect();
        return LinkFileDefinition {
            src: String::from(splitted[0]),
            dest: String::from(splitted[1]),
            copy: false,
        };
    }
}

impl From<&Yaml> for LinkFileDefinition {
    fn from(value: &Yaml) -> Self {
        let text = value.as_str();
        if text.is_some() {
            return LinkFileDefinition::from(text.unwrap());
        }
        return LinkFileDefinition {
            src: String::from(value["src"].as_str().unwrap()),
            dest: String::from(value["dest"].as_str().unwrap()),
            copy: value["copy"].as_bool().unwrap_or(false),
        };
    }
}

pub type PackageCollection = HashMap<String, PackageDefinition>;

#[derive(Debug, PartialEq, Eq)]
pub struct PackageDefinition {
    pub name: String,
    pub description: Option<String>,
    pub dependencies: Option<Vec<DependencyDefinition>>,
    pub post_install: Option<Vec<String>>,
    pub links: Option<Vec<LinkFileDefinition>>,
}

impl PackageDefinition {
    pub fn from_yaml(doc: &Yaml, name: String, dirpath: PathBuf) -> Result<PackageDefinition> {
        return Ok(PackageDefinition {
            name: name,
            description: if doc["description"].as_str().is_some() {
                Some(doc["description"].as_str().unwrap().to_string())
            } else {
                None
            },

            dependencies: if doc["dependencies"].is_array() {
                Some(
                    doc["dependencies"]
                        .as_vec()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|row| DependencyDefinition::from(row))
                        .collect::<Vec<DependencyDefinition>>(),
                )
            } else {
                None
            },

            post_install: if doc["post_install"].is_array() {
                Some(
                    doc["post_install"]
                        .as_vec()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|y| String::from(y.as_str().unwrap_or_default()))
                        .collect::<Vec<String>>(),
                )
            } else {
                None
            },

            links: if doc["links"].is_array() {
                Some(
                    doc["links"]
                        .as_vec()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|y| LinkFileDefinition::from(y).prepend_src(&dirpath))
                        .collect::<Vec<LinkFileDefinition>>(),
                )
            } else {
                None
            },
        });
    }

    pub fn from_path(pathname: &Path) -> Result<PackageDefinition> {
        println!("Load Package from {}", pathname.to_string_lossy());
        let file_content = fs::read_to_string(pathname)?;
        let docs = YamlLoader::load_from_str(&file_content)?;
        let parentdir = pathname
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        return Self::from_yaml(
            &docs[0],
            parentdir,
            pathname.parent().unwrap().to_path_buf(),
        );
    }
}
