use anyhow::Result;
use pathbuf::pathbuf;
use std::{fs,env};

use crate::{host,package};

pub struct RunInitOptions {

}

pub fn run_init(hostname: String, options: RunInitOptions) -> Result<()> {
    let pwd = env::current_dir()?;
    fs::create_dir_all(pathbuf![&pwd, "packages", "flatpak"])?;
    fs::create_dir_all(pathbuf![&pwd, "packages", "tmux"])?;
    fs::create_dir_all(pathbuf![&pwd, "hosts", &hostname])?;
    // TODO Don't overwrite files
    // TODO How initiate only a host
    [
        (
            pathbuf![&pwd, "hosts", &hostname, "package.yml"],
            host::DEFAULT_HOST_PACKAGE_CONTENT,
        ),
        (
            pathbuf![&pwd, "hosts", &hostname, "config.yml"],
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
        fs::write(path, content).expect(&format!(
            "Unable to write the file {}",
            path.to_str().unwrap()
        ));
    });
    return Ok(());
}