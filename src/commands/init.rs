use anyhow::Result;
use pathbuf::pathbuf;
use std::{env, fs};

use crate::{host, package};

pub struct RunInitOptions {}

pub fn run_init(hostname: String, _: RunInitOptions) -> Result<()> {
    let pwd = env::current_dir()?;
    fs::create_dir_all(pathbuf![&pwd, "packages", "flatpak"])?;
    fs::create_dir_all(pathbuf![&pwd, "packages", "tmux"])?;
    fs::create_dir_all(pathbuf![&pwd, "hosts", &hostname])?;

    // TODO Make possible to initiate only a host
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
        // Don't overwrite files
        if path.metadata().is_err() && path.symlink_metadata().is_err() {
            fs::write(path, content).expect(&format!(
                "Unable to write the file {}",
                path.to_str().unwrap()
            ));
        }
    });
    return Ok(());
}
