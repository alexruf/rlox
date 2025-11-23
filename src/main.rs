use rlox as lib;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Parse command line arguments. The first argument is always the executable name - skip it.
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        println!("Usage: rlox [script]");
        ExitCode::from(64)
    } else if args.len() == 1 {
        // One argument provided. Assume it's the path to a script file.
        match lib::run_file(&args[0]) {
            Ok(_) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("Error: {}", err);
                ExitCode::FAILURE
            }
        }
    } else {
        // No arguments provided. Default to REPL mode.
        match lib::run_repl() {
            Ok(_) => ExitCode::SUCCESS,
            Err(err) => {
                eprintln!("Error: {}", err);
                ExitCode::FAILURE
            }
        }
    }
}
