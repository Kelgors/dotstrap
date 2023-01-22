use crate::{
    action::{FileOperation, PackageOperation, ScriptOperation, SystemAction},
    host::config::HostConfiguration,
};
use anyhow::Result;
use pathbuf::pathbuf;

pub fn generate_shell_script(
    sytem_actions: &Vec<SystemAction>,
    config: &HostConfiguration,
    hostname: &String,
) -> Result<Vec<String>> {
    let mut output = vec!["# Shell Generation".to_string()];
    for sysaction in sytem_actions.into_iter() {
        match sysaction {
            SystemAction::Package {
                operation,
                source,
                name,
            } => {
                let pm = config
                    .package_managers
                    .get(source)
                    .expect(&format!("Invalid source {}", source));
                output.push(match operation {
                    PackageOperation::Install => pm.commands.install.replace("<package>", &name),
                    PackageOperation::Uninstall => {
                        pm.commands.uninstall.replace("<package>", &name)
                    }
                });
            }
            SystemAction::Script { operation, script } => match operation {
                ScriptOperation::Run => {
                    output.push("# RunScript".to_string());
                    output.push(script.clone());
                }
            },
            SystemAction::File {
                operation,
                src,
                dest,
            } => {
                let pwd = std::env::current_dir()?;
                let src_path = pathbuf![&pwd, "packages", hostname, src];
                output.push(match operation {
                    FileOperation::Link => {
                        let src_path_string = src_path.to_str().unwrap();
                        format!("ln -sf {} {}", dest, src_path_string)
                    }
                    FileOperation::Copy => {
                        let src_path_string = src_path.to_str().unwrap();
                        format!("cp {} {}", src_path_string, dest)
                    }
                    FileOperation::Remove => {
                        format!("rm {}", dest)
                    }
                });
            }
        }
    }
    return Ok(output);
}
