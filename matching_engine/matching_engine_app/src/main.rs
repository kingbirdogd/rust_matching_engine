use std::io::{self, BufRead};

use matching_engine_core::LineMatchingEngine;

fn main() {
    let mut engine = LineMatchingEngine::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        match handle.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let line = buffer.trim_end_matches(['\r', '\n']);
                let result = engine.process_line(line);

                for message in &result.outputs {
                    println!("{}", message.to_csv());
                }

                for error in &result.errors {
                    eprintln!("{error}");
                }
            }
            Err(_) => {
                eprintln!("Failed to read stdin");
                std::process::exit(1);
            }
        }
    }
}
