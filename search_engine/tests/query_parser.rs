extern crate search_engine;

use search_engine::parser::query_parser::QueryParser;

#[test]
fn test_parser() {
    let parser = QueryParser::new();
    let query = "testing 1 2 + 3 \"hello1 世界 world\" hi + \"hello2 世界 world\" test (hello3 + \"hello4 world\" (inner + \"hello5 world\" \"(still + in + same + group)\"))";
    println!("Original Query: {}", query);
    let tokens = parser.tokenize_query(query);
    let groups = parser.group_tokens(&tokens, None);
    println!("ALMOST FINISHED");
    for token_group in groups {
        for tokens in token_group {
            println!("OUR FINAL FINISHED TOKENS {} ", tokens);
        }
        //println!("Group {:?}", token_group);
    }
    //println!("FINAL FUCKING RESULT {:?}" groups);
    println!("FINISHED");
}
