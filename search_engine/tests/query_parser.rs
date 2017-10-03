extern crate search_engine;

use search_engine::parser::query_parser::QueryParser;

#[test]
fn test_parser() {
    let parser = QueryParser::new();
    let query = "testing 1 2 + 3 \"hello1 世界 world\" hi + \"hello2 世界 world\" test (hello3 + \"hello4 world\" (inner + \"hello5 world\" \"(still + in + same + group)\")) + hello (banana + strawberry) + bye";
    println!("Original Query: \n{}", query);
    let new_query = "hey -y + \"this one\"  + \"hello world\" -y (hello world + (hello2 world2 + hello3 world3) + hello4 world4) + bye";
    println!("New Query: {}", new_query);
    let results = parser.process_query(new_query);
    //parser.multiply_query("\"hello world\" -y (1 + (2 + 3 (4 + 5)) + 6)");
    println!("Process Query Results: {}", results.join(" + "));
    let multi = parser.multiply_query("hello NEAR\\2 david -y (1 world + 2 world)");
    println!("Process Multi Results: {}", multi.join(" + "));
    //parser.multiply_query("hello world (what + what2)");
    /*let multiply_test = vec![String::from("this"), String::from("that"), String::from("\"(who + am + i)\"")];
    let multiply = parser.multiply_token(String::from("hello"), &multiply_test);
    for item in multiply {
        println!("Multiply Item: {}", item);
    }
    

    let parenthesis_remove = parser.parenthesis_query_to_vec(String::from("(hello + world (hello2 + world2))"));
    println!("Parenthesis remove test: {:?}", parenthesis_remove);

    // hello * this + that + (real + parenthesis + test (hello + world))
    // hello this + hello that (real + parenthesis + test (hello + world))
    // hello this + hello that real + hello that parenthesis + hello that test (hello + world)
    let multiply_test2 = vec![String::from("this"), String::from("that"), String::from("(real + parenthesis + test (hello + world))")];
    let multiply2 = parser.multiply_token(String::from("hello"), &multiply_test2);
    for item in multiply2 {
        println!("Multiply2 Item: {}", item);
    }*/

    //let final_query = parser.process_query(query);
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
