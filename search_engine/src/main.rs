extern crate search_engine;

use std::collections::HashMap;

use search_engine::porter_stemmer::{stemmer};
use search_engine::index::inverted_index::InvertedIndex;

use std::io::{stdin,stdout,Write};
fn main() {
    let mut inverted_index = InvertedIndex {
        mIndex : HashMap::new(),
    };

    loop {
        let mut term = String::new();
        print!("Please enter some text: ");
        let _=stdout().flush();
        stdin().read_line(&mut term).expect("Did not enter a correct string");
        if let Some('\n')=term.chars().next_back() {
            term.pop();
        }
        if let Some('\r')=term.chars().next_back() {
            term.pop();
        }

        let mut docID = String::new();
        print!("Enter doc ID: ");

        let _=stdout().flush();
        stdin().read_line(&mut docID).expect("Did not enter a correct string");
        if let Some('\n')=docID.chars().next_back() {
            docID.pop();
        }
        if let Some('\r')=docID.chars().next_back() {
            docID.pop();
        }
        let my_string = docID.to_string();

        let docID_int = my_string.parse::<i32>().unwrap();

        inverted_index.addTerm(&term, docID_int);

        println!("There are {} in index", inverted_index.getTermCount());

        let postings = inverted_index.getPostings(&term);

        print!("{} : ", term);

        for p in postings {
            print!("{} ", p); 
        }
        println!("\n");
    }



}
