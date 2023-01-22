use action::execution::execute;
use anyhow::Result;
use clap::Parser;
use pathbuf::pathbuf;
use promptly::prompt_default;
use std::env;
use std::fs;

mod action;
mod cli;
mod helpers;
mod host;
mod lockfile;
mod package;
mod resolver;

use crate::action::compact_mergeable_actions;
use crate::action::generation::generate_shell_script;
use crate::action::transform_package_to_actions;
use crate::action::SystemAction;
use crate::host::HostDefinition;
use crate::lockfile::get_cleaning_actions;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let machine_hostname = String::from(
        fs::read_to_string("/etc/hostname")
            .unwrap_or("".to_string())
            .trim(),
    );
    let pwd = env::current_dir()?;

    match args.action {
        Some(cli::Action::Init {}) => {
            std::fs::create_dir_all(pathbuf![&pwd, "packages", "flatpak"])?;
            std::fs::create_dir_all(pathbuf![&pwd, "packages", "tmux"])?;
            std::fs::create_dir_all(pathbuf![&pwd, "hosts", &machine_hostname])?;
            [
                (
                    pathbuf![&pwd, "hosts", &machine_hostname, "package.yml"],
                    host::DEFAULT_HOST_PACKAGE_CONTENT,
                ),
                (
                    pathbuf![&pwd, "hosts", &machine_hostname, "config.yml"],
                    host::DEFAULT_HOST_CONFIG_CONTENT,
                ),
                (
                    pathbuf![&pwd, "packages", "flatpak", "package.yml"],
                    package::DEFAULT_PACKAGE_EXAMPLE_FLATPAK,
                ),
                (
                    pathbuf![&pwd, "packages", "tmux", "package.yml"],
                    package::DEFAULT_PACKAGE_EXAMPLE_TMUX,
                ),
                (
                    pathbuf![&pwd, "packages", "tmux", "tmux.conf"],
                    package::DEFAULT_PACKAGE_EXAMPLE_TMUX_CONFIG,
                ),
            ]
            .iter()
            .for_each(|(path, content)| {
                std::fs::write(path, content).expect(&format!(
                    "Unable to write the file {}",
                    path.to_str().unwrap()
                ));
            });
        }
        Some(cli::Action::Generate { hostname }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            // Load host definition and prepare system actions from it
            let host_definition = HostDefinition::from_path(&pathbuf!["hosts", &hostname])?;
            let packages_repo = resolver::resolve_dependencies(&host_definition.package)?;
            let next_system_actions = transform_package_to_actions(
                &host_definition.package,
                &packages_repo,
                &mut vec![],
            )?;
            // TODO Err no handled correctly
            // merge next actions with cleaning actions
            let all_actions: Vec<SystemAction> =
                if let Ok(Some(cleaning_actions)) = get_cleaning_actions(&next_system_actions) {
                    [cleaning_actions, next_system_actions].concat()
                } else {
                    next_system_actions
                };
            // compacting actions when possible
            let merged_actions = compact_mergeable_actions(&all_actions, &host_definition.config);
            // generate shell script
            let script = generate_shell_script(&merged_actions, &host_definition.config)?;
            println!("{}", script.join("\n"));
        }
        Some(cli::Action::Install { hostname, dry }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            // Load host definition and prepare system actions from it
            let host_definition = HostDefinition::from_path(&pathbuf!["hosts", &hostname])?;
            let packages_repo = resolver::resolve_dependencies(&host_definition.package)?;
            let next_system_actions = transform_package_to_actions(
                &host_definition.package,
                &packages_repo,
                &mut vec![],
            )?;
            // save next lockfile before "cleaning" mutation
            let serialized_next_lockfile: String = serde_yaml::to_string(&next_system_actions)?;
            // TODO Err no handled correctly
            // merge next actions with cleaning actions
            let all_actions: Vec<SystemAction> =
                if let Ok(Some(cleaning_actions)) = get_cleaning_actions(&next_system_actions) {
                    [cleaning_actions, next_system_actions].concat()
                } else {
                    next_system_actions
                };
            // compacting actions when possible
            let merged_actions = compact_mergeable_actions(&all_actions, &host_definition.config);

            if dry {
                println!("DryMode: {}", dry);
            }
            let confirm_execution = prompt_default(
                format!("Do you want to apply {} operations?", all_actions.len()),
                false,
            )?;

            if confirm_execution {
                execute(&merged_actions, &host_definition.config, dry)?;
                if !dry {
                    std::fs::write(pathbuf![&pwd, ".lockfile"], &serialized_next_lockfile)?;
                }
            }
        }
        None => {}
    }
    return Ok(());
}
