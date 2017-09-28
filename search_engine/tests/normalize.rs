extern crate search_engine;
extern crate stemmer;

use stemmer::Stemmer;
use search_engine::parser::document_parser;
#[test]
fn test_term() {
    let term = "Swimming";

    println!("{}", term);

    let result = document_parser::normalize_token(term.to_string());

    println!("Size of result: {}", result.len());

    println!("{:?}\n", result);
}
