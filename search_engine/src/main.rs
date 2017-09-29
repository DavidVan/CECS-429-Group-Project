extern crate search_engine;
extern crate stemmer;

use search_engine::parser::query_parser::QueryParser;
use search_engine::paths::search_engine_paths;
use search_engine::reader::user_input;
use std::env::current_exe;
use std::path::PathBuf;

fn main() {
    println!("Hello World");    

    let mut documentPath = search_engine_paths::initializePath();
    let mut done = false;
    let mut initial = true;

    let mut input: String; 

    while(!done) {
        while (initial) {
            print!("Enter a directory to access: ");
            input = user_input::read_input();
            println!("You typed: {}",input);
            let successAdd = search_engine_paths::addToPath(&mut documentPath, input.as_str());
            println!("{}", documentPath.display());
            if (successAdd) {
                initial = false;
            }
        } 
        // TODO: Build Index after directory input
        // TODO: Take user query/input
        // TODO: Process query
    } 

}
