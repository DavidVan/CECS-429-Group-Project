extern crate search_engine;
extern crate stemmer;

use search_engine::parser::query_parser::QueryParser;
use search_engine::parser::document_parser;
use search_engine::paths::search_engine_paths;
use search_engine::reader::read_file;
use search_engine::reader::user_input;
use search_engine::index::positional_inverted_index::PositionalInvertedIndex;
use search_engine::index::k_gram_index::KGramIndex;
use std::collections::HashMap;
use std::env::current_exe;
use std::path::*;

fn main() {
    let mut index_path = search_engine_paths::initializePath();
    let mut initial = true;

    let mut current : String;
    let mut input: String; 
    let mut change : bool;

    let mut index = PositionalInvertedIndex::new();
    let mut k_gram_index = KGramIndex::new();
    let mut id_number = HashMap::new();
    loop {
        print!("Enter a directory to access: ");
        input = user_input::read_input();
        println!("You typed: {}",input);
        change = search_engine_paths::addToPath(&mut index_path, input.as_str());
        if change {
            current = input.clone();
            break; 
        }
    }

    loop {
        println!("{}", index_path.display());
        // TODO: Build Index after directory input
        if change {
            id_number = build_index(&index_path, &mut index, &mut k_gram_index);
            change = false;
        }

        print!("Input a Query: ");
        input = user_input::read_input_line();

        // TODO: Process query
        
        process_query(&input, &index_path, &index, &id_number, &k_gram_index);

        if input.starts_with(":"){
            if input == ":q" {
                break; 
            }
            else if input.starts_with(":o ") || input.starts_with(":open ") {
                open_file(&index_path, input.as_str());
            }
            else if input.starts_with(":stem ") {
                stem_term(input.as_str());
            }
            else if input.starts_with(":index ") {
                change = index_directory(&mut index_path, input.clone());
            }
            else if input == ":vocab" {
                println!("Vocabulary");
                print_vocab(&index); 
            }
            else {
                println!("Invalid command"); 
            }
        }
    } 
}

fn build_index(index_path: &PathBuf, index : &mut PositionalInvertedIndex, k_gram_index: &mut KGramIndex) -> HashMap <u32, String> {
    let directory = index_path.to_str().expect("Not a valid directory");
    document_parser::build_index(directory.to_string(),index,k_gram_index)
}

fn process_query(input: &str, index_path: &PathBuf, index: &PositionalInvertedIndex, id_number: &HashMap<u32, String>, k_gram_index: &KGramIndex) {
   
    let results = document_parser::normalize_token(input.to_string());
    let result = results.get(0).expect("not a valid token");
    let query = result.to_string();

    if index.contains_term(query.as_str()) {
        let postings_list = index.get_postings(query.as_str());
        print!("{} : ", query);
        for posting in postings_list {
            let doc_id = posting.getDocID();
            let file : &Path = id_number.get(&doc_id)
                .expect("Not a valid thing")
                .as_ref();
            let file_name = file.file_name()
                .expect("Invalid os string")
                .to_str()
                .expect("Invalid string");
           print!("{} ", file_name);
        }
        println!();

    }
}

fn stem_term(input: &str) {
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

fn index_directory(mut index_path: &mut PathBuf, input: String) -> bool {
    let input_clone = input.clone();
    let mut string = input_clone.split_whitespace();
    let mut directory = string.nth(1).expect("Not a valid token");
    search_engine_paths::changeDirectory(&mut index_path, directory) 
}

fn open_file(index_path: &PathBuf, input: &str) {
    let mut string = input.split_whitespace();
    let file = string.nth(1).expect("Not a valid file");
    let mut filePath = index_path.clone();
    println!("Opening {}", file);
    filePath.push(file);
    if (filePath.exists()) {
        let document = read_file::read_file(filePath.to_str().expect("Not a valid string"));
        println!("{}", document.getBody());
    }
    else {
        println!("{} does not exist", filePath.display()); 
    }
}

fn print_vocab(index: &PositionalInvertedIndex) {
    let dictionary = index.get_dictionary();

    for term in dictionary.iter() {
        println!("{}", term); 
    }
}
