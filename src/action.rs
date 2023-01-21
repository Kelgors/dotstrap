use crate::host::config::HostConfiguration;
use crate::package::PackageCollection;
use crate::package::PackageDefinition;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum SystemAction {
    InstallPackage(String, String),
    RunScript(String),
    CreateLink {
        src: String,
        dest: String,
        copy: bool,
    },
    UninstallPackage(String, String),
    DeleteLink(String),
}

fn flush_pending_actions(
    merged_actions: &mut Vec<SystemAction>,
    last_source: &String,
    pending_packages: &mut Vec<String>,
) {
    merged_actions.push(SystemAction::InstallPackage(
        last_source.to_string(),
        pending_packages.join(" "),
    ));
    pending_packages.clear();
}

pub fn compact_mergeable_actions(
    actions: &Vec<SystemAction>,
    config: &HostConfiguration,
) -> Vec<SystemAction> {
    let mut merged_actions = vec![];
    let mut last_source: String = "".to_string();
    let mut pending_packages = vec![];
    for action in actions.into_iter() {
        match action {
            SystemAction::InstallPackage(source, package) => {
                if !config.package_managers.get(source).unwrap().multiple {
                    merged_actions.push(action.clone());
                    continue;
                }
                if last_source.eq("") {
                    last_source = source.clone();
                } else if last_source.ne(source) {
                    flush_pending_actions(&mut merged_actions, &last_source, &mut pending_packages);
                    last_source = source.clone();
                }
                pending_packages.push(package.clone());
            }
            _ => {
                if pending_packages.len() > 0 {
                    flush_pending_actions(&mut merged_actions, &last_source, &mut pending_packages);
                    last_source = "".to_string();
                }
                merged_actions.push(action.clone())
            }
        }
    }
    if pending_packages.len() > 0 {
        flush_pending_actions(&mut merged_actions, &last_source, &mut pending_packages);
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
        system_actions.push(SystemAction::InstallPackage(
            dep_src.to_string(),
            dep_name.to_string(),
        ));
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
        package_actions.push(SystemAction::RunScript(
            package.post_install.clone().unwrap(),
        ));
    }
    if package.links.len() > 0 {
        package_actions.append(
            &mut (&package.links)
                .into_iter()
                .map(|link| SystemAction::CreateLink {
                    src: link.src.clone(),
                    dest: link.dest.clone(),
                    copy: link.copy,
                })
                .collect::<Vec<SystemAction>>(),
        );
    }
    return Ok(package_actions);
}
