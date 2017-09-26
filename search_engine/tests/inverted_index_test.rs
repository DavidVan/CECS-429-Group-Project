extern crate search_engine;

use std::collections::HashMap;
use search_engine::index::inverted_index::InvertedIndex;

#[test]
fn inverted_index() {

    let mut inverted_index = InvertedIndex { m_index: HashMap::new() };

    let mut term = "Test";

    for id in 1..10 {
        inverted_index.add_term(&term, id);
        println!("There are {} in index", inverted_index.get_term_count());

        let postings = inverted_index.get_postings(&term);

        print!("{} : ", term);

        for p in postings {
            print!("{} ", p);
        }
        println!("\n");
    }
}
