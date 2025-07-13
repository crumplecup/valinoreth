use clap::Parser;
use valinoreth::{trace_init, Cli, Players, Random};

fn main() {
    trace_init();
    let cli = Cli::parse();
    match cli.command().as_str() {
        "paeva" => Players::paeva(),
        "prob" => {
            let prob = bio::stats::combinatorics::combinations_with_repl(6, 3);
            tracing::info!("6 choose 3 equals {}", prob);
        }
        "roll" => {
            let mut random = Random::default();
            tracing::info!("Roll is {}", random.roll());
        }
        "tanithas" => Players::tanithas(),
        _ => tracing::info!("Command not recognized."),
    };
}
