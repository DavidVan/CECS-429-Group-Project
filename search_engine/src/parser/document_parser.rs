use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::ops::Add;
use ::serde_json::Error;
use std::fs::{self, DirEntry};
use std::path::Path;
use ::stemmer::Stemmer;

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
    let mut document: Document;
    for file in files {
        let mut f = File::open(file).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        document =  ::serde_json::from_str(&contents).unwrap();
        let mut iter = document.body.split_whitespace();

        while let Some(mut token) = iter.next() {
            
        }
    }
}

fn normalize_token(term: String) -> Vec<String> {
    let mut start_index = 0;
    let mut end_index = term.len() - 1;
    for c in term.chars() {
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
    let mut alphanumeric_string: String = term.chars()
        .skip(start_index)
        .take(end_index - start_index)
        .collect();
    let apostrophe = "'";
    let empty_string = "";
    let mut apostrophe_reduced = alphanumeric_string.replace(apostrophe, empty_string);
    let hyphen = "-";
    let mut strings_to_stem: Vec<String> = Vec::new();
    if apostrophe_reduced.contains(hyphen) {
        let mut hyphen_index = 0;
        for c in apostrophe_reduced.chars() {
            if c == '\'' {
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
