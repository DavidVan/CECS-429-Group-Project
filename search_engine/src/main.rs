extern crate search_engine;
extern crate stemmer;

use search_engine::parser::query_parser::QueryParser;
use search_engine::parser::document_parser;
use search_engine::paths::search_engine_paths;
use search_engine::reader::user_input;
use std::env::current_exe;
use std::path::PathBuf;

fn main() {
    println!("Hello World");    

    let mut documentPath = search_engine_paths::initializePath();
    let mut initial = true;

    let mut current : String;
    let mut input: String; 
    loop {
        print!("Enter a directory to access: ");
        input = user_input::read_input();
        println!("You typed: {}",input);
        let successAdd = search_engine_paths::addToPath(&mut documentPath, input.as_str());
        if (successAdd) {
            current = input.clone();
            break; 
        }
    }

    loop {
        println!("{}", documentPath.display());
        // TODO: Build Index after directory input

        print!("Input a Query: ");
        input = user_input::read_input_line();

        // TODO: Process query
        //
        if input.starts_with(":"){
            if input == ":q" {
                break; 
            }
            else if input.starts_with(":stem ") {
                let mut stem = input.split_whitespace();
                if stem.size_hint().0 > 2 {
                    println!("Invalid token"); 
                }
                else {
                    let mut string = stem.nth(1).expect("Not a valid token");
                    let results = document_parser::normalize_token(string.to_string());
                    let result = results.get(0).expect("Not a valid token");
                    let result_string = result.to_string();
                    println!("{}", result);
                }
            }
            else if input.starts_with(":index ") {
                let mut string = input.split_whitespace();
                let directory = string.nth(1).expect("Not a valid token");
                let valid = search_engine_paths::changeDirectory(&mut documentPath, directory); 
                if !valid {
                    search_engine_paths::addToPath(&mut documentPath, current.as_str()); 
                }
                else {
                    current = directory.clone().to_string();
                }

            }
            else if input == ":vocab" {
                // TODO: Build index before this can be used 
            }
        }
    } 

}
