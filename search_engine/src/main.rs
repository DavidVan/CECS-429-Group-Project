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
use std::collections::HashSet;
use std::env::current_exe;
use std::path::*;

fn main() {
    let mut index_path = search_engine_paths::initializePath();
    let mut initial = true;

    let mut current: String;
    let mut input: String;
    let mut change: bool;

    let mut index = PositionalInvertedIndex::new();
    let mut k_gram_index = KGramIndex::new();
    let mut id_file = HashMap::new();
    loop {
        print!("Enter a directory to access: ");
        input = user_input::read_input();
        println!("You typed: {}", input);
        change = search_engine_paths::addToPath(&mut index_path, input.as_str());
        if change {
            current = input.clone();
            break;
        }
    }

    loop {
        println!("{}", index_path.display());
        if change {
            id_file = build_index(&index_path, &mut index, &mut k_gram_index);
            change = false;
        }

        print!("Input a Query: ");
        input = user_input::read_input_line();

        // TODO: Process query

        if !input.starts_with(":") {
            process_query(&input, &index_path, &index, &id_file, &k_gram_index);
        }
        
        else {
            if input == ":q" {
                break;
            } else if input.starts_with(":o ") || input.starts_with(":open ") {
                open_file(&index_path, input.as_str());
            } else if input.starts_with(":stem ") {
                stem_term(input.as_str());
            } else if input.starts_with(":index ") {
                change = index_directory(&mut index_path, input.clone());
            } else if input == ":vocab" {
                println!("Vocabulary");
                print_vocab(&index);
            } else {
                println!("Invalid command");
            }
        }
    }
}

fn build_index(
    index_path: &PathBuf,
    index: &mut PositionalInvertedIndex,
    k_gram_index: &mut KGramIndex,
) -> HashMap<u32, String> {
    let directory = index_path.to_str().expect("Not a valid directory");
    document_parser::build_index(directory.to_string(), index, k_gram_index)
}


fn process_query(input: &str, index_path: &PathBuf, index: &PositionalInvertedIndex, id_file: &HashMap<u32, String>, k_gram_index: &KGramIndex) {
    let parser = QueryParser::new();
    let processed_query = QueryParser::process_query(&parser, input);

    let mut results : HashSet<String> = HashSet::new();
    let mut or_results = Vec::new();
    let mut and_entries_precursor_string_vec = Vec::new(); // Dirty hack to get around lifetimes...
    for query in processed_query {
        println!("Query For: {}", query);
        let mut and_entries = Vec::new();
        let and_entries_precursor : Vec<&str> = query.split_whitespace().collect();
        for item in and_entries_precursor {
            and_entries_precursor_string_vec.push(String::from(item));
        }
        let mut and_entries_precursor_iter = and_entries_precursor_string_vec.iter();
        let mut entry_builder : Vec<String> = Vec::new();
        while let Some(entry) = and_entries_precursor_iter.next() {
            if entry.starts_with("\"") {
                let mut modified_entry : String = entry.chars().skip(1).collect();
                entry_builder.push(modified_entry);
                while let Some(next_entry) = and_entries_precursor_iter.next() {
                    if next_entry.ends_with("\"") {
                        modified_entry = next_entry.chars().take(next_entry.len() - 1).collect();
                        entry_builder.push(modified_entry);
                        and_entries.push(entry_builder.join(" "));
                        entry_builder.clear();
                        break;
                    }
                }
                continue;
            }
            and_entries.push(String::from(entry.clone()));
        }
        let mut and_results = Vec::new();
        for entry in and_entries {
            println!("AND PART: {}", entry);
            let normalized_tokens = document_parser::normalize_token(entry.to_string());
            for normalized_token in normalized_tokens {
                println!("Normalized Token: {}",  normalized_token);
                if !index.contains_term(normalized_token.as_str()) {
                    break;
                }
                let postings = index.get_postings(normalized_token.as_str()); 
                let mut and_inner_results = HashSet::new();
                for posting in postings {
                    and_inner_results.insert(id_file.get(&posting.getDocID()).unwrap().to_string());
                }
                and_results.push(and_inner_results);
            }
        }
        // Let's handle the AND logic...
        let mut and_results_iter = and_results.iter();
        let first_and_result = match and_results_iter.next() {
            Some(result) => result,
            None => break
        };
        let mut intersection = HashSet::new();
        for item in first_and_result {
            intersection.insert(item.clone());
        }
        while let Some(and_result) = and_results_iter.next() {
            let mut intersection_result : HashSet<_> = and_result.intersection(&intersection).cloned().collect();
            intersection.clear();
            for item in intersection_result {
                intersection.insert(item);
            }
        }
        or_results.push(intersection);
    }
    // Let's handle the OR logic...
    let mut or_results_iter = or_results.iter();
    let first_or_result = match or_results_iter.next() {
        Some(result) => result,
        None => {
            println!("0 Documents");
            return
        }
    };
    let mut union = HashSet::new();
    for item in first_or_result {
        union.insert(item.clone());
    }
    while let Some(or_result) = or_results_iter.next() {
        let mut union_result : HashSet<_> = or_result.union(&union).cloned().collect();
        union.clear();
        for item in union_result {
            union.insert(item);
        }
    }
    for x in union {
        results.insert(x);
    }

    for result in results.clone() {
        println!("Result: {}", result);
    }
    if results.len() != 1 {
        println!("{} Documents", results.len());
    }
    else {
        println!("{} Document", results.len());
    }
}

