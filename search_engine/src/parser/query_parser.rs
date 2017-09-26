use std::io::{self, Read};
use std::process::exit;

pub struct QueryParser {
    
}

impl QueryParser {
    pub fn new() -> QueryParser {
        QueryParser {}
    }

    pub fn handle_query(&self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut input_iter = input.split_whitespace();
        while let Some(mut token) = input_iter.next() {
            if token.starts_with("\"") {
                let mut phrase_literal = String::new();
                phrase_literal.push_str(token.trim_matches('\"'));
                phrase_literal.push_str(" ");
                while let Some(mut nextToken) = input_iter.next() {
                    if nextToken.ends_with("\"") {
                        phrase_literal.push_str(nextToken.trim_matches('\"'));
                        break;
                    }
                    phrase_literal.push_str(nextToken);
                    phrase_literal.push_str(" ");
                }
                tokens.push(phrase_literal);
            }
            else {
                tokens.push(String::from(token));
            }
        }
        /*match input.chars().nth(0) {
            Some(c) => match c {
                ":" => match input[1..].split(" ").collect() {

                }
            },
            None(_) => panic!("{Nothing was entered!}"),
        }*/
        tokens
    }
}
