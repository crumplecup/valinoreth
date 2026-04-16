//! Command-line interface for Valinoreth.

/// Command-line interface arguments.
///
/// # Examples
///
/// ```
/// use valinoreth::Cli;
/// use clap::Parser;
///
/// // Parse from command line arguments
/// // let cli = Cli::parse();
/// ```
#[derive(clap::Parser, derive_getters::Getters)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'c', long, help = "Command to execute.")]
    /// Command to execute
    command: String,
}
