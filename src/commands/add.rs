use anyhow::Result;
use git2::Repository;
use pathbuf::pathbuf;
use std::str::FromStr;

use super::install::{run_install, RunInstallOptions};
use crate::{
    git::add_and_commit,
    package::{DependencyDefinition, PackageDefinition},
};

pub struct RunAddOptions {
    pub package_names: Vec<String>,
    pub install: bool,
    pub commit: bool,
    pub push: bool,
}

pub fn run_add(hostname: String, options: RunAddOptions) -> Result<()> {
    let path = pathbuf!["hosts", &hostname, "package.yml"];
    let mut definition = PackageDefinition::load(&path)?;
    let prev_dependencies_count = definition.dependencies.len();
    let mut new_dependencies: Vec<DependencyDefinition> = (&options.package_names)
        .into_iter()
        .map(|name| {
            return DependencyDefinition::from_str(&name)
                .expect(&format!("Unable to parse package {}", &name));
        })
        .collect();
    definition.dependencies.append(&mut new_dependencies);
    definition.save()?;
    println!(
        "{} dependencies added to {}",
        definition.dependencies.len() - prev_dependencies_count,
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
        let commit_oid = add_and_commit(
            &repo,
            &path,
            &build_commit_message(&hostname, &options.package_names),
        )
        .expect("Unable to commit properly");
        if options.push {
            // git push
        }
    }
    return Ok(());
}

fn build_commit_message(hostname: &String, package_names: &Vec<String>) -> String {
    if package_names.len() == 1 {
        return format!("Add {} to {}", package_names.get(0).unwrap(), hostname);
    }
    return format!(
        "Add packages to {}\n- {}",
        hostname,
        package_names.join("\n- ")
    );
}
