extern crate search_engine;

use search_engine::parser::query_parser::QueryParser;

#[test]
fn test_parser() {
    let parser = QueryParser::new();
    let query = "testing 1 2 + 3 \"hello1 世界 world\" hi + \"hello2 世界 world\" test (hello3 + \"hello4 world\" (inner + \"hello5 world\" \"(still + in + same + group)\")) + hello (banana + strawberry) + bye";
    println!("Original Query: \n{}", query);
    let multiply_test = vec!["this", "that", "\"(who + am + i)\""];
    let multiply = parser.multiply_token(String::from("multiply"), &multiply_test);
    for item in multiply {
        println!("Multiply Item: {}", item);
    }
    let final_query = parser.process_query(query);
    /*println!("Original Query: {}", query);
    let tokens = parser.tokenize_query(query);
    println!("ALL ZE TOKENS: {:?}", tokens);
    let groups = parser.group_tokens(&tokens, None);
    println!("ALMOST FINISHED");
    for token_group in groups {
        for tokens in token_group {
            println!("OUR FINAL FINISHED TOKENS {} ", tokens);
        }
        //println!("Group {:?}", token_group);
    }
    println!("FINISHED");*/
    //let tokens = parser.tokenize_query(query, 1);
    //for token in tokens {
     //   println!("TOKEN: {:?}", token);
   // }
    //let query_group = parser.group_tokens(&tokens);
    /*for token_group in query_group {
        println!("Token Group: {}", token_group);
        for token in token_group {
            println!("Individual Token: {}", token);
        }
    }*/
}
