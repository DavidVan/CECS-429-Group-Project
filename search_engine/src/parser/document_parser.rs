extern crate serde;
extern crate humanesort;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::ops::Add;
use ::serde_json::Error;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::ops::Add;
use ::serde_json::Error;
use std::fs::{self, DirEntry};
use std::path::Path;
use ::stemmer::Stemmer;
use index::positional_inverted_index::PositionalInvertedIndex;
use index::k_gram_index::KGramIndex;
use reader::read_file;
use reader::read_file::Document;
use self::humanesort::prelude::*;

pub fn build_index(
    directory: String,
    index: &mut PositionalInvertedIndex,
    k_gram_index: &mut KGramIndex,
) -> HashMap<u32, String> {
    let paths = fs::read_dir(directory).unwrap();
    let mut files = Vec::new();

    for path in paths {
        files.push(path.unwrap().path().display().to_string())
    }
    files.humane_sort();
    let mut document: Document;

    let mut id_number = HashMap::new();

    for (i,file) in files.iter().enumerate() {
        //println!("{}", i);
         
        let document = read_file::read_file(file);
        let document_body = document.clone().getBody();
        let mut iter = document_body.split_whitespace();

        id_number.insert(i as u32, file.to_string());

        for (j,iter) in iter.enumerate() {
            let mut tokens = normalize_token(iter.to_owned());
            for term in tokens {
                index.addTerm(&term,i as u32,j as u32);
                // k_gram_index.checkIndex(&term);
            }
        }
    }
    return id_number;
}
