extern crate search_engine;
extern crate stemmer;

use std::collections::HashMap;
use stemmer::Stemmer;

use search_engine::index::inverted_index::InvertedIndex;
use search_engine::parser::query_parser::QueryParser;


use std::io::{stdin, stdout, Write};
fn main() {

    let mut stemmer = Stemmer::new("english").unwrap();
    println!("{}", stemmer.stem("consolingly"));

    let parser = QueryParser::new();
    let tokens = parser.handle_query("testing 1 2 3 \"hello 世界 world\" and \"hello 世界 world\"");

    for token in tokens {
        println!("{}", token);
    }
}
