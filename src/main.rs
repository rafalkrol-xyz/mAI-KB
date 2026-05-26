use std::fmt;
use clap::{Parser, Subcommand, ValueEnum};

/// Create a knowledge base from a given directory with a given provider.
/// Defaults to this directory (`.`) and AWS.
#[derive(Parser, Debug)]
#[command(name = "mkb")]
#[command(version, about = "Turn local folders into knowledge bases (KBs).", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all knowledge bases for a given provider
    #[command(name = "ls", alias = "list")]
    Ls {
        #[arg(short = 'p', long = "provider", default_value_t = Provider::Aws)]
        provider: Provider,
    }
}

/// Provider to use for the knowledge base, e.g. AWS, GCP, local
#[derive(ValueEnum, Clone, Debug)]
enum Provider {
    Aws,
    // Gcp,
    // Local,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Provider::Aws => write!(f, "aws"),
            // Provider::Gcp   => write!(f, "gcp"),
            // Provider::Local => write!(f, "local"),
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Ls { provider } => println!("Listing knowledge bases for provider: {}", provider),
    }
}
