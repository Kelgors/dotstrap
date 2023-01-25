use anyhow::Result;
use super::install::{run_install, RunInstallOptions};

pub struct RunAddOptions {
    pub package_names: Vec<String>,
    pub install: bool,
    pub commit: bool,
    pub push: bool,
}

pub fn run_add(hostname: String, options: RunAddOptions) -> Result<()> {
    println!("pkgs: {}", &options.package_names.join(",").to_string());
    if options.install {
        run_install(hostname.clone(), RunInstallOptions { dry: false })?;
    }
    if options.push || options.commit {
        // git commit -m "Add {packages} to {hostname}"
    }
    if options.push {
        // git push
    }
    return Ok(());
}