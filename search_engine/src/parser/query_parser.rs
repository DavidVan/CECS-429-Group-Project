use std::io::{self, Read};
use std::process::exit;
use std::option;

pub struct QueryParser {

}

impl QueryParser {
    pub fn new() -> QueryParser {
        QueryParser {}
    }

    pub fn process_query(&self, input: &str) -> Vec<String> {
        let mut results : Vec<String> = Vec::new();
        // Part 1 - Group the token into groups separated by "+"
        let mut tokens = input.split_whitespace();
        let mut query_builder : Vec<String> = Vec::new();

        while let Some(mut token) = tokens.next() {
            if token.len() == 1 && token.starts_with("+") {
                if query_builder.len() != 0 {
                    results.push(query_builder.join(" "));
                    query_builder.clear();
                }
                continue;
            }
            if token.starts_with("(") {
                // Replace this block with multiply...
                let mut parenthesis_query_vec = Vec::new();
                let previous_token = query_builder.join(" ");
                query_builder.clear();
                parenthesis_query_vec.push(previous_token);
                parenthesis_query_vec.push(String::from(token));
                token = tokens.next().unwrap();
                let mut left_parenthesis_counter = 1;
                while left_parenthesis_counter != 0 {
                    parenthesis_query_vec.push(String::from(token));
                    if token.starts_with("(") {
                        left_parenthesis_counter += 1;
                    }
                    if token.ends_with(")") {
                        let mut reverse_token_iter = token.chars().rev();
                        while let Some(mut c) = reverse_token_iter.next() {
                            if c == ')' {
                                left_parenthesis_counter -= 1;
                            }
                            else {
                                break;
                            }
                        }
                    }
                    match tokens.next() {
                        Some(part) => token = part,
                        None => break
                    }
                }
                results.push(parenthesis_query_vec.join(" "));
                continue;
            }
            query_builder.push(String::from(token));
        }
        if query_builder.len() != 0 {
            results.push(query_builder.join(" "));
        }

        // Part 2 - Expand inner queries if any...
        let mut final_results : Vec<String> = Vec::new();
        let mut results_iter = results.iter();
        while let Some(result) = results_iter.next() {
            if result.ends_with(")") { // I should find a better way of checking if it's nested...
                // Process the result...
                println!("Recursive call on: {}", result);
                let sub_results = self.multiply_query(result);
                // Add sub-results to final results...
                for sub_result in sub_results {
                    println!("Sub results: {}", sub_result);
                    final_results.push(sub_result);
                }
                continue;
            }
            final_results.push(result.clone());
        }
        final_results
    }

