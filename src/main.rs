use clap::Parser;

/// Create a knowledge base from a given directory with a given provider.
/// Defaults to this directory (`.`) and AWS.
#[derive(Parser, Debug)]
struct Cli {
    /// Path to the directory to create the knowledge base from
    #[arg(short = 'd', long = "dir", default_value = ".")]
    dir: std::path::PathBuf,
    /// Provider to use for the knowledge base, e.g. AWS, GCP, local
    #[arg(short = 'p', long = "provider", default_value = "aws")]
    provider: String, // TODO: change to an enum
}

fn main() {
    let args = Cli::parse();
    
    println!("{:?}", args);
}
