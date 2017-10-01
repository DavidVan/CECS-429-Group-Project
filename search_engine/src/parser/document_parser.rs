extern crate serde;
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

pub fn build_index(directory: String, index : &mut PositionalInvertedIndex, k_gram_index: &mut KGramIndex) -> HashMap<u32, String> {
    let paths = fs::read_dir(directory).unwrap();
    let mut files = Vec::new();

    for path in paths {
        files.push(path.unwrap().path().display().to_string())
    }
    let mut document: Document;

    let mut id_number = HashMap::new();

    for (i,file) in files.iter().enumerate() {
        println!("{}", i);
         
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

pub fn normalize_token(term: String) -> Vec<String> {
    let mut start_index:i32 = 0;
    let mut end_index:i32 = (term.len() as i32) - 1;
    for c in term.chars() {
        if !c.is_digit(10) && !c.is_alphabetic() && term.len() == 1 {
           let empty = "".to_string(); 
           let mut empty_vector = Vec::new();
           empty_vector.push(empty);
           return empty_vector;
        }
        if !c.is_digit(10) && !c.is_alphabetic() {
            start_index += 1;
        } else {
            break;
        }
    }
    for c in term.chars().rev() {
        if !c.is_digit(10) && !c.is_alphabetic() {
            end_index -= 1;
        } else {
            break;
        }
    }
    if (start_index > end_index) {
        let empty = "";
        return vec!{empty.to_owned()};
    }
    let mut alphanumeric_string: String = term.chars()
        .skip(start_index as usize)
        .take((end_index as usize)- (start_index as usize) + 1)
        .collect();
    // println!("alphanumeric_string - {}", alphanumeric_string);
    let apostrophe = "'";
    let empty_string = "";
    let mut apostrophe_reduced = alphanumeric_string.replace(apostrophe, empty_string);
    let hyphen = "-";
    let mut strings_to_stem: Vec<String> = Vec::new();
    if apostrophe_reduced.contains(hyphen) {
        let mut hyphen_index = 0;
        for c in apostrophe_reduced.chars() {
            if c == '-'{
                break;
            }
            hyphen_index += 1;
        }
        strings_to_stem.push(
            apostrophe_reduced
                .chars()
                .skip(0)
                .take(hyphen_index - 1)
                .collect(),
        );
        strings_to_stem.push(
            apostrophe_reduced
                .chars()
                .skip(hyphen_index)
                .take(apostrophe_reduced.len() - 1 - hyphen_index)
                .collect(),
        );
        strings_to_stem.push(apostrophe_reduced.replace(hyphen, empty_string));
    } else {
        strings_to_stem.push(apostrophe_reduced);
    }
    for mut word in strings_to_stem.iter_mut() {
        *word = word.to_lowercase();
    }

    let mut stemmer = Stemmer::new("english").unwrap();
    for mut word in strings_to_stem.iter_mut() {
        *word = stemmer.stem(word);
    }


    return strings_to_stem;
}
