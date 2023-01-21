use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;

use crate::action::SystemAction;
use crate::package::DependencyDefinition;
use crate::package::LinkFileDefinition;
use crate::package::PackageCollection;
use crate::package::PackageDefinition;

#[derive(Debug, Serialize, Deserialize)]
struct DependencyState {
    source: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageState {
    source: String,
    name: String,
    dependencies: Vec<DependencyState>,
    files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lockfile {
    dependencies: Vec<PackageState>,
    files: Vec<String>,
}

impl Lockfile {
    pub fn save(self) -> Result<()> {
        let output = serde_yaml::to_string(&self)?;
        std::fs::write(".lockfile", output)?;
        return Ok(());
    }
    pub fn load() -> Result<Lockfile> {
        let file_content = std::fs::read_to_string(".lockfile").unwrap_or_default();
        let lockfile = serde_yaml::from_str(&file_content)?;
        return Ok(lockfile);
    }
    // pub fn from_yaml(doc: &Yaml) -> Result<Lockfile> {}
    // pub fn from_path(pathname: &Path) -> Result<Lockfile> {}
}

fn lock_links(links: &Vec<LinkFileDefinition>) -> Vec<String> {
    return links.into_iter().map(|item| item.dest.clone()).collect();
}

fn lock_package_dependencies(
    package_dependencies: &Vec<DependencyDefinition>,
) -> Vec<DependencyState> {
    return package_dependencies
        .into_iter()
        .map(|dep| {
            let dep_name = dep.name.as_ref().unwrap();
            return DependencyState {
                source: dep.source.clone(),
                name: dep_name.clone(),
            };
        })
        .collect();
}

fn lock_host_dependencies(
    host_dependencies: &Vec<DependencyDefinition>,
    repo: &PackageCollection,
) -> Vec<PackageState> {
    return host_dependencies
        .into_iter()
        .filter(|dep| dep.name.is_some())
        .map(|dep| {
            let dep_name = dep.name.as_ref().unwrap();
            if dep.source != "dot" {
                return PackageState {
                    source: dep.source.clone(),
                    name: dep_name.clone(),
                    dependencies: vec![],
                    files: vec![],
                };
            }
            let package = repo.get(dep_name).unwrap();
            return PackageState {
                source: dep.source.clone(),
                name: dep_name.clone(),
                dependencies: if let Some(dependencies) = package.dependencies.as_ref() {
                    lock_package_dependencies(dependencies)
                } else {
                    vec![]
                },
                files: if let Some(links) = package.links.as_ref() {
                    lock_links(links)
                } else {
                    vec![]
                },
            };
        })
        .collect();
}

pub fn create_lockfile(package: &PackageDefinition, repo: &PackageCollection) -> Lockfile {
    let dependencies = package.dependencies.as_ref();
    return Lockfile {
        dependencies: if let Some(dependencies) = dependencies {
            lock_host_dependencies(dependencies, repo)
        } else {
            vec![]
        },
        files: if let Some(links) = package.links.as_ref() {
            lock_links(links)
        } else {
            vec![]
        },
    };
}

fn transform_lockfile_to_actions(lockfile: &Lockfile) -> (Vec<(String, String)>, Vec<String>) {
    let mut packages: Vec<(String, String)> = vec![];
    let mut files: Vec<String> = vec![];
    for package in (&lockfile.dependencies).into_iter() {
        for file in (&package.files).into_iter() {
            files.push(file.clone());
        }
    }
    return (packages, files);
}

pub fn make_lockfile_diff(next_lockfile: &Lockfile, prev_lockfile: &Lockfile) -> Vec<SystemAction> {
    let output = vec![];
    let (next_packages, next_files) = transform_lockfile_to_actions(&next_lockfile);
    let (prev_packages, prev_files) = transform_lockfile_to_actions(&prev_lockfile);
    return output;
}
