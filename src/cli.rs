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
    /// Generate a shell script from your configuration
    Generate {
        /// Override hostname, load specific hosts/<hostname/package.yml
        #[arg()]
        hostname: Option<String>,
    },
    /// Apply your hostname configuration
    Install {
        /// Override hostname, load specific hosts/<hostname/package.yml
        #[arg()]
        hostname: Option<String>,

        #[arg(short, long)]
        dry: bool,
    },
}
