use byteorder::{WriteBytesExt, BigEndian};
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use std::mem;
use index::positional_inverted_index::PositionalInvertedIndex;
use parser::document_parser::DocumentWeight;

pub struct IndexWriter<'a> {
    folder_path: &'a str
}

pub trait DiskIndex {
    fn get_folder_path(&self) -> &str;

    fn build_index(&self, index: &PositionalInvertedIndex, doc_weights: &Vec<DocumentWeight>, average_doc_length: f64) {
        self.build_index_for_directory(index, doc_weights, average_doc_length, self.get_folder_path());
    }

    fn build_index_for_directory(&self, index: &PositionalInvertedIndex, doc_weights: &Vec<DocumentWeight>, average_doc_length: f64, folder: &str);
    fn build_vocab_file(&self, folder: &str, dictionary: &Vec<&String>, vocab_positions: &mut Vec<u64>);
    fn build_doc_id_file(&self, folder: &str, doc_weights: &Vec<DocumentWeight>, doc_id_positions: &mut Vec<u64>);
    fn build_postings_file(&self, folder: &str, index: &PositionalInvertedIndex, dictionary: &Vec<&String>, vocab_positions: &mut Vec<u64>);
    fn build_doc_weights_file(&self, folder: &str, average_doc_length: f64, doc_weights: &Vec<DocumentWeight>, doc_id_positions: &mut Vec<u64>);
}

impl<'a> IndexWriter<'a> {
    pub fn new(folder_path: &'a str) -> IndexWriter {
        IndexWriter { folder_path: folder_path }
    }
}

impl<'a> DiskIndex for IndexWriter<'a> {
    fn get_folder_path(&self) -> &str {
        self.folder_path
    }

    fn build_index_for_directory(&self, index: &PositionalInvertedIndex, doc_weights: &Vec<DocumentWeight>, average_doc_length: f64, folder: &str) {
        let dictionary = index.get_dictionary();
        let mut vocab_positions : Vec<u64> = Vec::new();
        self.build_vocab_file(folder, &dictionary, &mut vocab_positions);
        self.build_postings_file(folder, index, &dictionary, &mut vocab_positions);
        let mut doc_id_positions : Vec<u64> = Vec::new();
        self.build_doc_id_file(folder, doc_weights, &mut doc_id_positions);
        self.build_doc_weights_file(folder, average_doc_length, &doc_weights, &mut doc_id_positions);
    }
    
    fn build_vocab_file(&self, folder: &str, dictionary: &Vec<&String>, vocab_positions: &mut Vec<u64>) {
        let mut vocab_list = File::create(format!("{}/{}", folder, "vocab.bin")).unwrap(); // Might need to enforce ASCII
        let mut vocab_position = 0;
        for vocab_word in dictionary {
            vocab_positions.push(vocab_position);
            vocab_list.write_all(vocab_word.as_bytes()).expect("Error writing to file");
            vocab_position += vocab_word.len() as u64;
        }
    }

    fn build_doc_id_file(&self, folder: &str, doc_weights: &Vec<DocumentWeight>, doc_id_positions: &mut Vec<u64>) {
        let mut doc_id_list = File::create(format!("{}/{}", folder, "doc_id.bin")).unwrap();
        let mut doc_id_position = 0;
        for doc_id in doc_weights.iter().map(|x| x.get_doc_id()).collect::<Vec<_>>() {
            doc_id_positions.push(doc_id_position);
            doc_id_list.write_u32::<BigEndian>(doc_id).expect("Error writing to file");
            doc_id_position += mem::size_of::<u32>() as u64;
        }
    }

    fn build_postings_file(&self, folder: &str, index: &PositionalInvertedIndex, dictionary: &Vec<&String>, vocab_positions: &mut Vec<u64>) {
        let mut postings_file = File::create(format!("{}/{}", folder, "postings.bin")).unwrap();
        let mut vocab_table = File::create(format!("{}/{}", folder, "vocab_table.bin")).unwrap();

        vocab_table.write_u32::<BigEndian>(dictionary.len() as u32).expect("Error writing to file");
        let mut vocab_index = 0;
        for s in dictionary {
            let postings_file_size = fs::metadata(format!("{}/{}", folder, "postings.bin")).unwrap().len();
            let postings = index.get_postings(s);
            let vocab_position = *vocab_positions.get(vocab_index).unwrap(); // Location of vocab
            vocab_table.write_u64::<BigEndian>(vocab_position).expect("Error writing to file");

            vocab_table.write_u64::<BigEndian>(postings_file_size).expect("Error writing to file");

            let document_frequency = postings.len() as u32;
            postings_file.write_u32::<BigEndian>(document_frequency).expect("Error writing to file");
            let mut last_doc_id = 0;
            for doc_id in postings {
                let doc_id_location = doc_id.get_doc_id() - last_doc_id;
                postings_file.write_u32::<BigEndian>(doc_id_location).expect("Error writing to file");

                let term_score = doc_id.get_term_score();
                postings_file.write_f64::<BigEndian>(term_score).expect("Error writing to file"); //Wdt

                let tf_idf_term_score = doc_id.get_tf_idf_term_score();
                postings_file.write_f64::<BigEndian>(tf_idf_term_score).expect("Error writing to file"); //Wdt

                let okapi_term_score = doc_id.get_okapi_term_score();
                postings_file.write_f64::<BigEndian>(okapi_term_score).expect("Error writing to file"); //Wdt

                let wacky_term_score = doc_id.get_wacky_term_score();
                postings_file.write_f64::<BigEndian>(wacky_term_score).expect("Error writing to file"); //Wdt

                let positions = doc_id.get_positions(); // Get postings positions for every document
                let term_frequency = positions.len() as u32;
                postings_file.write_u32::<BigEndian>(term_frequency).expect("Error writing to file");
                let mut last_pos = 0;
                for pos in positions {
                    let pos_location = pos - last_pos;
                    postings_file.write_u32::<BigEndian>(pos_location).expect("Error writing to file");
                    last_pos = pos;
                }
                last_doc_id = doc_id.get_doc_id();
            }
            vocab_index += 1;
            println!("postings file size change? {}", postings_file_size);
        }
        
    }

    fn build_doc_weights_file(&self, folder: &str, average_doc_length: f64, doc_weights: &Vec<DocumentWeight>, doc_id_positions: &mut Vec<u64>) {
        let mut document_weights = File::create(format!("{}/{}", folder, "doc_weights.bin")).unwrap();
        document_weights.write_f64::<BigEndian>(average_doc_length).expect("Error writing to file");
        for weight in doc_weights {
            document_weights.write_f64::<BigEndian>(weight.get_doc_weight()).expect("Error writing to file");
            document_weights.write_u32::<BigEndian>(weight.get_doc_length()).expect("Error writing to file");
            document_weights.write_u32::<BigEndian>(weight.get_byte_size()).expect("Error writing to file");
            document_weights.write_f64::<BigEndian>(weight.get_avg_tftd()).expect("Error writing to file");
        }
    }

}
