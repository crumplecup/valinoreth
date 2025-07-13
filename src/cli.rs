#[derive(clap::Parser, derive_getters::Getters)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'c', long, help = "Command to execute.")]
    command: String,
}
