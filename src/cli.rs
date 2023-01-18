use clap::{Parser, Subcommand, *};

#[derive(Parser, Debug)]
#[clap(name = "dotstrap", author, version, about, long_about=None)]
#[command(next_line_help = true)]
pub struct Args {
    #[command(subcommand)]
    pub action: Option<Action>,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    Validate {
        #[arg(value_name = "hostname")]
        hostname: Option<String>,
    },

    Generate {
        #[arg(value_name = "hostname")]
        hostname: Option<String>,

        /// system root directory (default: /)
        #[arg(short, long)]
        root: Option<String>,
    },

    Install {
        #[arg(value_name = "hostname")]
        hostname: Option<String>,

        /// system root directory (default: /)
        #[arg(short, long)]
        root: Option<String>,

        /// verbose mode (default: false)
        #[arg(short, long)]
        verbose: Option<bool>,

        /// dry mode (default: false)
        #[arg(short, long)]
        dry: Option<bool>,
    },
}
