extern crate search_engine;
extern crate stemmer;

use std::collections::HashMap;
use stemmer::Stemmer;

use search_engine::index::inverted_index::InvertedIndex;


use std::io::{stdin, stdout, Write};
fn main() {

    let mut stemmer = Stemmer::new("english").unwrap();
    println!("{}", stemmer.stem("consolingly"));

}
