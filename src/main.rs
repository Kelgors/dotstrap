use anyhow::Result;
use clap::Parser;
use commands::add::{run_add, RunAddOptions};
use commands::generate::{run_generate, RunGenerateOptions};
use commands::init::{run_init, RunInitOptions};
use commands::install::{run_install, RunInstallOptions};
use commands::list::{run_list, RunListOptions};
use commands::remove::{run_remove, RunRemoveOptions};
use std::fs;

mod action;
mod cli;
mod commands;
mod git;
mod helpers;
mod host;
mod lockfile;
mod package;
mod resolver;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let machine_hostname = String::from(
        fs::read_to_string("/etc/hostname")
            .unwrap_or("".to_string())
            .trim(),
    );

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
            run_add(
                machine_hostname,
                RunAddOptions {
                    package_names,
                    install,
                    commit,
                    push,
                },
            )?;
        }
        Some(cli::Action::Remove {
            package_names,
            install,
            commit,
            push,
        }) => {
            run_remove(
                machine_hostname,
                RunRemoveOptions {
                    package_names,
                    install,
                    commit,
                    push,
                },
            )?;
        }
        Some(cli::Action::List {}) => {
            run_list(machine_hostname, RunListOptions {})?;
        }
        Some(cli::Action::Generate { hostname, full }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            // Load host definition and prepare system actions from it
            run_generate(hostname, RunGenerateOptions { full })?;
        }
        Some(cli::Action::Install {
            hostname,
            dry,
            full,
            lock,
        }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            run_install(hostname, RunInstallOptions { dry, full, lock })?;
        }
        None => {}
    }
    return Ok(());
}
