use clap::{Parser, Subcommand, *};

#[derive(Parser, Debug)]
#[clap(name = "dotstrap", author, version, about, long_about=None)]
pub struct Args {
    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Generate a sample dostrap configuration in the current directory
    Init {},
    /// Add packages to a host configuration
    Add {
        #[arg()]
        package_names: Vec<String>,
        /// Install the packages now
        #[arg(short, long, default_value_t = false)]
        install: bool,
        // Automatically commit after removal
        #[arg(short, long, default_value_t = false)]
        commit: bool,
        // Automatically push after removal
        #[arg(short, long, default_value_t = false)]
        push: bool,
    },
    /// Remove packages from a host configuration
    Remove {
        #[arg()]
        package_names: Vec<String>,
        /// Remove the packages now
        #[arg(short, long, default_value_t = false)]
        install: bool,
        // Automatically commit after removal
        #[arg(short, long, default_value_t = false)]
        commit: bool,
        // Automatically push after removal
        #[arg(short, long, default_value_t = false)]
        push: bool,
    },
    /// Generate a shell script from your configuration
    Generate {
        /// Override hostname, load specific hosts/<hostname/package.yml
        #[arg()]
        hostname: Option<String>,
        /// (Re)Install all packages
        #[arg(short, long, default_value_t = false)]
        full: bool,
    },
    /// Apply your hostname configuration
    Install {
        /// Override hostname, load specific hosts/<hostname/package.yml
        #[arg()]
        hostname: Option<String>,

        /// Don't perform actions on your system
        #[arg(short, long, default_value_t = false)]
        dry: bool,

        /// (Re)Install all packages
        #[arg(short, long, default_value_t = false)]
        full: bool,
    },
}
