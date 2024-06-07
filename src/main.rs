mod tokenizer;
mod file_utils;

use std::env;
use file_utils::read_file_to_string;
use tokenizer::Tokenizer;



fn main() {
    let file_path = "data/example.md";

    let content = match read_file_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut tokenizer = Tokenizer::new(&content);
    let tokens = tokenizer.tokenize();

    for token in tokens {
        println!("{:?}", token);
    }
}