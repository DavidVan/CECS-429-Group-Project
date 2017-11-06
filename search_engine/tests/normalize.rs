extern crate search_engine;
extern crate stemmer;

use search_engine::parser::document_parser;
use std::collections::HashMap;
#[test]
fn test_normalize() {

    let mut dictionary = HashMap::new();

    dictionary.insert("swim", vec!["swimming"]);
    dictionary.insert("swam", vec!["swam"]);
    dictionary.insert("swimmer", vec!["Swimmer"]);
    dictionary.insert("test", vec!["Tested", "tEsts", "Testing"]);
    dictionary.insert("needless", vec!["Needlessly"]);
    dictionary.insert("fast", vec!["Fasting"]);
    dictionary.insert("faster", vec!["Faster"]);
    dictionary.insert("seed", vec!["seeding", "seeds"]);

    for (term, variants) in dictionary.iter() {
        println!("Testing term: {}\n", term);
        for variant in variants {
            println!("Testing with variant: {}", variant);

            let normalized_results = document_parser::normalize_token(variant.to_string());
            let results = document_parser::stem_terms(normalized_results);
            
            let result = results.get(0).expect("Not a term");

            // println!("Size of result: {}", results.len());

            println!("Result - {}\n", result);

            assert_eq!(
                term,
                result,
                "Result {} does not stem into {}",
                result,
                term
            );
        }
    }
}
