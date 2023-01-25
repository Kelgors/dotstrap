use anyhow::Result;
use clap::Parser;
use commands::add::{run_add, RunAddOptions};
use commands::init::{run_init, RunInitOptions};
use commands::install::{run_install, RunInstallOptions};
use commands::remove::{run_remove, RunRemoveOptions};
use pathbuf::pathbuf;
use std::env;
use std::fs;

mod action;
mod cli;
mod commands;
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
use crate::lockfile::build_action_diff;

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
            run_init(machine_hostname, RunInitOptions {})?;
        }
        Some(cli::Action::Add {
            package_names,
            install,
            commit,
            push,
        }) => {
            run_add(machine_hostname, RunAddOptions {
                package_names,
                install,
                commit,
                push
            })?;
        }
        Some(cli::Action::Remove {
            package_names,
            install,
            commit,
            push,
        }) => {
            run_remove(machine_hostname, RunRemoveOptions {
                package_names,
                install,
                commit,
                push
            })?;
        }
        Some(cli::Action::Generate { hostname }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            // Load host definition and prepare system actions from it
            
        }
        Some(cli::Action::Install { hostname, dry }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            let options = RunInstallOptions { dry };
            run_install(hostname, options)?;
        }
        None => {}
    }
    return Ok(());
}
