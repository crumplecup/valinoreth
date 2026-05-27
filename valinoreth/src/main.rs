use clap::Parser;
use elicitation::Generator;
use valinoreth::{trace_init, Cli, Players, ThreeDiceRoll};

fn main() {
    trace_init();
    let cli = Cli::parse();
    match cli.command().as_str() {
        "lobby" => {
            #[cfg(feature = "frontend-ratatui")]
            {
                let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
                if let Err(e) = rt.block_on(valinoreth::run_lobby()) {
                    eprintln!("Lobby error: {}", e);
                    std::process::exit(1);
                }
            }
            #[cfg(not(feature = "frontend-ratatui"))]
            tracing::warn!("lobby command requires the frontend-ratatui feature");
        }
        "paeva" => Players::paeva(),
        "prob" => {
            #[cfg(feature = "bin-extras")]
            {
                let prob = bio::stats::combinatorics::combinations_with_repl(6, 3);
                tracing::info!("6 choose 3 equals {}", prob);
            }
            #[cfg(not(feature = "bin-extras"))]
            tracing::warn!("prob command requires the bin-extras feature");
        }
        "roll" => {
            let roll = ThreeDiceRoll::random_generator(42).generate();
            tracing::info!("Roll is {}", roll.sum());
        }
        "tanithas" => Players::tanithas(),
        _ => tracing::info!("Command not recognized."),
    };
}
