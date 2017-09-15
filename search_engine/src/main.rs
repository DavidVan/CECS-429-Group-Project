extern crate search_engine;

use std::collections::HashMap;

use search_engine::porter_stemmer::{stemmer};
use search_engine::index::inverted_index::InvertedIndex;

fn main() {
    let inverted_index = InvertedIndex {
        mIndex : HashMap::new(),
    };


}