//fn process_query(input: &str, index_path: &PathBuf, index: &PositionalInvertedIndex, id_file: &HashMap<u32, String>, k_gram_index: &KGramIndex) {
//    let parser = QueryParser::new();
//    let processed_query = QueryParser::process_query(&parser, input);
//
//    // let mut postings = Vec::new();
//    
//    for query in processed_query {
//        println!("{}", query); 
//        let and_entries = query.split_whitespace();
//        for (i, entry) in and_entries.enumerate() {
//            let results = document_parser::normalize_token(entry.to_string());
//            let result = results.get(0).expect("Invalid token");
//            let query = result.to_string();
//            if index.contains_term(&query) {
//                let postings_list = index.get_postings(query.as_str());
//                print!("{} : ", query);
//                for posting in postings_list {
//                    let doc_id = posting.getDocID();
//                    let file: &Path = id_file.get(&doc_id).expect("Not a valid thing").as_ref();
//                    let file_name = file.file_name()
//                        .expect("Invalid os string")
//                        .to_str()
//                        .expect("Invalid string");
//                    print!("{} ", file_name);
//                }
//                println!();
//            }
//            if i == 0 {
//                 
//            }
//
//            println!("{}", result);
//        }
//    }
//    let results = document_parser::normalize_token(input.to_string());
//    let result = results.get(0).expect("not a valid token");
//    let query = result.to_string();
//
//    if index.contains_term(query.as_str()) {
//        let postings_list = index.get_postings(query.as_str());
//        print!("{} : ", query);
//        for posting in postings_list {
//            let doc_id = posting.getDocID();
//            let file: &Path = id_file.get(&doc_id).expect("Not a valid thing").as_ref();
//            let file_name = file.file_name()
//                .expect("Invalid os string")
//                .to_str()
//                .expect("Invalid string");
//            print!("{} ", file_name);
//        }
//        println!();
//    }
//}

fn stem_term(input: &str) {
    let mut stem = input.split_whitespace();
    if stem.size_hint().0 > 2 {
        println!("Invalid token");
    } else {
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
    if filePath.exists() {
        let document = read_file::read_file(filePath.to_str().expect("Not a valid string"));
        println!("{}", document.getBody());
    } else {
        println!("{} does not exist", filePath.display());
    }
}

fn print_vocab(index: &PositionalInvertedIndex) {
    let dictionary = index.get_dictionary();

    for term in dictionary.iter() {
        println!("{}", term);
    }
    println!("Total terms: {}", dictionary.len());
}
