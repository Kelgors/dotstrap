use anyhow::{Context, Result};
use clap::Parser;
use pathbuf::pathbuf;
use std::env;
use std::fs;

mod cli;
mod models;

use crate::models::package_definition::PackageDefinition;

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
            let host_package_path = pathbuf![&pwd, "hosts", &format!("{}.yml", hostname)];
            let definition = PackageDefinition::from_path(&host_package_path)?;
            println!("{:#?}", definition);
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
