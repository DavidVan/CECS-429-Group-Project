extern crate serde;
extern crate serde_json;

use index::disk_inverted_index::DiskInvertedIndex;
use index::disk_inverted_index::IndexReader;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use classifier::classifier::Classifier;
use classifier::classifier::DocumentClass;
use classifier::classifier::Scalar;
use classifier::classifier::TermComponentScore;

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

    fn calculate_centroid_for_document(&self, index: &DiskInvertedIndex) -> Vec<TermComponentScore> {
        let doc_ids = self.retrieve_doc_ids(index);
        let number_of_documents_in_class = doc_ids.len();
        let mut sum_of_docs = self.calculate_normalized_vector_for_document(*doc_ids.get(0).expect("Error retrieving doc ID"), index);
        
        for i in 1..(number_of_documents_in_class - 1) {
            let doc = *doc_ids.get(i).expect("Error Retrieving doc ID");
            sum_of_docs = add_vector_components(self.calculate_normalized_vector_for_document(doc, index), sum_of_docs);
        }
        // println!("Sum of Docs: {:?}", sum_of_docs);
        let num_docs_in_class_scalar : Scalar = Scalar::new(number_of_documents_in_class as f64);
        sum_of_docs / num_docs_in_class_scalar
    }

    fn calculate_normalized_vector_for_document(&self,doc_id: u32, index: &DiskInvertedIndex) -> Vec<TermComponentScore> {
        let document_weight = index.get_document_weights(doc_id).unwrap().1;

        let vocab_set = self.index_disputed.get_vocab();

        let mut vocab_list : Vec<String> =  Vec::new();

        for vocab in vocab_set {
            vocab_list.push(vocab);
        }

        vocab_list.sort();

        let mut document_vector: Vec<TermComponentScore> = Vec::new();
        for term in vocab_list {
            let res = index.get_postings_no_positions(&term);
            let term_exists = res.is_ok();
            if !term_exists {
                let new_tcs = TermComponentScore::new(0f64, term).expect("Error creating TermComponentScore");
                document_vector.push(new_tcs);
                continue;
            }
            let posting = res.unwrap();
            for (id, _, term_score, _, _, _) in posting {
                if id == doc_id {
                    let new_tcs = TermComponentScore::new((term_score)/(document_weight), term).expect("Error creating TermComponentScore");
                    document_vector.push(new_tcs);
                    break;
                }
            }
        }

        return document_vector;
    }

    fn calculate_centroid_for_index(&self, index: &DiskInvertedIndex) -> Vec<TermComponentScore> {
        let doc_ids = self.retrieve_doc_ids(index);
        let number_of_documents_in_class = doc_ids.len();
        let mut sum_of_docs = self.calculate_normalized_vector_for_index(*doc_ids.get(0).expect("Error retrieving doc ID"), index);
        
        for i in 1..(number_of_documents_in_class - 1) {
            let doc = *doc_ids.get(i).expect("Error Retrieving doc ID");
            sum_of_docs = add_vector_components(self.calculate_normalized_vector_for_index(doc, index), sum_of_docs);
        }
        // println!("Sum of Docs: {:?}", sum_of_docs);
        let num_docs_in_class_scalar : Scalar = Scalar::new(number_of_documents_in_class as f64);
        sum_of_docs / num_docs_in_class_scalar
    }

    fn calculate_normalized_vector_for_index(&self,doc_id: u32, index: &DiskInvertedIndex) -> Vec<TermComponentScore> {
        let document_weight = index.get_document_weights(doc_id).unwrap().1;

        let vocab_set = index.get_vocab();
        let mut vocab_list : Vec<String> =  Vec::new();

        for vocab in vocab_set {
            vocab_list.push(vocab);
        }

        vocab_list.sort();

        let mut document_vector: Vec<TermComponentScore> = Vec::new();
        for term in vocab_list {
            let res = index.get_postings_no_positions(&term);
            let term_exists = res.is_ok();
            if !term_exists {
                let new_tcs = TermComponentScore::new(0f64, term).expect("Error creating TermComponentScore");
                document_vector.push(new_tcs);
                continue;
            }
            let posting = res.unwrap();
            for (id, _, term_score, _, _, _) in posting {
                if id == doc_id {
                    let new_tcs = TermComponentScore::new((term_score)/(document_weight), term).expect("Error creating TermComponentScore");
                    document_vector.push(new_tcs);
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

    pub fn get_hamilton_centroid(&self) -> Vec<TermComponentScore> {
        let mut term_centroid : Vec<TermComponentScore> = Vec::new();
        let components = self.calculate_centroid_for_index(self.index_hamilton);

        let components_clone = components.clone();
        for (i, component) in components_clone.iter().enumerate() {
            if i == 30 {
                break; 
            }
            println!("{}. Term: {}, Score: {}", i + 1, component.term, component.score); 
        }
        components

    }

    pub fn get_madison_centroid(&self) -> Vec<TermComponentScore> {
        let components = self.calculate_centroid_for_index(self.index_madison);
        let components_clone = components.clone();
        for (i, component) in components_clone.iter().enumerate() {
            if i == 30 {
                break; 
            }
            println!("{}. Term: {}, Score: {}", i + 1, component.term, component.score); 
        }
        components
    }

    pub fn get_jay_centroid(&self) -> Vec<TermComponentScore> {
        let components = self.calculate_centroid_for_index(self.index_jay);
        let components_clone = components.clone();
        for (i, component) in components_clone.iter().enumerate() {
            if i == 30 {
                break; 
            }
            println!("{}. Term: {}, Score: {}", i + 1, component.term, component.score); 
        }
        components
    }
}

fn add_vector_components(vec_1: Vec<TermComponentScore>, vec_2: Vec<TermComponentScore>) -> Vec<TermComponentScore> {

    let mut res = Vec::new();
    // println!("Vec1: {:?}", vec_1);
    // println!("Vec2: {:?}", vec_2);
    for (x,y) in vec_1.iter().zip(vec_2.iter()) {
        let x_copy = x.clone();
        let y_copy = y.clone();

        let answer = x_copy + y_copy;
        
        res.push(answer);
    }

    return res;
}

fn calculate_euclidian_distance(vec_1: &Vec<TermComponentScore>, vec_2: &Vec<TermComponentScore>) ->f64 {
    let mut distance = 0f64;
    for (x,y) in vec_1.iter().zip(vec_2.iter()) {
       distance += (y.score - x.score).powi(2);
    }
    return distance.sqrt();
}

impl<'a> Classifier<'a> for RocchioClassifier<'a> {
    fn classify(&self, doc_id: u32) -> &'a str {
        
        let hamilton_centroid = self.calculate_centroid_for_document(self.index_hamilton);
        let jay_centroid = self.calculate_centroid_for_document(self.index_jay);
        let madison_centroid = self.calculate_centroid_for_document(self.index_madison);

        // println!("Hamilton Centroid: {:?}\n", hamilton_centroid);
        // println!("Jay Centroid: {:?}\n", jay_centroid);
        // println!("Madison Centroid: {:?}\n", madison_centroid);

        let components = self.calculate_normalized_vector_for_document(doc_id, self.index_disputed);

        let components_clone = components.clone();
        for (i, component) in components_clone.iter().enumerate() {
            if i == 30 {
                break; 
            }
            println!("{}. Term: {}, Score: {}", i + 1, component.term, component.score); 
        }

        // println!("Normalized Vector {:?}\n", components);

        let distance_disputed_hamilton = calculate_euclidian_distance(&components,&hamilton_centroid);
        let distance_disputed_jay = calculate_euclidian_distance(&components,&jay_centroid);
        let distance_disputed_madison = calculate_euclidian_distance(&components,&madison_centroid);

        // println!("Hamilton Euclidian Distance: {:?}\n", distance_disputed_hamilton);
        // println!("Jay Euclidian Distance: {:?}\n", distance_disputed_jay);
        // println!("Madison Euclidian Distance: {:?}\n", distance_disputed_madison);

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
    fn get_all_vocab(&self) -> Vec<String> {

        let vocabulary_hamilton = self.index_hamilton.get_vocab();
        let vocabulary_jay = self.index_jay.get_vocab();
        let vocabulary_madison = self.index_madison.get_vocab();

        let first_union: HashSet<_> = vocabulary_hamilton.union(&vocabulary_madison).collect();
        let mut first_union_final: HashSet<String> = HashSet::new();
        for vocab in first_union {
            first_union_final.insert(vocab.clone());
        }

        let second_union: HashSet<_> = first_union_final.union(&vocabulary_jay).collect();
        let mut final_union: HashSet<String> = HashSet::new();
        for vocab in second_union {
            final_union.insert(vocab.clone());
        }

        let mut vocab_list : Vec<String> = Vec::new();

        for vocab in final_union {
            vocab_list.push(vocab); 
        }
        vocab_list.sort();
        
        vocab_list
    }
}
    


