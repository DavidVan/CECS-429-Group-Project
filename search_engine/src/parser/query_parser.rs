use std::io::{self, Read};
use std::process::exit;

pub struct QueryParser {

}

impl QueryParser {
    pub fn new() -> QueryParser {
        QueryParser {}
    }

    pub fn group_tokens(&self, input: &Vec<String>) -> Vec<Vec<String>> {
        let mut query_group = Vec::new();
        let mut token_group = Vec::new();
        let mut previous_token = String::new();
        for token in input {
            //println!("This is a token: {}", token);
            if token.len() == 1 && token.starts_with('+') {
                query_group.push(token_group);
                token_group = Vec::new();
                continue;
            }
            else if token.starts_with('(') && token.ends_with(')') && previous_token.len() != 0 {
                if (token.chars().nth(0).unwrap() == '\"') {
                    println!("quote mark");
                }
                else {
                    let mut inner_query : String = token.as_str().chars().skip(1).collect(); 
                    inner_query = inner_query.chars().take(inner_query.len() - 1).collect();
                    println!("{}", inner_query);
                    // token is already (term1 + term2) use regex to remove ()+?
                    let inner_token = self.tokenize_query(inner_query.as_str());
                    let inner_group = self.group_tokens(&inner_token);
                    for group in inner_group {
                        println!("Inner Group: {:?}", group);
                    }
                    // maybe I should push into the vec?
                }
            }
            token_group.push(token.clone());
            previous_token = token.clone();
        }
        query_group.push(token_group);
        query_group
    }

    pub fn tokenize_query(&self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut input_iter = input.split_whitespace();
        while let Some(token) = input_iter.next() {
            if token.starts_with("\"") && !token.starts_with("(\"") {
                let mut phrase_literal = String::new();
                phrase_literal.push_str(token.trim_matches('\"'));
                phrase_literal.push_str(" ");
                while let Some(next_token) = input_iter.next() {
                    if next_token.ends_with("\"") && !next_token.ends_with("\")") {
                        phrase_literal.push_str(next_token.trim_matches('\"'));
                        break;
                    }
                    phrase_literal.push_str(next_token);
                    phrase_literal.push_str(" ");
                }
                tokens.push(phrase_literal);
            }
            else if token.starts_with("(") {
                let mut phrase_literal = String::new();
                phrase_literal.push_str(token);
                phrase_literal.push_str(" ");
                while let Some(next_token) = input_iter.next() {
                    if next_token.ends_with(")") {
                        phrase_literal.push_str(next_token);
                        break;
                    }
                    phrase_literal.push_str(next_token);
                    phrase_literal.push_str(" ");
                }
                tokens.push(phrase_literal);
            } else {
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
        println!("FINAL RESULT: {:?}", tokens);
        tokens
    }
}
