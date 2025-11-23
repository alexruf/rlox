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

pub fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("Run REPL");
    Ok(())
}

fn run(source: &str) -> Result<(), Box<dyn std::error::Error>> {
    if source.is_empty() {
        return Err(String::from("Source code cannot be empty").into());
    }

    dbg!(source);

    // Scan tokens
    let mut scanner = Scanner::new(source.trim().to_string());
    let tokens = scanner.scan_tokens();

    tokens.iter().for_each(|token| println!("{:?}", token));

    Ok(())
}
