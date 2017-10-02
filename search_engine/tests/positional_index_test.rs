extern crate search_engine;

use std::collections::HashMap;
use search_engine::index::positional_inverted_index::PositionalInvertedIndex;
use search_engine::index::positional_inverted_index::PositionalPosting;
use search_engine::index::k_gram_index::KGramIndex;
use search_engine::parser::document_parser;
use search_engine::paths::search_engine_paths;

#[test]
fn add_term() {

    let mut positional_inverted_index = PositionalInvertedIndex::new();

    let sentence_1 = "The quick brown fox jumps over the lazy dog";

    let sentence_2 = "The fox and the hound are friends";

    let sentence_3 = "Get over there quick";

    let tokens_1 = sentence_1.split(" ");

    let tokens_2 = sentence_2.split(" ");

    let tokens_3 = sentence_3.split(" ");

    for (i, token) in tokens_1.enumerate() {
        let normalize_term = document_parser::normalize_token(token.to_string());
        let term = normalize_term.get(0).unwrap();
        let docID = 1;
        let pos: u32 = i as u32;
        positional_inverted_index.addTerm(term, docID, pos);
    }
    println!("Testing term count...\nThere should be 8 terms...");
    assert_eq!(positional_inverted_index.get_term_count(), 8);

    for (i, token) in tokens_2.enumerate() {
        let normalize_term = document_parser::normalize_token(token.to_string());
        let term = normalize_term.get(0).unwrap();
        let docID = 2;
        let pos: u32 = i as u32;
        positional_inverted_index.addTerm(term, docID, pos);
    }
    println!("Testing term count...\nThere should be 12 terms...");
    assert_eq!(positional_inverted_index.get_term_count(), 12);
    for (i, token) in tokens_3.enumerate() {
        let normalize_term = document_parser::normalize_token(token.to_string());
        let term = normalize_term.get(0).unwrap();
        let docID = 3;
        let pos: u32 = i as u32;
        positional_inverted_index.addTerm(term, docID, pos);
    }
    println!("Testing term count...\nThere should be 14 terms...");
    assert_eq!(positional_inverted_index.get_term_count(), 14);
}

#[test]
fn read_documents() {
    let mut index_path = search_engine_paths::initializePath();
    index_path.push("documents");
    let directory = index_path.to_str().expect("Invalid directory");
    let mut positional_inverted_index = PositionalInvertedIndex::new();
    let mut k_gram_index = KGramIndex::new();
    let docid_file = document_parser::build_index(directory.to_string(), &mut positional_inverted_index, &mut k_gram_index); 
    
    let alpha_postings_list = positional_inverted_index.get_postings("alpha");

    let mut test_postings_list = Vec::new();
    let test_case_1 = PositionalPosting {
        mDocID: 0,
        mPositions: vec![2]
    };
    let test_case_2 = PositionalPosting {
        mDocID: 1,
        mPositions: vec![5]
    };
    let test_case_3 = PositionalPosting {
        mDocID: 2,
        mPositions: vec![8]
    };
    test_postings_list.push(test_case_1);
    test_postings_list.push(test_case_2);
    test_postings_list.push(test_case_3);

}
