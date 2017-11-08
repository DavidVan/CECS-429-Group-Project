extern crate search_engine;
extern crate stemmer;

use search_engine::index::index_writer;
use search_engine::parser::document_parser;
use search_engine::paths::search_engine_paths;
use search_engine::processor::query_processor;
use search_engine::reader::read_file;
use search_engine::reader::user_input;
use search_engine::index::positional_inverted_index::PositionalInvertedIndex;
use search_engine::index::k_gram_index::KGramIndex;
use std::collections::HashMap;
use std::path::*;

fn main() {
    let mut index_path = search_engine_paths::initialize_path();

    let mut input: String;
    let mut change: bool;

    let mut id_file : HashMap<u32, String> = HashMap::new();
    let mut index = PositionalInvertedIndex::new();
    let mut k_gram_index = KGramIndex::new();

    // Loops lets user select first directory to access
    loop {
        print!("Select directory to access: ");
        input = user_input::read_input();
        if input == ":q" {
            return (); // Prematurely ends program
        }
        if input.is_empty() {
            println!("Please input valid directory");
            continue;
        }
        change = search_engine_paths::change_directory(&mut index_path, input.as_str());
        
        // Exit loop if successful change to new directory
        if change {
            break;
        }
    }

    // Loop that drives program after initial setup
    loop {
        // Links document ID's to file names

        // Builds new index if directory was changed
        if change {
            id_file = build_index(&index_path, &mut index, &mut k_gram_index);
            change = false;
        }

        println!("{}", index_path.display());
        print!("Input a Query: ");
        input = user_input::read_input_line();

        if !input.starts_with(":") {
            process_query(&input, &index, &k_gram_index, &id_file);
        } else {
            if input == ":q" || input == ":quit" {
               return (); 
            } else if input.starts_with(":o ") || input.starts_with(":open ") {
                open_file(&index_path, input.as_str());
            } else if input.starts_with(":s ") || input.starts_with(":stem ") {
                stem_term(input.as_str());
            } else if input.starts_with(":i ") || input.starts_with(":index ") {
                change = index_directory(&mut index_path, input.clone());
            } else if input == ":v" || input == ":vocab" {
                print_vocab(&index);
            } else if input == ":enable k" || input == ":enable kgram" {
                if !k_gram_index.is_enabled() {
                   change = true;  
                }
                toggle_k_gram(&mut k_gram_index, true);
            } else if input == ":disable k" || input == ":disable kgram" {
                if k_gram_index.is_enabled() {
                   change = true;  
                }
                toggle_k_gram(&mut k_gram_index, false);
            } else if input == ":h" || input == ":help" {
                print_help(); 
            } else {
                println!("Invalid command - Use ':help' to view commands");
            }
        }
    }
}

/*
 * Builds the Positional Inverted Index given the directory containing the files it will read
 * 
 * # Arguments
 * 
 * *`index_path` - The Path Buffer of the full file path the program is observing
 * *`index` - The Postitional Inverted Index that will be built given the directory being observed
 * *`k_gram_index` - The K Gram Index that will be built give the directory being observed
 *
 * # Returns
 * 
 * HashMap that contains the association between each Document ID and the file name
 * 
 */
fn build_index(
    index_path: &PathBuf,
    index: &mut PositionalInvertedIndex,
    k_gram_index: &mut KGramIndex,) -> HashMap<u32, String> {

    let directory = index_path.to_str().expect("Not a valid directory");
    document_parser::build_index(directory.to_string(), index, k_gram_index)
}

/*
 * Processes an inputted query before printing the results of that query
 * 
 * # Arguments
 * 
 * *`input` - The inputted query that will be processed
 * *`index` - The Positional Inverted Index that will be used to process the term
 * *`id_file` - HashMap that contains the association between a Document ID and the file name
 */
