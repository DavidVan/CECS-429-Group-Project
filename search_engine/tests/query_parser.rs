extern crate search_engine;

use search_engine::parser::query_parser::QueryParser;

#[test]
fn test_parser() {
    let parser = QueryParser::new();
    let query = "testing 1 2 + 3 \"hello 世界 world\" hi + \"hello 世界 world\" test (hello + \"hello world\" (inner + \"hello world\" \"(still + in + same + group)\"))";
    println!("Original Query: {}", query);
    let tokens = parser.tokenize_query(query);
    let groups = parser.group_tokens(&tokens);
    for token_group in groups {
        println!("Group {:?}", token_group);
    }
}
