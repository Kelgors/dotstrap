use crate::package::PackageCollection;
use crate::package::PackageDefinition;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum SystemAction {
    InstallPackage(String, String),
    RunScript(Vec<String>),
    CreateLink {
        src: String,
        dest: String,
        copy: bool,
    },
}

fn transform_package_deps_to_actions(
    package: &PackageDefinition,
    repo: &PackageCollection,
    loaded: &mut Vec<String>,
) -> Result<Vec<SystemAction>> {
    let mut dependencies_actions = vec![];
    let mut system_actions = vec![];

    if package.dependencies.is_none() {
        return Ok(dependencies_actions);
    }

    let package_deps = package.dependencies.as_ref().unwrap();
    for dependency in package_deps.into_iter() {
        if dependency.tagged.is_some() {
            continue;
        }
        let dep_name = dependency.name.as_ref().unwrap();
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
    if package.links.is_some() {
        package_actions.append(
            &mut package
                .links
                .as_ref()
                .unwrap()
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
