use crate::{
    action::{FileOperation, PackageOperation, ScriptOperation, SystemAction},
    host::config::HostConfiguration,
};
use anyhow::Result;
use pathbuf::pathbuf;

fn add_comment(new_comment: String, last_comment: &String, output: &mut Vec<String>) -> String {
    if new_comment.ne(last_comment) {
        output.push(new_comment.clone());
        return new_comment;
    }
    return last_comment.clone();
}

pub fn generate_shell_script(
    sytem_actions: &Vec<SystemAction>,
    config: &HostConfiguration,
) -> Result<Vec<String>> {
    let mut last_comment = String::new();
    let mut output = vec!["# Shell Generation".to_string()];
    for sysaction in sytem_actions.into_iter() {
        match sysaction {
            SystemAction::Package {
                operation,
                source,
                name,
                origin,
            } => {
                let pm = config
                    .package_managers
                    .get(source)
                    .expect(&format!("Invalid source {} from {}", source, origin));
                last_comment = add_comment(
                    format!("# {}:dependencies", &origin),
                    &last_comment,
                    &mut output,
                );
                output.push(match operation {
                    PackageOperation::Install => pm.commands.install.replace("<package>", &name),
                    PackageOperation::Uninstall => {
                        pm.commands.uninstall.replace("<package>", &name)
                    }
                });
            }
            SystemAction::Script {
                operation,
                script,
                origin,
            } => match operation {
                ScriptOperation::Run => {
                    last_comment = add_comment(format!("# {}", origin), &last_comment, &mut output);
                    output.push(script.clone());
                }
            },
            SystemAction::File {
                operation,
                src,
                dest,
                origin,
            } => {
                last_comment =
                    add_comment(format!("# {}:links", &origin), &last_comment, &mut output);
                let src_path = pathbuf![&origin, src];
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
