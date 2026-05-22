use clap::Parser;
use elicitation::Generator;
use valinoreth::{trace_init, Cli, Players, ThreeDiceRoll};

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
            let roll = ThreeDiceRoll::random_generator(42).generate();
            tracing::info!("Roll is {}", roll.sum());
        }
        "tanithas" => Players::tanithas(),
        _ => tracing::info!("Command not recognized."),
    };
}
