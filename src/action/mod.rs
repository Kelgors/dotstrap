use crate::host::config::HostConfiguration;
use crate::package::PackageCollection;
use crate::package::PackageDefinition;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PackageOperation {
    Install,
    Uninstall,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScriptOperation {
    Run,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FileOperation {
    Link,
    Copy,
    Remove,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum SystemAction {
    Package {
        operation: PackageOperation,
        source: String,
        name: String,
    },
    Script {
        operation: ScriptOperation,
        script: String,
    },
    File {
        operation: FileOperation,
        src: String,
        dest: String,
    },
}

fn flush_pending_actions(
    merged_actions: &mut Vec<SystemAction>,
    last_source: &String,
    pending_packages: &mut Vec<String>,
    last_operation: &PackageOperation,
) {
    merged_actions.push(SystemAction::Package {
        operation: last_operation.clone(),
        source: last_source.to_string(),
        name: pending_packages.join(" "),
    });
    pending_packages.clear();
}

/* TODO Look for a better implementation */
pub fn compact_mergeable_actions(
    system_actions: &Vec<SystemAction>,
    config: &HostConfiguration,
) -> Vec<SystemAction> {
    let mut merged_actions = vec![];

    let mut last_source: String = "".to_string();
    let mut last_operation: PackageOperation = PackageOperation::Install;
    let mut pending_packages = vec![];
    for system_action in system_actions.into_iter() {
        match system_action {
            SystemAction::Package {
                operation,
                source,
                name,
            } => {
                let pm_supports_multiple = config
                    .package_managers
                    .get(source)
                    .unwrap()
                    .multiple
                    .clone();
                if last_source.len() > 0 && (last_source.ne(source) || last_operation.ne(operation))
                {
                    flush_pending_actions(
                        &mut merged_actions,
                        &last_source,
                        &mut pending_packages,
                        &last_operation,
                    );
                    last_source = String::new();
                }
                if pm_supports_multiple {
                    if last_source.len() == 0 {
                        last_source = source.clone();
                        last_operation = operation.clone();
                    }
                    pending_packages.push(name.clone());
                } else {
                    merged_actions.push(system_action.clone());
                }
            }
            _ => {
                if pending_packages.len() > 0 {
                    flush_pending_actions(
                        &mut merged_actions,
                        &last_source,
                        &mut pending_packages,
                        &last_operation,
                    );
                    last_source = String::new();
                }
                merged_actions.push(system_action.clone());
            }
        }
    }
    if pending_packages.len() > 0 {
        flush_pending_actions(
            &mut merged_actions,
            &last_source,
            &mut pending_packages,
            &last_operation,
        );
    }
    return merged_actions;
}

fn transform_package_deps_to_actions(
    package: &PackageDefinition,
    repo: &PackageCollection,
    loaded: &mut Vec<String>,
) -> Result<Vec<SystemAction>> {
    let mut dependencies_actions = vec![];
    let mut system_actions = vec![];

    if package.dependencies.len() == 0 {
        return Ok(dependencies_actions);
    }

    let package_deps = &package.dependencies;
    for dependency in package_deps.into_iter() {
        let dep_name = &dependency.name;
        let dep_src = &dependency.source;
        if dep_src == "dot" {
            if loaded.contains(dep_name) {
                continue;
            }
            loaded.push(dep_name.clone());
            // Load package
            dependencies_actions.append(&mut transform_package_to_actions(
                repo.get(dep_name).unwrap(),
                repo,
                loaded,
            )?);
            continue;
        }
        // Load system packages
        system_actions.push(SystemAction::Package {
            operation: PackageOperation::Install,
            source: dep_src.to_string(),
            name: dep_name.to_string(),
        });
    }
    dependencies_actions.append(&mut system_actions);
    return Ok(dependencies_actions);
}

pub fn transform_package_to_actions(
    package: &PackageDefinition,
    repo: &PackageCollection,
    loaded: &mut Vec<String>,
) -> Result<Vec<SystemAction>> {
    let mut package_actions: Vec<SystemAction> =
        transform_package_deps_to_actions(package, repo, loaded)?;
    if package.post_install.is_some() {
        package_actions.push(SystemAction::Script {
            operation: ScriptOperation::Run,
            script: package.post_install.clone().unwrap(),
        });
    }
    if package.links.len() > 0 {
        package_actions.append(
            &mut (&package.links)
                .into_iter()
                .map(|link| SystemAction::File {
                    operation: if link.copy {
                        FileOperation::Copy
                    } else {
                        FileOperation::Link
                    },
                    src: link.src.clone(),
                    dest: link.dest.clone(),
                })
                .collect::<Vec<SystemAction>>(),
        );
    }
    return Ok(package_actions);
}
