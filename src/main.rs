use clap::{Parser, Subcommand, ValueEnum};
use std::fmt;

/// Create a knowledge base from a given directory with a given provider.
/// Defaults to this directory (`.`) and AWS.
#[derive(Parser, Debug)]
#[command(name = "mkb")]
#[command(version, about = "Turn local folders into knowledge bases (KBs).", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// List all knowledge bases for a given provider
    #[command(name = "ls", alias = "list")]
    Ls {
        #[arg(short = 'p', long = "provider", default_value_t = Provider::Aws)]
        provider: Provider,
    },
}

/// Provider to use for the knowledge base, e.g. AWS, GCP, local
#[derive(ValueEnum, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_ls_defaults_to_aws() {
        // We pass "mkb" as the first argument because it's the program name
        let args = Cli::try_parse_from(["mkb", "ls"])
            .expect("The `ls` command should be a valid command for the mkb cli tool");
        // Check that it parsed as the Ls variant and that the provider is Aws
        match args.command {
            Commands::Ls { provider } => assert_eq!(provider, Provider::Aws),
        }
    }
}
