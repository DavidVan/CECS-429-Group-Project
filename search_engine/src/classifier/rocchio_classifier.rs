extern crate serde;
extern crate serde_json;

use index::disk_inverted_index::DiskInvertedIndex;
use index::disk_inverted_index::IndexReader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use classifier::classifier::Classifier;

pub struct RocchioClassifier<'a> {
    index_disputed: &'a DiskInvertedIndex<'a>,
    index_hamilton: &'a DiskInvertedIndex<'a>,
    index_jay: &'a DiskInvertedIndex<'a>,
    index_madison: &'a DiskInvertedIndex<'a>,
}

impl<'a> RocchioClassifier<'a> {
    pub fn new(index_disputed: &'a DiskInvertedIndex, index_hamilton: &'a DiskInvertedIndex, index_jay: &'a DiskInvertedIndex, index_madison: &'a DiskInvertedIndex) -> RocchioClassifier<'a> {
        RocchioClassifier {
            index_disputed: index_disputed,
            index_hamilton: index_hamilton,
            index_jay: index_jay,
            index_madison: index_madison,
        }
    }

    fn calculate_centroid(&self, index: &DiskInvertedIndex) -> Vec<f64> {
        let doc_ids = self.retrieve_doc_ids(index);
        let number_of_documents_in_class = doc_ids.len();
        let mut sum_of_docs = self.calculate_normalized_vector_for_document(*doc_ids.get(0).expect("Error retrieving doc ID"), index);
        
        for i in 1..(number_of_documents_in_class - 1) {
            let doc = *doc_ids.get(i).expect("Error Retrieving doc ID");
            sum_of_docs = add_vector_components(self.calculate_normalized_vector_for_document(doc, index), sum_of_docs);
        }
        // println!("Sum of Docs: {:?}", sum_of_docs);
        return sum_of_docs.iter().map(|&x| x/(number_of_documents_in_class as f64)).collect::<Vec<_>>();
    }

    fn calculate_normalized_vector_for_document(&self,doc_id: u32, index: &DiskInvertedIndex) -> Vec<f64> {
        let document_weight = index.get_document_weights(doc_id).unwrap().1;

        let vocab = self.get_all_vocab();
        let mut document_vector: Vec<f64> = Vec::new();
        for term in vocab {
            let res = index.get_postings_no_positions(&term);
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

    fn retrieve_id_file(&self, index: &DiskInvertedIndex) -> HashMap<u32, String> {
        //read and deserialize the id file for an index
        let id_file_filename = format!("{}/{}", index.get_path(), "id_file.bin");
        let mut id_file_file = File::open(id_file_filename).unwrap();
        let mut id_file_contents = String::new();
        id_file_file
            .read_to_string(&mut id_file_contents)
            .expect("Failed to read id file");
        let id_file: HashMap<u32, String> = serde_json::from_str(&id_file_contents).unwrap();
        return id_file;
    }

    fn retrieve_doc_ids(&self, index: &DiskInvertedIndex) -> Vec<u32> {
        let id_file = self.retrieve_id_file(index);
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
    // println!("Vec1: {:?}", vec_1);
    // println!("Vec2: {:?}", vec_2);
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

impl<'a> Classifier<'a> for RocchioClassifier<'a> {
    fn classify(&self, doc_id: u32) -> &'a str {
        
        let hamilton_centroid = self.calculate_centroid(self.index_hamilton);
        let jay_centroid = self.calculate_centroid(self.index_jay);
        let madison_centroid = self.calculate_centroid(self.index_madison);

        // println!("Hamilton Centroid: {:?}\n", hamilton_centroid);
        // println!("Jay Centroid: {:?}\n", jay_centroid);
        // println!("Madison Centroid: {:?}\n", madison_centroid);

        let x = self.calculate_normalized_vector_for_document(doc_id, self.index_disputed);

        // println!("Normalized Vector {:?}\n", x);

        let distance_disputed_hamilton = calculate_euclidian_distance(&x,&hamilton_centroid);
        let distance_disputed_jay = calculate_euclidian_distance(&x,&jay_centroid);
        let distance_disputed_madison = calculate_euclidian_distance(&x,&madison_centroid);

        println!("Hamilton Euclidian Distance: {:?}\n", distance_disputed_hamilton);
        println!("Jay Euclidian Distance: {:?}\n", distance_disputed_jay);
        println!("Madison Euclidian Distance: {:?}\n", distance_disputed_madison);

        let min = distance_disputed_hamilton.min(distance_disputed_jay.min(distance_disputed_madison));
        if min == distance_disputed_hamilton {
            return "Hamilton";
        } else if min == distance_disputed_jay {
            return "Jay";
        } else if min == distance_disputed_madison {
            return "Madison";
        } else {
            return "Error";
        }
        
    }
    fn get_all_vocab(&self) -> HashSet<String> {

        let vocabulary_hamilton = self.index_hamilton.get_vocab();
        let vocabulary_jay = self.index_jay.get_vocab();
        let vocabulary_madison = self.index_madison.get_vocab();

        let first_union: HashSet<_> = vocabulary_hamilton.union(&vocabulary_madison).collect();
        let mut first_union_final: HashSet<String> = HashSet::new();
        for vocab in first_union {
            first_union_final.insert(vocab.clone());
        }

        let second_union: HashSet<_> = first_union_final.union(&vocabulary_jay).collect();
        let mut second_union_final: HashSet<String> = HashSet::new();
        for vocab in second_union {
            second_union_final.insert(vocab.clone());
        }

        second_union_final
    }
}
    


