extern crate search_engine;

use std::collections::HashMap;
use search_engine::index::inverted_index::InvertedIndex;

#[test]
fn inverted_index() {

    let mut inverted_index = InvertedIndex { mIndex: HashMap::new() };

    let mut term = "Test";

    for id in 1..10 {
        inverted_index.addTerm(&term, id);
        println!("There are {} in index", inverted_index.getTermCount());

        let postings = inverted_index.getPostings(&term);

        print!("{} : ", term);

        for p in postings {
            print!("{} ", p);
        }
        println!("\n");
    }
}
