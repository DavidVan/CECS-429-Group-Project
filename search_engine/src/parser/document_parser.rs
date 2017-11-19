use std::collections::HashMap;
use std::fs::{self};
use std::time::SystemTime;
use index::index_writer::IndexWriter;
use index::index_writer::DiskIndex;
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

pub struct DocumentWeight {
    doc_id: u32,
    doc_weight: f64,
    doc_length: u32,
    byte_size: u32,
    avg_tftd: f64,
}

impl DocumentWeight {
    fn new(doc_id: u32, doc_weight: f64, doc_length: u32, byte_size: u32, avg_tftd: f64) -> Self {
        DocumentWeight {
            doc_id: doc_id,
            doc_weight: doc_weight,
            doc_length: doc_length,
            byte_size: byte_size,
            avg_tftd: avg_tftd,
        }
    } 

    pub fn get_doc_id(&self) -> u32 {
        self.doc_id
    }

    pub fn get_doc_weight(&self) -> f64 {
        self.doc_weight
    }

    pub fn get_doc_length(&self) -> u32 {
        self.doc_length
    }

    pub fn get_byte_size(&self) -> u32 {
        self.byte_size
    }

    pub fn get_avg_tftd(&self) -> f64 {
        self.avg_tftd
    }
}

pub fn build_index(
    directory: String,
    index: &mut PositionalInvertedIndex,
    k_gram_index: &mut KGramIndex,
    ) -> HashMap<u32, String> {
    let paths = fs::read_dir(directory.clone()).unwrap();
    let mut files = Vec::new();

    // Add all files in path to vector
    for path in paths {
        files.push(path.unwrap().path().display().to_string())
    }

    let mut id_number = HashMap::new();

    let now = SystemTime::now();
    println!("Indexing...");
    let mut avg_doc_weight_accumulator = 0;
    let mut doc_weights : Vec<DocumentWeight> = Vec::new();
    //iterate through all files in directory
    for (i, file) in files.iter().enumerate() {
        // println!("Indexing {} out of {}...", i, files.len());

        //read the file and split it into each word
        let document = read_file::read_file(file);
        let document_body = document.clone().get_body();
        let iter = document_body.split_whitespace();

        id_number.insert(i as u32, file.to_string());

        let mut tftd: HashMap<String,u32> = HashMap::new(); 

        //normalize each token in the file and add it to the index with its document id and position
        for (j, word) in iter.enumerate() {


            // println!("File {} / {} - Indexing token {} out of {}...", i, files.len(), j, iter_length);
            let tokens = normalize_token(word.to_string());
            // if k_gram_index.is_enabled() {
                // k_gram_index.check_terms(&tokens);
            // }
            for term in tokens {
                if !tftd.contains_key(&term) {
                    tftd.insert(term.to_string(),1);
                } else {
                    *tftd.get_mut(&term).unwrap() = tftd.get(&term).unwrap() + 1;
                }
                index.add_term(&term, i as u32, j as u32);
            }
        }

        let mut wdt: HashMap<String,f64> = HashMap::new();
        let mut wdt_tf_idf: HashMap<String,f64> = HashMap::new();
        let mut wdt_okapi: HashMap<String,f64> = HashMap::new();
        let mut wdt_wacky: HashMap<String,f64> = HashMap::new();
        for (term,value) in &tftd {
            let weight:f64 = 1.0f64 + (*value as f64).ln();
            let tf_idf_weight: f64 = ((tftd.len() as f64/(*value)as f64)).ln();
            let okapi_weight: f64 =2.2f64 * (*value as f64);
            let wacky_weight = weight/(1.0f64 + ( ( (tftd.values().sum::<u32>() as f64) / (tftd.len() as f64) ).ln() ) );
            wdt.insert(term.to_string(),weight);
            wdt_tf_idf.insert(term.to_string(),tf_idf_weight);
            wdt_okapi.insert(term.to_string(),okapi_weight);
            wdt_wacky.insert(term.to_string(),wacky_weight);
            index.set_score(term,weight);
            index.set_tf_idf_score(term,tf_idf_weight);
            index.set_okapi_score(term,okapi_weight);
            index.set_wacky_score(term,wacky_weight);
        }

        avg_doc_weight_accumulator += tftd.len();


        let mut sum_weights_squared: f64 = 0.0f64;
        for val in wdt.values() {
            sum_weights_squared = sum_weights_squared + val.powi(2);
        }

        let euclidian_doc_weights = sum_weights_squared.sqrt();
        let doc_length = tftd.len() as u32;
        let byte_size = fs::metadata(file).unwrap().len() as u32;
        let avg_tftd = (tftd.values().sum::<u32>() as f64) / (tftd.len() as f64);

        doc_weights.push(DocumentWeight::new(i as u32, euclidian_doc_weights, doc_length, byte_size, avg_tftd));
    }

    let avg_doc_length = avg_doc_weight_accumulator as f64 / doc_weights.len() as f64;

    // Build DiskInvertedIndex
    let index_writer = IndexWriter::new(directory.as_str());
    index_writer.build_index_for_directory(index, &doc_weights, avg_doc_length, directory.as_str());


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
    // println!("Original - {}", term);
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
    let mut alphanumeric_string: String = term.chars()
        .skip(start_index as usize)
        .take((end_index as usize) - (start_index as usize) + 1)
        .collect();
    // println!("Alphanumeric - {}", alphanumeric_string);
    let apostrophe = "'";
    let empty_string = "";

    // Replace UTF 8 Apostrophes
    alphanumeric_string = alphanumeric_string.replace("\\u{{2018}}", apostrophe);
    alphanumeric_string = alphanumeric_string.replace("\\u{{2019}}", apostrophe);

    let mut reduced_string = alphanumeric_string.replace(apostrophe, empty_string);
    reduced_string = reduced_string.replace("(", "-");
    reduced_string = reduced_string.replace(")", "-");
    // println!("Reduced - {}", reduced_string);
    // println!("is ASCII: {}", reduced_string.is_ascii());
    let hyphen = "-";
    let mut normalized_strings: Vec<String> = Vec::new();
    //check if string contains a hyphen and remove the hyphen and normalize the two separated words
    if reduced_string.contains(hyphen) {
        let sub_words: Vec<&str> = reduced_string.split(hyphen).collect();
        for i in sub_words {
            normalized_strings.push(i.to_string());
        }
        normalized_strings.push(reduced_string.replace(hyphen, empty_string));
    } else {
        normalized_strings.push(reduced_string);
    }
    //lowercase the remaining word(s)
    for word in normalized_strings.iter_mut() {
        *word = word.to_lowercase();
    }

    return normalized_strings;
}

pub fn stem_terms(mut strings_to_stem: Vec <String> ) -> Vec <String>{
    //stem the remaining word(s)
    let mut stemmer = Stemmer::new("english").unwrap();
    for word in strings_to_stem.iter_mut() {
        *word = stemmer.stem(word);
    }

    return strings_to_stem;
}
