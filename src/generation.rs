use crate::{action::SystemAction, host::config::HostConfiguration};
use anyhow::Result;

pub fn generate_shell_script(
    actions: &Vec<SystemAction>,
    config: &HostConfiguration,
) -> Result<Vec<String>> {
    let mut output = vec!["# Shell Generation".to_string()];
    for action in actions.into_iter() {
        match action {
            SystemAction::InstallPackage(source, package) => {
                let pm = config
                    .package_managers
                    .get(source)
                    .expect(&format!("Invalid source {}", source));
                output.push(pm.commands.install.replace("<package>", &package));
            }
            SystemAction::UninstallPackage(source, package) => {
                let pm = config
                    .package_managers
                    .get(source)
                    .expect(&format!("Invalid source {}", source));
                output.push(pm.commands.uninstall.replace("<package>", &package));
            }
            SystemAction::RunScript(scripts) => {
                output.push("# RunScript".to_string());
                output.push(scripts.clone());
            }
            SystemAction::CreateLink { src, dest, copy } => {
                output.push(format!("# CreateLink between {} and {}", src, dest));
                if copy.clone() {
                    output.push(format!("cp {} {}", src, dest));
                } else {
                    output.push(format!("ln -sf {} {}", dest, src));
                }
            }
            SystemAction::DeleteLink(file) => {
                output.push(format!("rm {}", file));
            }
        }
    }
    return Ok(output);
}
