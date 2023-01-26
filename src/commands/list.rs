use anyhow::Result;
use pathbuf::pathbuf;

use crate::package::PackageDefinition;

pub struct RunListOptions {}

pub fn run_list(hostname: String, _: RunListOptions) -> Result<()> {
    let path = pathbuf![&std::env::current_dir()?, "hosts", &hostname, "package.yml"];
    let definition = PackageDefinition::load(&path)?;

    println!(
        "{}",
        definition
            .dependencies
            .into_iter()
            .map(|dependency| dependency.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );

    return Ok(());
}
