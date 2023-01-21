use anyhow::Result;
use clap::Parser;
use pathbuf::pathbuf;
use std::env;
use std::fs;

mod action;
mod cli;
mod generation;
mod host;
// mod lockfile;
mod package;
mod resolver;

use crate::action::compact_mergeable_actions;
use crate::action::transform_package_to_actions;
use crate::action::SystemAction;
use crate::generation::generate_shell_script;
use crate::host::HostDefinition;
// use crate::lockfile::create_lockfile;
// use crate::lockfile::Lockfile;

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
            std::fs::write(
                pathbuf![&pwd, "hosts", &machine_hostname, "package.yml"],
                host::DEFAULT_HOST_PACKAGE_CONTENT,
            )?;
            std::fs::write(
                pathbuf![&pwd, "hosts", &machine_hostname, "config.yml"],
                host::DEFAULT_HOST_CONFIG_CONTENT,
            )?;
            std::fs::write(
                pathbuf![&pwd, "packages", "flatpak", "package.yml"],
                package::DEFAULT_PACKAGE_EXAMPLE_FLATPAK,
            )?;
            std::fs::write(
                pathbuf![&pwd, "packages", "tmux", "package.yml"],
                package::DEFAULT_PACKAGE_EXAMPLE_TMUX,
            )?;
            std::fs::write(
                pathbuf![&pwd, "packages", "tmux", "tmux.conf"],
                package::DEFAULT_PACKAGE_EXAMPLE_TMUX_CONFIG,
            )?;
        }
        Some(cli::Action::Validate { hostname }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            println!("Validate for {}", hostname);
        }
        Some(cli::Action::Generate { hostname }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            println!("Generate for {}", hostname);
            let definition = HostDefinition::from_path(&pathbuf![&pwd, "hosts", &hostname])?;
            let repo = resolver::resolve_dependencies(&definition.package)?;

            // let lockfile = create_lockfile(&definition.package, &repo);
            // let clean_actions = if let Ok(old_lockfile) = Lockfile::load() {
            //     // generate cleaning actions from lockfiles
            //     let mut output = vec![];
            //     output.push(SystemAction::UninstallPackage(
            //         "".to_string(),
            //         "".to_string(),
            //     ));
            //     Some(output)
            // } else {
            //     None
            // };

            let actions = transform_package_to_actions(&definition.package, &repo, &mut vec![])?;
            let merged_actions = compact_mergeable_actions(&actions, &definition.config);
            let script = generate_shell_script(&merged_actions, &definition.config)?;
            // println!("{:#?}\n\n\n\n", definition);
            // println!("{:#?}\n\n\n\n", repo);
            // println!("{:#?}", &merged_actions);
            // let lockfile = std::fs::write(pathbuf![&pwd, ".lockfile"], &lockfile)?;

            // println!("{:#?}", &clean_actions);

            println!("{}", script.join("\n"));
        }
        Some(cli::Action::Install {
            hostname,
            verbose,
            dry,
        }) => {
            let hostname = hostname.unwrap_or(machine_hostname);
            println!(
                "Install for {} (verbose: {}, dry: {})",
                hostname,
                verbose.unwrap_or(false),
                dry.unwrap_or(false)
            );
        }
        None => {}
    }
    return Ok(());
}
