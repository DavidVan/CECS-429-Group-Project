extern crate search_engine;
extern crate stemmer;

use search_engine::parser::query_parser::QueryParser;
use std::io::{stdin, stdout, Write};
use std::env::current_exe;

fn main() {
    println!("Hello World");    

    let mut documentPath = current_exe().expect("Not a valid path");

    for i in 1..4 {
        documentPath.pop();
    }
    documentPath.push("assets");
    println!("{}", documentPath.display());
    
    let mut s=String::new();
    print!("Enter a directory to access: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    println!("You typed: {}",s);

    documentPath.push(s);
    println!("{}", documentPath.display());

    if (documentPath.exists()) {
        println!("{} exists! Yay!", documentPath.display()); 
    }
    else {
        println!("{} does not exist!", documentPath.display());
    }
}
