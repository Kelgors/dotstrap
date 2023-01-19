use crate::{action::SystemAction, host::config::HostConfiguration};
use anyhow::Result;

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

fn compact_mergeable_actions(
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
        flush_pending_actions(&mut merged_actions, &mut last_source, &mut pending_packages);
    }
    return merged_actions;
}

pub fn generate_shell_script(
    actions: &Vec<SystemAction>,
    config: &HostConfiguration,
) -> Result<Vec<String>> {
    let mut output = vec!["# Shell Generation".to_string()];
    let merged_actions = compact_mergeable_actions(actions, config);
    for action in merged_actions.into_iter() {
        match action {
            SystemAction::InstallPackage(source, package) => {
                let pm = config
                    .package_managers
                    .get(&source)
                    .expect(&format!("Invalid source {}", &source));
                output.push(pm.commands.install.replace("<package>", &package));
            }
            SystemAction::RunScript(scripts) => {
                output.push("# RunScript".to_string());
                for line in scripts {
                    output.push(line.clone());
                }
            }
            SystemAction::CreateLink { src, dest, copy } => {
                output.push(format!("# CreateLink between {} and {}", src, dest));
                if copy.clone() {
                    output.push(format!("cp {} {}", src, dest));
                } else {
                    output.push(format!("ln -sf {} {}", dest, src));
                }
            }
        }
    }
    return Ok(output);
}