    pub fn multiply_query(&self, input: &str) -> Vec<String> {
        let mut results : Vec<String> = Vec::new();
        let mut tokens = input.split_whitespace();
        let mut multiplier_vec : Vec<String> = Vec::new();
        let mut multiplicand_vec : Vec<String> = Vec::new();

        while let Some(token) = tokens.next() {
            if token.starts_with("(") {
                multiplicand_vec.push(String::from(token));
                break;
            }
            multiplier_vec.push(String::from(token));
        }

        let mut query_builder : Vec<String> = Vec::new();
        while let Some(mut token) = tokens.next() {
            if token.len() == 1 && token.starts_with("+") {
                if query_builder.len() != 0 {
                    multiplicand_vec.push(query_builder.join(" "));
                    query_builder.clear();
                }
                continue;
            }
            if token.starts_with("(") {
                query_builder.push(String::from(token));
                token = tokens.next().unwrap();
                let mut left_parenthesis_counter = 1;
                while left_parenthesis_counter != 0 {
                    query_builder.push(String::from(token));
                    if token.starts_with("(") {
                        left_parenthesis_counter += 1;
                    }
                    if token.ends_with(")") {
                        let mut reverse_token_iter = token.chars().rev();
                        while let Some(mut c) = reverse_token_iter.next() {
                            if c == ')' {
                                left_parenthesis_counter -= 1;
                            }
                            else {
                                break;
                            }
                        }
                    }
                    match tokens.next() {
                        Some(part) => token = part,
                        None => break
                    }
                }
                multiplicand_vec.push(query_builder.join(" "));
                query_builder.clear();
                continue;
            }
            multiplicand_vec.push(String::from(token)); // Finish the job...
        }
        let multiplier = multiplier_vec.join(" ");
        multiplicand_vec[0] = multiplicand_vec[0].chars().skip(1).collect();
        let multiplicand_vec_length = multiplicand_vec.len();
        let multiplicand_last_element_length = multiplicand_vec[multiplicand_vec_length - 1].len();
        multiplicand_vec[multiplicand_vec_length - 1] = multiplicand_vec[multiplicand_vec_length - 1].chars().take(multiplicand_last_element_length - 1).collect();
        let multiplicand_precursor = multiplicand_vec.join(" ");
        let multiplicand : String = multiplicand_precursor.chars().take(multiplicand_precursor.len() - 2).skip(1).collect();
        for multiplicand in multiplicand_vec {
            results.push(multiplier.clone() + " " + multiplicand.as_str());
        }
        let mut final_results : Vec<String> = Vec::new();
        let mut results_iter = results.iter();
        while let Some(result) = results_iter.next() {
            if result.ends_with(")") { // I should find a better way of checking if it's nested...
                // Process the result...
                let sub_results = self.multiply_query(result);
                // Add sub-results to final results...
                for sub_result in sub_results {
                    final_results.push(sub_result);
                }
                continue;
            }
            final_results.push(result.clone());
        }
        final_results
    }
    /*pub fn multiply_query(&self, input: &str) -> Vec<String> {
        let mut results : Vec<String> = Vec::new();
        let mut tokens = input.split_whitespace();
        for token in tokens.clone() {
            println!("multiply token: {}", token);
        }
        let mut multiplier_vec : Vec<String> = Vec::new();
        let mut query_builder : Vec<String> = Vec::new();
        while let Some(mut token) = tokens.next() {
            if !token.starts_with("(") {
                multiplier_vec.push(String::from(token));
                continue;
            }
            else {
                println!("Multiplier {}", multiplier_vec.join(" "));
                let multiplier = multiplier_vec.join(" ");
                query_builder.push(multiplier.clone());
                query_builder.push(token.chars().skip(1).collect());
                loop {
                    match tokens.next() {
                        Some(part) => {
                            if part.len() == 1 && part.starts_with("+") {
                                results.push(query_builder.join(" "));
                                query_builder.clear();
                                query_builder.push(multiplier.clone());
                                continue;
                            }
                            if part.starts_with("(") {
                                println!("My paren token: {}", part);
                                query_builder.push(String::from(part));
                                let mut left_parenthesis_counter = 1;
                                while let Some(next_token) = tokens.next() {
                                    println!("Next Token Is: {}", next_token);
                                    /*if next_token.len() == 1 && next_token.starts_with("+") {
                                        let next_next_token = tokens.next().unwrap();
                                        println!("Handling plus {}:",query_builder.join(" "));
                                        query_builder.clear();
                                        if next_next_token.starts_with("(") {
                                            
                                        }
                                        continue;
                                    }*/
                                    if left_parenthesis_counter != 0 {
                                        println!("Pushing next token: {}", next_token);
                                        query_builder.push(String::from(next_token));
                                        println!("Query Builder after pushing: {:?}", query_builder);
                                        if next_token.starts_with("(") {
                                            left_parenthesis_counter += 1;
                                        }
                                        if next_token.ends_with(")") {
                                            let mut reverse_token_iter = next_token.chars().rev();
                                            while let Some(mut c) = reverse_token_iter.next() {
                                                if c == ')' {
                                                    left_parenthesis_counter -= 1;
                                                }
                                                else {
                                                    break;
                                                }
                                            }
                                            continue;
                                        }
                                        query_builder.push(String::from(next_token));
                                    }
                                    println!("MY BEAUTIFUL QUERY {}", query_builder.join(" "));
                                    results.push(query_builder.join(" "));
                                    continue;
                                }
                                continue;
                            }
                            query_builder.push(String::from(part));
                        },
                        None => break
                    }
                }
                continue;
                println!("build query paren: {}", query_builder.join(" "));
            }
            query_builder.push(String::from(token));
        }
        println!("Multiply Success? {:?}", results);
        results
    }*/

    /*pub fn parenthesis_query_to_vec(&self, query: String) -> Vec<String> {
      if !query.starts_with("(") && !query.ends_with(")") {
      panic!("Query is not a parenthesis query!");
      }
      let mut query_with_no_parenthesis_vec = Vec::new();
      let mut new_query : String = query.chars().take(query.len() - 1).skip(1).collect();
      let mut new_query_iter = new_query.split_whitespace();
      while let Some(mut query_part) = new_query_iter.next() {
      if query_part.len() == 1 && query_part.starts_with("+") {
      continue;
      }
      if query_part.starts_with("(") {
      let previous_query = query_with_no_parenthesis_vec.pop().unwrap();
      let mut new_inner_query_vec = Vec::new();
      new_inner_query_vec.push(String::from(previous_query) + " " + query_part);
      let mut left_parenthesis_counter = 1;
      let mut right_parenthesis_counter = 0;
      query_part = new_query_iter.next().unwrap(); // skip to next..
      while left_parenthesis_counter > right_parenthesis_counter {
      new_inner_query_vec.push(String::from(query_part)); 
      if query_part.starts_with("(") {
      left_parenthesis_counter += 1
      }
      else if query_part.ends_with(")") {
      right_parenthesis_counter += 1;
      left_parenthesis_counter -= 1;
      }
      match new_query_iter.next() {
      Some(part) => query_part = part,
      None => break
      }
      }
      query_with_no_parenthesis_vec.push(new_inner_query_vec.join(" "));
      continue;
      }
      query_with_no_parenthesis_vec.push(String::from(query_part));
      }
      query_with_no_parenthesis_vec
      }

      pub fn multiply_token(&self, multiplier: String, multiplicand: &Vec<String>) -> Vec<String> {
      let mut results = Vec::new();
      let mut previous_multiplied = Vec::new();
      for item in multiplicand {
      if item.starts_with("(") {
      println!("starts with paren {}", item);
      println!("previous multiplied: {}", previous_multiplied.join(" "));
      let mut query_builder = Vec::new();
      let previous_multiplied_string = previous_multiplied.join(" ");
      let multiplicand_without_parenthesis : String = item.clone().chars().skip(1).take(item.len() - 2).collect();
      let mut multiplicand_without_parenthesis_iter = multiplicand_without_parenthesis.split_whitespace();
      for i in multiplicand_without_parenthesis.split_whitespace() {
      println!("multiplicant without paren {}", i);
      }
      while let Some(query_part) = multiplicand_without_parenthesis_iter.next() {
      println!("Query part: {}", query_part);
      if query_part.len() == 1 && query_part.starts_with("+") {
      continue;
      }
      if query_part.starts_with("(") {
      let mut inner_query_builder = Vec::new();
      inner_query_builder.push(query_part);
      println!("query pushed LOL {}", query_part);
      let mut left_parenthesis_counter = 1;
      let mut right_parenthesis_counter = 0;
      let mut next_query_part = multiplicand_without_parenthesis_iter.next().unwrap();
      while left_parenthesis_counter > right_parenthesis_counter {
      println!("Current State of inner query: {}", inner_query_builder.join(" "));
      inner_query_builder.push(next_query_part);
      if next_query_part.starts_with("(") {
      left_parenthesis_counter += 1
}
else if next_query_part.ends_with(")") {
    right_parenthesis_counter += 1;
    left_parenthesis_counter -= 1;
}
match multiplicand_without_parenthesis_iter.next() {
    Some(part) => next_query_part = part,
    None => break
}
}
let combined_term = query_builder.pop().unwrap() + " " + inner_query_builder.join(" ").as_str();
query_builder.push(combined_term);
println!("built query going to multiply again {:?}", query_builder);
let new_query = self.multiply_token(previous_multiplied.join(" "), &query_builder);
println!("FINAL MULTI RESULT: {:?}", new_query);
query_builder.clear();
for query in new_query {
    if query.starts_with("(") {
        let multipler_token_test = query_builder.pop().unwrap();
        query_builder.push(self.multiply_token(multipler_token_test, &self.parenthesis_query_to_vec(query.clone())).join(" "));
    }
    query_builder.push(query);
}
continue;
}
query_builder.push(String::from(query_part));
}
println!("paren removed: {}", multiplicand_without_parenthesis);
println!("built query: {:?}", query_builder);
}
println!("multiplying now! {}", item);
previous_multiplied.clear();
previous_multiplied.push(multiplier.clone());
previous_multiplied.push(item.clone());
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
                query_builder.push(next_sub_query); // should I clone? I removed it just now.
                if next_sub_query.starts_with("(") {
                    left_parenthesis_counter += 1;
                }
                else if next_sub_query.ends_with(")") {
                    right_parenthesis_counter += 1;
                    left_parenthesis_counter -= 1;
                }
                //next_sub_query = query_iter.next().unwrap();
                match query_iter.next() {
                    Some(part) => next_sub_query = part,
                    None => break
                }
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
}*/
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
