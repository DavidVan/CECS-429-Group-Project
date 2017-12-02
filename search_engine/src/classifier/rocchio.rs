extern crate serde;
extern crate serde_json;

use index::disk_inverted_index::DiskInvertedIndex;
use index::disk_inverted_index::IndexReader;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn rocchio_calculation_for_class(index: &DiskInvertedIndex) -> f64 {

    let doc_ids = retrieve_doc_ids(index);    
    let number_of_documents_in_class = doc_ids.len() as f64;
    let mut sum_of_docs = 0f64;
    for doc in doc_ids {
        sum_of_docs += calculate_normalized_vector_for_document(doc,index);
    }
    
    return sum_of_docs/number_of_documents_in_class;
}

fn calculate_normalized_vector_for_document(doc_id: u32, index:&DiskInvertedIndex) -> f64 {
    
    let document_weight = index.get_document_weights(doc_id).unwrap().1;
    /*TODO
     * Calculate the document vector:
     * Retrieve vocabulary of the index, then create vector of that size
     * Pull wdt for each term in the document then divide by document_weight
     * wdt =  0 for terms not in the document
     */ 
    return 0f64;
}

fn retrieve_id_file(index: &DiskInvertedIndex) -> HashMap<u32,String> {
    let id_file_filename = format!("{}/{}", index.get_path(), "id_file.bin");
    let mut id_file_file = File::open(id_file_filename).unwrap();
    let mut id_file_contents = String::new();
    id_file_file.read_to_string(&mut id_file_contents).expect("Failed to read id file");
    let id_file : HashMap<u32, String> = serde_json::from_str(&id_file_contents).unwrap();
    return id_file;
}

fn retrieve_doc_ids(index: &DiskInvertedIndex) -> Vec<u32> {
    let id_file = retrieve_id_file(index);
    let mut ids: Vec<u32> = Vec::new();
    for key in id_file.keys() {
        ids.push(*key);
    }
    return ids;
}