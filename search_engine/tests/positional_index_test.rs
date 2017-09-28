extern crate search_engine;

use std::collections::HashMap;
use search_engine::index::positional_inverted_index::PositionalInvertedIndex;

#[test]
fn add_term() {

    let mut positional_inverted_index = PositionalInvertedIndex::new();

    let sentence_1 = "the quick brown fox jumps over the lazy dog";

    let sentence_2 = "the fox and the hound are friends";

    let sentence_3 = "get over there quick";

    let tokens_1 = sentence_1.split(" ");

    let tokens_2 = sentence_2.split(" ");

    let tokens_3 = sentence_3.split(" ");

    for (i, token) in tokens_1.enumerate() {
        let term = token;
        let docID = 1;
        let pos: u32 = i as u32;
        positional_inverted_index.addTerm(term, docID, pos);
    }
    println!("Testing term count...\nThere should be 8 terms...");
    assert_eq!(positional_inverted_index.get_term_count(), 8);

    for (i, token) in tokens_2.enumerate() {
        let term = token;
        let docID = 2;
        let pos: u32 = i as u32;
        positional_inverted_index.addTerm(term, docID, pos);
    }
    println!("Testing term count...\nThere should be 12 terms...");
    assert_eq!(positional_inverted_index.get_term_count(), 12);
    for (i, token) in tokens_3.enumerate() {
        let term = token;
        let docID = 3;
        let pos: u32 = i as u32;
        positional_inverted_index.addTerm(term, docID, pos);
    }
    println!("Testing term count...\nThere should be 14 terms...");
    assert_eq!(positional_inverted_index.get_term_count(), 14);
}
