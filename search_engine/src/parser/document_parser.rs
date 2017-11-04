use std::collections::HashMap;
use std::fs::{self};
use std::time::SystemTime;
use index::k_gram_index::KGramIndex;
use index::positional_inverted_index::PositionalInvertedIndex;
use reader::read_file;
use stemmer::Stemmer;

/*
 * Function used to build a positional inverted index and KGram index.
 *
 * # Arguments 
 *
 * *`directory` - directory to index
 * *`index` - a blank inverted index
 * *`k_gram_index` - a blank k-gram-index
 *
 * # Returns
 * 
 * A hashmap mapping document IDs to their actual file names
 */
pub fn build_index(
    directory: String,
    index: &mut PositionalInvertedIndex,
    k_gram_index: &mut KGramIndex,
) -> HashMap<u64, String> {
    let paths = fs::read_dir(directory).unwrap();
    let mut files = Vec::new();

    // Add all files in path to vector
    for path in paths {
        files.push(path.unwrap().path().display().to_string())
    }

    let mut id_number = HashMap::new();

    let now = SystemTime::now();
    println!("Indexing...");
    //iterate through all files in directory
    for (i, file) in files.iter().enumerate() {
        // println!("Indexing {} out of {}...", i, files.len());

        //read the file and split it into each word
        let document = read_file::read_file(file);
        let document_body = document.clone().get_body();
        let iter = document_body.split_whitespace();

        id_number.insert(i as u64, file.to_string());
        //normalize each token in the file and add it to the index with its document id and position
        for (j, word) in iter.enumerate() {
            // println!("File {} / {} - Indexing token {} out of {}...", i, files.len(), j, iter_length);
            if k_gram_index.is_enabled() {
                if !index.contains_term(&word) {
                    k_gram_index.check_term(&word);
                }
            }
            let tokens = normalize_token(word.to_string());
            for term in tokens {
                index.add_term(&term, i as u64, j as u64);
            }
        }
    }

    println!("Indexing complete!\n");

    let time_elapsed = now.elapsed().expect("Invalid time");
    let time_elapsed_seconds = time_elapsed.as_secs();
    let time_elapsed_nano = time_elapsed.subsec_nanos();
    
    print!("Directory indexed in: ");

    if time_elapsed_seconds > 1 {
        println!("{} Seconds", time_elapsed_seconds);
    } else {
        println!("{} Nanoseconds", time_elapsed_nano);
    }
    println!();

    return id_number;
}

/*
 * Function to perform token normalization to obtain the stem of a word
 *
 * # Arguments
 * *`term` the term to normalize
 *
 * # Returns
 *
 * A vector containing the normalized token and any other forms of it
 * ex// if it contains a hyphen
 */
pub fn normalize_token(term: String) -> Vec<String> {
    let mut start_index: i32 = 0;
    let mut end_index: i32 = (term.len() as i32) - 1;
    //scan the term forwards and backwards to remove all leading and trailing non-alphanumeric characters
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
    //string was all non-alphanumeric characters
    if start_index > end_index {
        let empty = "";
        return vec![empty.to_owned()];
    }
    let alphanumeric_string: String = term.chars()
        .skip(start_index as usize)
        .take((end_index as usize) - (start_index as usize) + 1)
        .collect();
    // println!("alphanumeric_string - {}", alphanumeric_string);
    let apostrophe = "'";
    let empty_string = "";
    let apostrophe_reduced = alphanumeric_string.replace(apostrophe, empty_string);
    let hyphen = "-";
    let mut strings_to_stem: Vec<String> = Vec::new();
    //check if string contains a hyphen and remove the hyphen and normalize the two separated words
    if apostrophe_reduced.contains(hyphen) {
        let sub_words: Vec<&str> = apostrophe_reduced.split(hyphen).collect();
        for i in sub_words {
            strings_to_stem.push(i.to_string());
        }
        strings_to_stem.push(apostrophe_reduced.replace(hyphen, empty_string));
    } else {
        strings_to_stem.push(apostrophe_reduced);
    }
    //lowercase the remaining word(s)
    for mut word in strings_to_stem.iter_mut() {
        *word = word.to_lowercase();
    }

    //stem the remaining word(s)
    let mut stemmer = Stemmer::new("english").unwrap();
    for mut word in strings_to_stem.iter_mut() {
        *word = stemmer.stem(word);
    }

    return strings_to_stem;
}
