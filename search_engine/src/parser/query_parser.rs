use std::io::{self, Read};
use std::process::exit;
use std::option;

pub struct QueryParser {

}

impl QueryParser {
    pub fn new() -> QueryParser {
        QueryParser {}
    }

    pub fn multiply_token(&self, multiplier: String, multiplicand: &Vec<&str>) -> Vec<String> {
        let mut results = Vec::new();
        for item in multiplicand {
            results.push(multiplier.clone() + " " + item);
        }
        results // All Results need to be OR'd / Insert '+' between.
    }

    pub fn process_query(&self, input: &str) -> Vec<Vec<String>> {
        let mut query = input;
        let mut final_query = Vec::new();

        let mut query_iter = query.split_whitespace(); // Split on whitespace
        let mut preprocessed_query = Vec::new(); // Hold tokens until we reach a '+' sign...
        while let Some(sub_query) = query_iter.next() { // Go through every token / sub-query
            if sub_query.len() == 1 && sub_query.starts_with("+") { // If it's a '+' sign...
                final_query.push(preprocessed_query); // Push into our results vector...
                preprocessed_query = Vec::new(); // Reset it
                continue; // Skip '+' signs...
            }
            if sub_query.starts_with("(") {
                //let mut extended_query : String = sub_query.clone().chars().skip(1).take(sub_query.len() - 1).collect();
                // Right now, I only have "(hello3"...
                // Get the rest of it, and make sure I get correct number of parenthesis...
                let mut query_builder = Vec::new();
                query_builder.push(sub_query);
                let mut left_parenthesis_counter = 1;
                let mut right_parenthesis_counter = 0;
                let mut next_sub_query = query_iter.next().unwrap();
                while left_parenthesis_counter > right_parenthesis_counter {
                    query_builder.push(next_sub_query.clone());
                    if next_sub_query.starts_with("(") {
                        left_parenthesis_counter += 1;
                    }
                    else if next_sub_query.ends_with(")") {
                        right_parenthesis_counter += 1;
                        left_parenthesis_counter -= 1;
                    }
                    next_sub_query = query_iter.next().unwrap();
                }
                preprocessed_query.push(query_builder.join(" "));
                println!("Built Query: {}", query_builder.join(" "));
                final_query.push(preprocessed_query);
                preprocessed_query = Vec::new();
                continue;
                //println!("Extended Query: {}", extended_query);
                //let multiplier = preprocessed_query.join(" ");
                //println!("Multiplier: {}", multiplier);
                //let multiply = self.multiply_token(multiplier, extended_query); 
            }
            preprocessed_query.push(String::from(sub_query));
        }
        final_query.push(preprocessed_query);

        for i in final_query.clone() {
            println!("Final Query: {:?}", i);
        }
        final_query
    }
    /*pub fn group_tokens(&self, input: &Vec<String>, previous_token_head: Option<&String>) -> Vec<Vec<String>> {
        let mut query_group = Vec::new();
        let mut token_group = Vec::new();
        let mut previous_token = String::new(); /*match previous_token_head {
            Some(token) => token.clone(),
            None => String::new(),
        };*/
        println!("GROUP USING PREV TOK {}", previous_token);
        for mut raw_token in input {
            let mut token = raw_token.clone();
            if token.starts_with('(') {
                println!("OH SHIT: {}", token);
                /*token = String::from(token.trim_left_matches("(").trim_right_matches(")"));
                let inner_query = self.tokenize_query(token.as_str());
                let inner_group = self.group_tokens(&inner_query, Some(&previous_token));

                for inner_token_group in inner_group {
                    let mut test = String::new();
                    test = inner_token_group.join(&String::from(""));
                    println!("INNER JOIN LOL: {:?}", test);
                    for mut inner_token in inner_token_group {
                        println!("Inner Token: {:?}", inner_token);
                    }
                }*/
                /*token = String::from(token.trim_left_matches("(").trim_right_matches(")"));
                //println!("{}", new_query);
                let inner_query = self.tokenize_query(token.as_str());
                let inner_group = self.group_tokens(&inner_query, Some(&previous_token));
                for inner_token_group in inner_group {
                    for mut inner_token in inner_token_group {
                        println!("Inner Token: {:?}", inner_token);
                        if inner_token.len() == 1 && inner_token.starts_with("+") {
                            continue;
                            //break;
                        }
                        inner_token = format!("{}{}", previous_token, inner_token);
                        token_group.push(inner_token);
                    }
                }
                //continue;
                //return query_group;
                //println!("OH SHIT INNER GROUP {:?}", inner_group);*/ 
            }
            else {
                println!("This is a token: {}", token);
            }
            if token.len() == 1 && token.starts_with('+') {
                query_group.push(token_group);
                token_group = Vec::new();
                previous_token = /*String::new();*/ match previous_token_head {
                    Some(token) => token.clone(),
                    None => String::new(),
                };
                continue;
            }
            else if token.starts_with('(') && token.ends_with(')') && previous_token.len() != 0 {
                let mut inner_query : String = token.as_str().chars().skip(1).collect(); 
                println!("Multiply using {}", previous_token);
                inner_query = inner_query.chars().take(inner_query.len() - 1).collect();
                println!("Inner Query: {}", inner_query);
                // token is already (term1 + term2) use regex to remove ()+?
                let inner_query = self.tokenize_query(inner_query.as_str());
                let inner_group = self.group_tokens(&inner_query, Some(&previous_token));
                for inner_token_group in inner_group {
                    for mut inner_token in inner_token_group {
                        println!("Inner Token: {:?}", inner_token);
                        if inner_token.len() == 1 && inner_token.starts_with("+") {
                            continue;
                            //break;
                        }
                        inner_token = format!("{} {}", previous_token, inner_token);
                        token_group.push(inner_token);
                    }
                }
            }
            token_group.push(token.clone());
            //previous_token.push(' ');
            //previous_token.push_str(token);
            //previous_token = String::new();
        }
        query_group.push(token_group);
        println!("Query Group: {:?}", query_group);
        let mut new_query_group = Vec::new();
        for groups in query_group {
            let mut join_vec = Vec::new();
            let join_tokens = groups.join(" ");
            println!("JOIN TOKENS {}", join_tokens);
            join_vec.push(join_tokens);
            new_query_group.push(join_vec);
        }
        new_query_group
    }*/

    /*pub fn group_tokens(&self, input: &Vec<String>) -> Vec<Vec<String>> {
        let mut query_group = Vec::new();
        let mut query = Vec::new();
        for token in input {
            if token.len() == 1 && token.contains("+") { // When we hit a + sign
                let builtQuery = query.join(" ");
                query_group.push(builtQuery); // Push current built query into the vector
                query = Vec::new(); // And clear the query
                continue; // Continue to next token...
            }
            query.push(token.clone());
        }
        query_group
    }*/

    /*pub fn tokenize_query(&self, input: &str, recursive_steps: u32) -> Vec<String> {
        let mut tokens = Vec::new();
        println!("ABC INPUT {}", input);
        let mut input_iter = input.split_whitespace();
        let mut multiply_token = Vec::new();
        while let Some(token) = input_iter.next() {
            if token.len() == 1 && token.starts_with("+") {
                tokens.push(String::from(token));
                multiply_token = Vec::new();
                continue;
            }
            if token.starts_with("\"") {
                let mut phrase_literal = String::new();
                phrase_literal.push_str(token);
                phrase_literal.push_str(" ");
                while let Some(next_token) = input_iter.next() {
                    if next_token.ends_with("\"") {
                        phrase_literal.push_str(next_token);
                        break;
                    }
                    phrase_literal.push_str(next_token);
                    phrase_literal.push_str(" ");
                }
                tokens.push(phrase_literal.clone());
                multiply_token.push(phrase_literal.clone());
            }
            else if token.starts_with("(") {
                let mut inner_query_literal = String::new();
                inner_query_literal.push_str(token);
                inner_query_literal.push_str(" ");
                while let Some(next_token) = input_iter.next() {
                    if next_token.ends_with(")") {
                        inner_query_literal.push_str(next_token);
                        break;
                    }
                    inner_query_literal.push_str(next_token);
                    inner_query_literal.push_str(" ");
                }
                //tokens.push(inner_query_literal.clone());
                let inner_query_literal = inner_query_literal.trim_left_matches("(").trim_right_matches(")");
                for i in 0..recursive_steps {
                    //tokens.pop();
                }
                let inner_tokens = self.tokenize_query(inner_query_literal, recursive_steps + 1);
                let mut inner_word_group = Vec::new();
                let mut inner_group = Vec::new();
                for inner_token in inner_tokens {
                    println!("These are the inner tokens: {}", inner_token);
                    if inner_token.len() == 1 && inner_token.starts_with("+") {
                        let combined_inner_word_group_tokens = inner_word_group.join(" ");
                        println!("These are the combined tokens: {}", combined_inner_word_group_tokens);
                        inner_group.push(combined_inner_word_group_tokens);
                        inner_word_group = Vec::new();
                        continue;
                    }
                    inner_word_group.push(inner_token.clone());
                }
                let combined_inner_word_group_tokens = inner_word_group.join(" ");
                println!("These are the combined tokens: {}", combined_inner_word_group_tokens);
                inner_group.push(combined_inner_word_group_tokens);

                println!("INNER GROUP {:?}", inner_group);
                for i in 0..multiply_token.len() {
                    tokens.pop();
                }
                for inner_group_token in inner_group {
                    println!("Multiplying using {}", multiply_token.join(" "));
                    println!("MULTIPLYING {}", inner_group_token);
                    tokens.push(String::from("+"));
                    let multiplied_token = multiply_token.join(" ") + " " + &inner_group_token;
                    tokens.push(multiplied_token);
                }
            }
            else {
                tokens.push(String::from(token));
                multiply_token.push(String::from(token));
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
    }*/
}
