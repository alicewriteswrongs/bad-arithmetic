pub mod parser;
pub mod tokenizer;

use tokenizer::tokenize;

fn main() {
    println!("{:#?}", tokenize("1 + 2 + 3"));
}
