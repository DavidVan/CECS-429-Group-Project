extern crate search_engine;
use search_engine::index::k_gram_index::KGramIndex;

#[test]
fn test_castle() {
    let mut k_gram_index = KGramIndex::new();

    // Term that will be tested
    let castle = vec!["castle"];

    // Builds index according to term
    k_gram_index.check_terms(castle);

    // Expected values in test cases
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

    let k_grams = k_gram_index.get_k_grams();

    for gram in k_grams.iter() {
        println!("{}", gram);
    }

    for test_case in test_cases.iter() {
        let contain = k_grams.contains(&&test_case.clone().to_string());
        assert!(contain, "{} not in k_gram", test_case);
    }
}
