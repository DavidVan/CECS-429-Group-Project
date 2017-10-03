extern crate search_engine;
use std::fs::File;
use std::io::Read;
use std::env::current_exe;
use std::path::Path;
use search_engine::index::k_gram_index::KGramIndex;
use search_engine::reader::read_file;

#[test]
fn test_castle() {
    let mut k_gram_index = KGramIndex::new();

    let castle = "castle";

    k_gram_index.checkIndex(castle);

    let test_cases = [
        "c",
        "a",
        "s",
        "t",
        "l",
        "e",
        "$c",
        "ca",
        "as",
        "st",
        "tl",
        "le",
        "e$",
        "$ca",
        "cas",
        "ast",
        "stl",
        "tle",
        "le$",
    ];

    let k_grams = k_gram_index.getKGrams();

    for gram in k_grams.iter() {
        println!("{}", gram);
    }

    for test_case in test_cases.iter() {
        let contain = k_grams.contains(&&test_case.clone().to_string());
        assert!(contain, "{} not in k_gram", test_case);
    }
}
