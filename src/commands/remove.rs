use anyhow::Result;
use git2::Repository;
use pathbuf::pathbuf;
use std::str::FromStr;

use super::install::{run_install, RunInstallOptions};
use crate::{
    git,
    package::{DependencyDefinition, PackageDefinition},
};

pub struct RunRemoveOptions {
    pub package_names: Vec<String>,
    pub install: bool,
    pub commit: bool,
    pub push: bool,
}

pub fn run_remove(hostname: String, options: RunRemoveOptions) -> Result<()> {
    println!("pkgs: {}", &options.package_names.join(",").to_string());

    let path = pathbuf![&std::env::current_dir()?, "hosts", &hostname, "package.yml"];
    let mut definition = PackageDefinition::load(&path)?;
    let prev_dependencies_count = definition.dependencies.len();
    let old_dependencies: Vec<DependencyDefinition> = (&options.package_names)
        .into_iter()
        .map(|name| {
            DependencyDefinition::from_str(&name)
                .expect(&format!("Unable to parse package {}", &name))
        })
        .collect();
    definition.dependencies = definition
        .dependencies
        .into_iter()
        .filter(|dependency| !old_dependencies.contains(dependency))
        .collect();
    definition.save()?;
    println!(
        "{} dependencies removed from {}",
        prev_dependencies_count - definition.dependencies.len(),
        &hostname
    );

    if options.install {
        run_install(
            hostname.clone(),
            RunInstallOptions {
                dry: false,
                full: false,
                lock: false,
            },
        )?;
    }
    if options.push || options.commit {
        let repo = Repository::open(std::env::current_dir()?)?;
        git::add_and_commit(
            &repo,
            &path,
            &build_commit_message(&hostname, &options.package_names),
        )
        .expect("Unable to commit properly");
        if options.push {
            git::push(&repo)?;
        }
    }
    return Ok(());
}

fn build_commit_message(hostname: &String, package_names: &Vec<String>) -> String {
    if package_names.len() == 1 {
        return format!("Remove {} from {}", package_names.get(0).unwrap(), hostname);
    }
    return format!(
        "Remove packages from {}\n- {}",
        hostname,
        package_names.join("\n- ")
    );
}
