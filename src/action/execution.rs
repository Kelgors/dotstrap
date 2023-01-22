use anyhow::Result;
use colored::Colorize;
use pathbuf::pathbuf;
use std::{
    os::unix::prelude::PermissionsExt,
    process::{Command, Output},
};

use crate::host::config::HostConfiguration;

use super::{FileOperation, PackageOperation, ScriptOperation, SystemAction};

pub fn execute_pm_command(command: &String, package_name: &String) {
    let args: Vec<String> = command
        .trim()
        .split(" ")
        .map(|str| str.to_string())
        .collect();
    let mut std_command = Command::new(&args[0]);
    for index in 1..args.len() {
        let part = args[index].trim();
        if part.trim().len() == 0 {}
        std_command.arg(if part == "<package>" {
            package_name
        } else {
            part
        });
    }
    handle_output(
        std_command
            .output()
            .expect(&format!("\"{}\" failed to start", command)),
    );
}

pub fn execute_script(script: &String) {
    let filepath = pathbuf![&std::env::temp_dir(), "dotstrap-tmp-script.sh"];
    std::fs::write(&filepath, script).expect(&format!(
        "Unable to write file {}",
        filepath.to_string_lossy()
    ));
    let mut perms = std::fs::metadata(&filepath)
        .expect(&format!(
            "Unable to read file permisions of {}",
            filepath.to_string_lossy()
        ))
        .permissions();
    perms.set_mode(0o744);
    std::fs::set_permissions(&filepath, perms).expect(&format!(
        "Unable to read file permisions of {}",
        filepath.to_string_lossy()
    ));
    let output = Command::new("sh")
        .arg(filepath.to_str().unwrap().to_string())
        .output()
        .expect(&format!(
            "Unable to run script in {}",
            filepath.to_string_lossy()
        ));
    std::fs::remove_file(&filepath).expect(&format!(
        "Unable to remove script in {}",
        filepath.to_string_lossy()
    ));
    handle_output(output);
}

fn handle_output(console_output: Output) {
    if console_output.status.success() {
        return;
    }
    eprintln!("{}", String::from_utf8(console_output.stderr).unwrap());
    panic!["Error during command"];
}

fn log_with_tag<T: std::fmt::Display>(tag: T, message: &String) {
    println!("[{}] {}", tag, message);
}

pub fn execute(
    sytem_actions: &Vec<SystemAction>,
    config: &HostConfiguration,
    dry_run: bool,
) -> Result<()> {
    let really_execute = !dry_run;
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
                let long_package_name = if "os".eq(source) {
                    name.clone()
                } else {
                    format!("{}:{}", source, name)
                };
                match operation {
                    PackageOperation::Install => {
                        log_with_tag("INSTALL".green(), &long_package_name);
                        if really_execute {
                            execute_pm_command(&pm.commands.install, name);
                        }
                    }
                    PackageOperation::Uninstall => {
                        log_with_tag("REMOVE".red(), &long_package_name);
                        if really_execute {
                            execute_pm_command(&pm.commands.uninstall, name);
                        }
                    }
                }
            }
            SystemAction::Script {
                operation,
                script,
                origin,
            } => match operation {
                ScriptOperation::Run => {
                    log_with_tag(
                        origin
                            .split(":")
                            .last()
                            .unwrap()
                            .to_uppercase()
                            .bright_purple(),
                        &script.trim().to_string(),
                    );
                    if really_execute {
                        execute_script(script);
                    }
                }
            },
            SystemAction::File {
                operation,
                src,
                dest,
                origin,
            } => {
                let src_path = pathbuf![&origin, src];
                match operation {
                    FileOperation::Link => {
                        log_with_tag("LINK".blue(), &format!("{} {}", src, dest));
                        if really_execute {
                            std::os::unix::fs::symlink(&src_path, dest).expect(&format!(
                                "Unable to symlink from {} to {}",
                                src_path.to_str().unwrap(),
                                dest
                            ));
                        }
                    }
                    FileOperation::Copy => {
                        log_with_tag("COPY".blue(), &format!("{} {}", src, dest));
                        if really_execute {
                            std::fs::copy(&src_path, dest).expect(&format!(
                                "Unable to copy from {} to {}",
                                src_path.to_str().unwrap(),
                                dest
                            ));
                        }
                    }
                    FileOperation::Remove => {
                        log_with_tag("DELETE".bright_red(), dest);
                        if really_execute {
                            std::fs::remove_file(dest)
                                .expect(&format!("Unable to remove file {}", dest));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
