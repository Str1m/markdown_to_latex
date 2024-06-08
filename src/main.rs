mod file_utils;
mod latex_converter;
mod tokenizer;

use file_utils::{read_file_to_string, write_to_file};
use latex_converter::LatexConverter;
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

    let latex_content = LatexConverter::convert(tokens);
    match write_to_file(latex_content, "data/output.tex") {
        Ok(_) => println!("Tex was saved"),
        Err(e) => println!("Error: {}", e),
    }
}