fn process_query(
    input: &str,
    index: &PositionalInvertedIndex,
    k_gram_index: &KGramIndex,
    id_file: &HashMap<u32, String>) {

    println!();
    let results = query_processor::process_query(input, index, k_gram_index, id_file);
    println!();
    for result in results.clone() {
        println!("Result: {}", result);
    }
    println!();
    if results.len() != 1 {
        println!("{} Documents", results.len());
    } else {
        println!("{} Document", results.len());
    }
    println!();
}

/*
 * Prints out a term after normalizing and stemming
 *
 * # Arguments
 *
 * *`input` - The term that will be normalized and stemmed
 *
 */
fn stem_term(
    input: &str) {

    let mut stem = input.split_whitespace();
    if stem.size_hint().0 > 2 {
        println!("Invalid token");
    } else {
        let string = stem.nth(1).expect("Not a valid token");
        let results = document_parser::normalize_token(string.to_string());
        let result = results.get(0).expect("Not a valid token");
        println!("{}", result);
    }
}

/*
 * Changes the directory of the index_path to a new specified directory
 *
 * # Arguments
 * 
 * *`index_path` - The PathBuf that is set to the current working directory and will be changed to
 * the new directory
 * *`input` - The new directory that will be changed to
 */
fn index_directory(
    mut index_path: &mut PathBuf,
    input: String) -> bool {
    let input_clone = input.clone();
    let mut string = input_clone.split_whitespace();
    let directory = string.nth(1).expect("Not a valid token");
    search_engine_paths::change_directory(&mut index_path, directory)
}

/*
 * Opens a file and prints out its contents
 *
 * # Arguments
 *
 * *`index_path` - The PathBuffer set to the current working directory containing the file to be
 * read
 * *`input` - User input representing the file that will be read
 */
fn open_file(
    index_path: &PathBuf,
    input: &str) {

    let mut string = input.split_whitespace();
    let file = string.nth(1).expect("Not a valid file");
    let mut file_path = index_path.clone();
    println!("Opening {}", file);
    file_path.push(file);
    if file_path.exists() {
        let document = read_file::read_file(file_path.to_str().expect("Not a valid string"));
        println!("\n{}", document.get_title());
        println!("\n{}", document.get_body());
        println!("\n{}", document.get_url());
        println!();
    } else {
        println!("{} does not exist", file_path.display());
    }
}

/*
 * Prints out all vocabulary terms in the index
 *
 * # Arguments
 * 
 * *`index` - The Positional Inverted Index containing the terms
 */
fn print_vocab(
    index: &PositionalInvertedIndex) {
    
    println!("Vocabulary");

    let dictionary = index.get_dictionary();

    for term in dictionary.iter() {
        println!("{}", term);
    }
    println!("Total terms: {}", dictionary.len());
}

/*
 * Toggled the  K_gram index on/off
 *
 * # Arguments
 *
 * *`k_gram` - The KGramIndex that will be enabled/disabled
 * *`enable` - The toggle value of the KGramIndex
 */
fn toggle_k_gram(
    k_gram_index: &mut KGramIndex, enable: bool) {

    println!();
    if enable {
        println!("K Gram Index Enabled\n");
        k_gram_index.enable_k_gram();
    } else {
        println!("K Gram Index Disabled\n");
        k_gram_index.disable_k_gram();
    }   
    println!();

}

/*
 * Prints the list of commands
 */
fn print_help(){
    println!("\nHelp: \n");
    println!(":h || :help - Prints this dialog :) ");
    println!(":o FILE || :open FILE - Opens a file to read in the current directory");
    println!(":q || :quit - Quits the Program");
    println!(":index DIRECTORY - Changes directory to specified directory and build index under that directory");
    println!(":stem TERM - Normalizes and applies the stemmer on a specified term");
    println!(":enable kgram || :enable k - Enables K Gram Index when indexing");
    println!(":disable kgram || :disable k - Disables K Gram Index when indexing");
    println!();
}
