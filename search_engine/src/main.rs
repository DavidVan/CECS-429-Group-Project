extern crate search_engine;
extern crate stemmer;

use search_engine::parser::query_parser::QueryParser;
use search_engine::reader::user_input;
use std::env::current_exe;
use std::path::PathBuf;

fn main() {
    println!("Hello World");    

    let mut documentPath = setPath();
    
    print!("Enter a directory to access: ");
    let input = user_input::read_input();
    println!("You typed: {}",input);
    addToPath(&mut documentPath, input.as_str());
    println!("{}", documentPath.display());

}

fn setPath() -> PathBuf {
    let mut documentPath = current_exe().expect("Not a valid path");

    while (!documentPath.ends_with("CECS-429-Group-Project")) {
        documentPath.pop();
    }
    documentPath.push("search_engine");
    documentPath.push("assets");
    println!("{}", documentPath.display());
    return documentPath;
}

fn addToPath(pathbuf:&mut PathBuf, add: &str) {
    let mut testPath = pathbuf.clone();
    testPath.push(add);

    if (testPath.exists()) {
        println!("{} exists! Yay!", testPath.display()); 
        pathbuf.push(add);
    }
    else {
        println!("{} does not exist!", testPath.display());
    }
}
