extern crate search_engine;

use search_engine::index::positional_inverted_index::PositionalInvertedIndex;
use search_engine::index::k_gram_index::KGramIndex;
use search_engine::parser::document_parser;
use search_engine::paths::search_engine_paths;
use search_engine::processer::query_processer;
use std::collections::HashMap;
use std::collections::HashSet;

#[test]
fn test_queries() {
    let mut index_path = search_engine_paths::initializePath();
    search_engine_paths::changeDirectory(&mut index_path, "documents");
    let directory = index_path.to_str().expect("Invalid directory");
    let mut index = PositionalInvertedIndex::new();
    let mut k_gram_index = KGramIndex::new();
    let mut docid_file = document_parser::build_index(directory.to_string(), &mut index, &mut k_gram_index);

    let test_query_1 = "alpha"; // Tests simple query
    let test_query_2 = "alpha bravo"; // Tests query with AND operator
    let test_query_3 = "alpha -november"; // Tests query with NOT operator
    let test_query_4 = "alpha + mike"; // Tests query with OR operator

    let result_query_1 = query_processer::process_query(&test_query_1, &index, &docid_file);
    let result_key_1 = vec!["doc1.txt", "doc2.txt", "doc5.txt"];

    let result_query_2 = query_processer::process_query(&test_query_2, &index, &docid_file);
    let result_key_2 = vec!["doc1.txt", "doc2.txt"];

    let result_query_3 = query_processer::process_query(&test_query_3, &index, &docid_file);

    let result_query_4 = query_processer::process_query(&test_query_4, &index, &docid_file);
    let result_key_4 = vec!["doc1.txt", "doc2.txt", "doc3.txt", "doc4.txt", "doc5.txt"];

    for result in result_key_1 {
        assert!(result_query_1.contains(result));
    }

    for result in result_key_2 {
        assert!(result_query_2.contains(result));
    }

    assert!(result_query_3.is_empty());
    
    for result in result_key_4 {
        assert!(result_query_4.contains(result));
    }
}

#[test]
fn near_operator {

}
