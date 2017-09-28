extern crate search_engine;
extern crate stemmer;

use stemmer::Stemmer;
use search_engine::parser::document_parser;
use std::collections::HashMap;
#[test]
fn test_normalize() {

    let mut dictionary = HashMap::new();

    // dictionary.insert("swim", vec!["swimming","swimmer","swam"]);
    dictionary.insert("test", vec!["tested","tests","testing"]);
    dictionary.insert("needless", vec!["needlessly"]);
    dictionary.insert("fast", vec!["fasting","faster", "fastest"]);
    dictionary.insert("seed", vec!["seeding","seeds"]);

    for (term, variants) in dictionary.iter() {
        println!("Testing term: {}", term);
        for variant in variants {
            println!("Testing with variant: {}", variant);

            let results = document_parser::normalize_token(variant.to_string());

            let result = results.get(0).expect("Not a term");

            // println!("Size of result: {}", results.len());

            println!("{}\n", result);

            assert_eq!(term, result, "Result {} does not stem into {}", result, term);
        }
    }
}
