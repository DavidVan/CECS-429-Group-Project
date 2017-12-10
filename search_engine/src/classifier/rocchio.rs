extern crate serde;
extern crate serde_json;

use index::disk_inverted_index::DiskInvertedIndex;
use index::disk_inverted_index::IndexReader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use classifier::classifier::Classifier;

pub struct Rocchio<'a> {
    index: &'a DiskInvertedIndex<'a>,
}

impl<'a> Rocchio<'a> {
    fn new(index: &'a DiskInvertedIndex) -> Rocchio<'a> {
        Rocchio { index: index }
    }

    fn calculate_centroid(&self) -> Vec<f64> {
        let doc_ids = self.retrieve_doc_ids();
        let number_of_documents_in_class = doc_ids.len() as f64;
        let mut sum_of_docs = Vec::new();
        for doc in doc_ids {
            sum_of_docs = add_vector_components(self.calculate_normalized_vector_for_document(doc),sum_of_docs);
        }
        return sum_of_docs.iter().map(|&x| x/number_of_documents_in_class).collect::<Vec<_>>();
    }

    fn calculate_normalized_vector_for_document(&self,doc_id: u32) -> Vec<f64> {
        let document_weight = self.index.get_document_weights(doc_id).unwrap().1;
        /*
        TODO: change get_vocab function to retreive global vocab, 
              currently only retrieves vocab of the index
        */
        let vocab = self.index.get_vocab();
        let mut document_vector: Vec<f64> = Vec::new();
        for term in vocab {
            let res = self.index.get_postings_no_positions(&term);
            let term_exists = res.is_ok();
            if !term_exists {
                document_vector.push(0f64);
                continue;
            }
            let posting = res.unwrap();
            for (id, _, term_score, _, _, _) in posting {
                if id == doc_id {
                    document_vector.push(term_score/document_weight);
                    break;
                }
            }
            
        }

        return document_vector;
    }

    fn retrieve_id_file(&self) -> HashMap<u32, String> {
        //read and deserialize the id file for an index
        let id_file_filename = format!("{}/{}", self.index.get_path(), "id_file.bin");
        let mut id_file_file = File::open(id_file_filename).unwrap();
        let mut id_file_contents = String::new();
        id_file_file
            .read_to_string(&mut id_file_contents)
            .expect("Failed to read id file");
        let id_file: HashMap<u32, String> = serde_json::from_str(&id_file_contents).unwrap();
        return id_file;
    }

    fn retrieve_doc_ids(&self) -> Vec<u32> {
        let id_file = self.retrieve_id_file();
        let mut ids: Vec<u32> = Vec::new();
        //unsure if i retrieved the right ids needed for the index
        for key in id_file.keys() {
            ids.push(*key);
        }
        return ids;
    }
}

fn add_vector_components(vec_1: Vec<f64>, vec_2: Vec<f64>) -> Vec<f64> {

    let mut res = Vec::new();
    for (x,y) in vec_1.iter().zip(vec_2.iter()) {
        res.push(x+y);
    }

    return res;
}

fn calculate_euclidian_distance(vec_1: &Vec<f64>, vec_2: &Vec<f64>) ->f64 {
    let mut distance = 0f64;
    for (x,y) in vec_1.iter().zip(vec_2.iter()) {
       distance += (y-x).powi(2);
    }
    return distance.sqrt();
}
    
pub struct RocchioClassifier<'a> {
    index_disputed: &'a DiskInvertedIndex<'a>,
    index_hamilton: &'a DiskInvertedIndex<'a>,
    index_jay: &'a DiskInvertedIndex<'a>,
    index_madison: &'a DiskInvertedIndex<'a>,
}

impl<'a> Classifier<'a> for RocchioClassifier<'a> {
    fn classify(&self, doc_id: u32) -> &'a str {
        /**
         * Perhaps should be void or take in a single doc id and classify that document only?
         * Code needs to be changed in order to support either cahnge as it currently will return after
         * the first document in the disputed list is classified
         */
        let rocchio_for_disputed = Rocchio::new(self.index_disputed);
        let rocchio_for_hamilton = Rocchio::new(self.index_hamilton);
        let rocchio_for_jay = Rocchio::new(self.index_jay);
        let rocchio_for_madison = Rocchio::new(self.index_madison);
       
        let hamilton_centroid = rocchio_for_hamilton.calculate_centroid();
        let jay_centroid = rocchio_for_jay.calculate_centroid();
        let madison_centroid = rocchio_for_madison.calculate_centroid();

        let docs = rocchio_for_disputed.retrieve_doc_ids();

        for doc in docs {

            let x = rocchio_for_disputed.calculate_normalized_vector_for_document(doc);

            let distance_disputed_hamilton = calculate_euclidian_distance(&x,&hamilton_centroid);
            let distance_disputed_jay = calculate_euclidian_distance(&x,&jay_centroid);
            let distance_disputed_madison = calculate_euclidian_distance(&x,&madison_centroid);

            let min = distance_disputed_hamilton.min(distance_disputed_jay.min(distance_disputed_madison));
            if min == distance_disputed_hamilton{
                return "Hamilton";
            }
            else if min == distance_disputed_jay {
                return "Jay";
            }
            else if min == distance_disputed_madison {
                return "Madison";
            } else {
                return "Error";
            }
        }

        
        "placeholder"
    }
    fn get_all_vocab(&self) -> HashSet<String> {
        /**
         * function will work but need a way to do this inside the Rocchio class
         * perhaps the global vocab file we discussed. I attempted to port this function inside
         * the Rocchio class but because the get_vocab method of the disk index class makes a call to 
         * the vocab table member variable I wasn't able to port it
         */
        let vocabulary_disputed = self.index_disputed.get_vocab();
        let vocabulary_hamilton = self.index_hamilton.get_vocab();
        let vocabulary_jay = self.index_jay.get_vocab();
        let vocabulary_madison = self.index_madison.get_vocab();

        let first_union: HashSet<_> = vocabulary_disputed.union(&vocabulary_hamilton).collect();
        let mut first_union_final: HashSet<String> = HashSet::new();
        for vocab in first_union {
            first_union_final.insert(vocab.clone());
        }

        let second_union: HashSet<_> = first_union_final.union(&vocabulary_jay).collect();
        let mut second_union_final: HashSet<String> = HashSet::new();
        for vocab in second_union {
            second_union_final.insert(vocab.clone());
        }

        let third_union: HashSet<_> = second_union_final.union(&vocabulary_madison).collect();
        let mut third_union_final: HashSet<String> = HashSet::new();
        for vocab in third_union {
            third_union_final.insert(vocab.clone());
        }

        third_union_final
    }
}
    


