use crate::scanner::Scanner;

mod scanner;
mod token;

pub fn run_file(path: &str) {
    println!("Run file {}", path);
    let source = std::fs::read_to_string(path).expect("Failed to read file");
    run(&source).expect("Failed to run script");
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
