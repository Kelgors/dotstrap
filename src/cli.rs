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
    /// Ensure your configuration is correct
    Validate {
        /// Override hostname, load specific hosts/<hostname/package.yml
        #[arg(value_name = "hostname")]
        hostname: Option<String>,
    },
    /// Generate a shell script from your configuration
    Generate {
        /// Override hostname, load specific hosts/<hostname/package.yml
        #[arg(value_name = "hostname")]
        hostname: Option<String>,
    },
    /// Apply your hostname configuration
    Install {
        /// Override hostname, load specific hosts/<hostname/package.yml
        #[arg(value_name = "hostname")]
        hostname: Option<String>,

        /// verbose mode (default: false)
        #[arg(short, long)]
        verbose: Option<bool>,

        /// dry mode (default: false)
        #[arg(short, long)]
        dry: Option<bool>,
    },
}
