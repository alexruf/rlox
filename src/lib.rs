use crate::scanner::Scanner;
use std::fs;
use std::path::Path;

mod scanner;
mod token;

pub fn run_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    run(&content)?;
    Ok(())
}

pub fn run_repl() {
    println!("Run REPL");
}

fn run(source: &str) -> Result<(), String> {
    if source.is_empty() {
        return Err(String::from("Source code cannot be empty"));
    }

    dbg!(source);

    // Scan tokens
    let mut scanner = Scanner::new(source.trim().to_string());
    let tokens = scanner.scan_tokens();

    tokens.iter().for_each(|token| println!("{:?}", token));

    Ok(())
}
