#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::ops::Add;
use serde_json::Error;
use std::fs::{self, DirEntry};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Corpus {
    documents: Vec<Document>,
}

#[derive(Serialize, Deserialize)]
struct Document {
    title: String,
    body: String,
    url: String,
}

fn build_index(directory: String) {
    let paths = fs::read_dir(directory).unwrap();
    let mut files = Vec::new();
    
    for path in paths {
        files.push(path.unwrap().path().display().to_string())
    }

    for file in files {
        let mut f = File::open(file).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("something went wrong reading the file");

        let mut iter = contents.split_whitespace();
        
        while let Some(mut token) = iter.next() {
            
        
        }
    }

}

fn normalize_token(term:String)-> String {
    let mut start_index = 0;
    let mut end_index = term.len()-1;
    for c in term.chars() {
        if !c.is_digit(10) || !c.is_alphabetic() {
          start_index += 1;
        } else {
          break;
        }
    }
    for c in term.chars().rev() {
      if !c.is_digit(10) || !c.is_alphabetic() {
          end_index -= 1;
        } else {
          break;
        }
    }
   let mut alphanumeric_string: String = term.chars().skip(start_index).take(end_index-start_index).collect();
   let apostrophe = "'";
   let empty_string = "";
   let alphanumeric_string = alphanumeric_string.replace(apostrophe,empty_string);

   
   return alphanumeric_string;
}