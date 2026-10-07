use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(
    name = "nyvrax",
    version,
    about = "AI-native security engine for software changes"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Scan the current repository.
    Scan {
        /// Scan only code changed from HEAD or --base.
        #[arg(long)]
        changed: bool,

        /// Compare against a Git base ref.
        #[arg(long)]
        base: Option<String>,

        /// Report output format.
        #[arg(
            long,
            value_enum,
            default_value_t = OutputFormat::Terminal
        )]
        format: OutputFormat,
    },

    /// Print the Nyvrax version.
    Version,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Terminal,
    Json,
    Sarif,
}
