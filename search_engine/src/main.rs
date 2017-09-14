extern crate search_engine;

use search_engine::stemmer::{porter_stemmer};

fn main() {
    println!("{}", porter_stemmer::hello());
}
