use anyhow::Result;
use clap::Parser;
use pathbuf::pathbuf;
use std::env;
use std::fs;

mod action;
mod cli;
mod generation;
mod host;
mod package;
mod resolver;

use crate::action::compact_mergeable_actions;
use crate::action::transform_package_to_actions;
use crate::generation::generate_shell_script;
use crate::host::HostDefinition;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    let machine_hostname = String::from(
        fs::read_to_string("/etc/hostname")
            .unwrap_or("".to_string())
            .trim(),
    );
    let pwd = env::current_dir()?;

    match args.action {
        Some(cli::Action::Validate { hostname }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            println!("Validate for {}", hostname);
        }
        Some(cli::Action::Generate { hostname, root }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            println!(
                "Generate for {} (root: {})",
                hostname,
                root.unwrap_or("/".to_string())
            );
            let definition = HostDefinition::from_path(&pathbuf![&pwd, "hosts", &hostname])?;
            let repo = resolver::resolve_dependencies(&definition.package)?;
            let actions = transform_package_to_actions(&definition.package, &repo, &mut vec![])?;
            let merged_actions = compact_mergeable_actions(&actions, &definition.config);
            let script = generate_shell_script(&merged_actions, &definition.config)?;
            // println!("{:#?}\n\n\n\n", definition);
            // println!("{:#?}\n\n\n\n", repo);
            // println!("{:#?}", actions);
            println!("{}", script.join("\n"));
        }
        Some(cli::Action::Install {
            hostname,
            root,
            verbose,
            dry,
        }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            println!(
                "Install for {} (root: {}, verbose: {}, dry: {})",
                hostname,
                root.unwrap_or("/".to_string()),
                verbose.unwrap_or(false),
                dry.unwrap_or(false)
            );
        }
        None => {
            println!("{:?}", args);
        }
    }
    return Ok(());
}
