extern crate serde_json;
use byteorder::{ReadBytesExt, BigEndian};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;
use index::variable_byte;
use reader::read_file::read_n;

pub struct DiskInvertedIndex<'a> {
    path: &'a str,
    vocab_list: File,
    doc_weights: File,
    pub postings: File,
    vocab_table: Vec<u64>,
}

pub trait IndexReader {
    fn read_postings_from_file(&self, postings: &File, postings_position: i64) -> Vec<(u32, u32, f64, f64, f64, f64, Vec<u32>)>; // Document ID, tf_td, regular term score, tf_idf term score, okapi term score, wacky term score, Positions
    fn read_postings_from_file_no_positions(&self, postings: &File, postings_position: i64) -> Vec<(u32, u32, f64, f64, f64, f64)>; // Document ID, tf_td, regular term score, tf_idf term score, okapi term score, wacky term score
    fn read_doc_weights_from_file(&self, doc_weights: &File, doc_id: u32) -> (f64, f64, u64, u64, f64); // Average Document Length, Document Weight, Document Length, Document Byte Size, Document Average tf-td
    fn get_path(&self) -> String;
    fn get_postings(&self, term: &str) -> Result<Vec<(u32, u32, f64, f64, f64, f64, Vec<u32>)>, &'static str>;
    fn get_postings_no_positions(&self, term: &str) -> Result<Vec<(u32, u32, f64, f64, f64, f64)>, &'static str>;
    fn get_document_weights(&self, doc_id: u32) -> Result<(f64, f64, u64, u64, f64), &'static str>;
    fn get_vocab(&self) -> HashSet<String>;
    fn contains_term(&self, term: &str) -> bool;
    fn get_document_frequency(&self, term: &str) -> u32;
    fn get_terms_for_document(&self, doc_id: u32) -> HashSet<String>;
    fn get_term_frequency(&self, term: &str) -> u32;
    fn get_total_term_frequency(&self) -> u32;
    fn binary_search_vocabulary(&self, term: &str) -> i64;
    fn read_vocab_table(index_name: &str) -> Vec<u64>;
    fn get_term_count(&self) -> u32;
    fn get_num_documents(&self) -> Result<u32, &'static str>;
}

impl<'a> DiskInvertedIndex<'a> {
    pub fn new(path: &'a str) -> DiskInvertedIndex {
        DiskInvertedIndex {
            path: path.clone(),
            vocab_list: File::open(format!("{}/{}", path, "vocab.bin")).expect("Failed to open vocab.bin"),
            doc_weights: File::open(format!("{}/{}", path, "doc_weights.bin")).expect("Failed to open doc_weights.bin"),
            postings: File::open(format!("{}/{}", path, "postings.bin")).expect("Failed to open postings.bin"),
            vocab_table: DiskInvertedIndex::read_vocab_table(path),
        }
    }
}

impl<'a> IndexReader for DiskInvertedIndex<'a> {
    fn read_postings_from_file(&self, mut postings: &File, postings_position: i64) -> Vec<(u32, u32, f64, f64, f64, f64, Vec<u32>)> {
        let mut results: Vec<(u32, u32, f64, f64, f64, f64, Vec<u32>)> = Vec::new();
        postings.seek(SeekFrom::Start(postings_position as u64)).unwrap();
        let mut doc_freq_buffer = [0; 4]; // Four bytes of 0.
        postings.read_exact(&mut doc_freq_buffer).unwrap();
        let document_frequency = (&doc_freq_buffer[..]).read_u32::<BigEndian>().unwrap();
        let mut doc_id = 0;
        for _ in 0..document_frequency {
            let (doc_id_vbe, doc_id_offset) = variable_byte::decode(postings).unwrap();
            postings.seek(SeekFrom::Current(-(5 - doc_id_offset as i64)));

            doc_id += doc_id_vbe;

            let mut term_score_buffer = [0; 8];
            postings.read_exact(&mut term_score_buffer).expect("Error reading buffer");
            let term_score = (&term_score_buffer[..]).read_f64::<BigEndian>().unwrap();

            let mut tf_idf_term_score_buffer = [0; 8];
            postings.read_exact(&mut tf_idf_term_score_buffer).expect("Error reading buffer");
            let tf_idf_term_score = (&tf_idf_term_score_buffer[..]).read_f64::<BigEndian>().unwrap();

            let mut okapi_term_score_buffer = [0; 8];
            postings.read_exact(&mut okapi_term_score_buffer).expect("Error reading buffer");
            let okapi_term_score = (&okapi_term_score_buffer[..]).read_f64::<BigEndian>().unwrap();

            let mut wacky_term_score_buffer = [0; 8];
            postings.read_exact(&mut wacky_term_score_buffer).expect("Error reading buffer");
            let wacky_term_score = (&wacky_term_score_buffer[..]).read_f64::<BigEndian>().unwrap();

            let (term_frequency_vbe, term_freq_offset) = variable_byte::decode(postings).unwrap();
            postings.seek(SeekFrom::Current(-(5 - term_freq_offset as i64)));

            let mut postings_accumulator = 0;
            let mut positions = Vec::new();
            for j in 0..term_frequency_vbe {
                let (postings_pos_vbe, postings_pos_offset) = variable_byte::decode(postings).unwrap();
                postings.seek(SeekFrom::Current(-(5 - postings_pos_offset as i64)));

                postings_accumulator += postings_pos_vbe;

                positions.push(postings_accumulator);
            }
            
            results.push((doc_id, term_frequency_vbe, term_score, tf_idf_term_score, okapi_term_score, wacky_term_score, positions));
        }
        results 
    }

    fn read_postings_from_file_no_positions(&self, mut postings: &File, postings_position: i64) -> Vec<(u32, u32, f64, f64, f64, f64)> {
        let mut results: Vec<(u32, u32, f64, f64, f64, f64)> = Vec::new();
        postings.seek(SeekFrom::Start(postings_position as u64)).unwrap();
        let mut doc_freq_buffer = [0; 4]; // Four bytes of 0.
        postings.read_exact(&mut doc_freq_buffer).unwrap();
        let document_frequency = (&doc_freq_buffer[..]).read_u32::<BigEndian>().unwrap();
        let mut doc_id = 0;
        for _ in 0..document_frequency {
            let (doc_id_vbe, doc_id_offset) = variable_byte::decode(postings).unwrap();
            postings.seek(SeekFrom::Current(-(5 - doc_id_offset as i64)));

            doc_id += doc_id_vbe;

            let mut term_score_buffer = [0; 8];
            postings.read_exact(&mut term_score_buffer).expect("Error reading buffer");
            let term_score = (&term_score_buffer[..]).read_f64::<BigEndian>().unwrap();

            let mut tf_idf_term_score_buffer = [0; 8];
            postings.read_exact(&mut tf_idf_term_score_buffer).expect("Error reading buffer");
            let tf_idf_term_score = (&tf_idf_term_score_buffer[..]).read_f64::<BigEndian>().unwrap();

            let mut okapi_term_score_buffer = [0; 8];
            postings.read_exact(&mut okapi_term_score_buffer).expect("Error reading buffer");
            let okapi_term_score = (&okapi_term_score_buffer[..]).read_f64::<BigEndian>().unwrap();

            let mut wacky_term_score_buffer = [0; 8];
            postings.read_exact(&mut wacky_term_score_buffer).expect("Error reading buffer");
            let wacky_term_score = (&wacky_term_score_buffer[..]).read_f64::<BigEndian>().unwrap();

            let (term_frequency_vbe, term_freq_offset) = variable_byte::decode(postings).unwrap();
            postings.seek(SeekFrom::Current(-(5 - term_freq_offset as i64)));

            results.push((doc_id, term_frequency_vbe, term_score, tf_idf_term_score, okapi_term_score, wacky_term_score));
            
            for _ in 0..term_frequency_vbe {
                let (postings_pos_vbe, postings_pos_offset) = variable_byte::decode(postings).unwrap();
                postings.seek(SeekFrom::Current(-(5 - postings_pos_offset as i64)));
            }
        }
        results 
    }

    fn read_doc_weights_from_file(&self, mut doc_weights: &File, doc_id: u32) -> (f64, f64, u64, u64, f64) {
        doc_weights.seek(SeekFrom::Start(0));
        let mut avg_doc_length_buffer = [0; 8];
        doc_weights.read_exact(&mut avg_doc_length_buffer).unwrap();
        let avg_doc_length = (&avg_doc_length_buffer[..]).read_f64::<BigEndian>().unwrap();

        doc_weights.seek(SeekFrom::Start(doc_id as u64 * 4 * 8 + 8)).unwrap(); // Doc ids are written in increasing order. We write four 8 byte values for each document id. First 8 bytes of the file is used for all documents.

        let mut doc_weight_buffer = [0; 8];
        doc_weights.read_exact(&mut doc_weight_buffer).unwrap();
        let doc_weight = (&doc_weight_buffer[..]).read_f64::<BigEndian>().unwrap();

        let mut doc_length_buffer = [0; 8];
        doc_weights.read_exact(&mut doc_length_buffer).unwrap();
        let doc_length = (&doc_length_buffer[..]).read_u64::<BigEndian>().unwrap();

        let mut byte_size_buffer = [0; 8];
        doc_weights.read_exact(&mut byte_size_buffer).unwrap();
        let byte_size = (&byte_size_buffer[..]).read_u64::<BigEndian>().unwrap();

        let mut avg_tftd_buffer = [0; 8];
        doc_weights.read_exact(&mut avg_tftd_buffer).unwrap();
        let avg_tftd = (&avg_tftd_buffer[..]).read_f64::<BigEndian>().unwrap();

        (avg_doc_length, doc_weight, doc_length, byte_size, avg_tftd)
    }
    
    fn get_path(&self) -> String {
        self.path.clone().to_string()
    }

    fn get_document_frequency(&self, term: &str) -> u32 {
        let postings_position = self.binary_search_vocabulary(term);
        if postings_position == -1 {
            return 0;
        }
        (&self.postings).seek(SeekFrom::Start(postings_position as u64)).expect("Error Seeking from Buffer");
        let mut doc_freq_buffer = [0; 4];
        (&self.postings).read_exact(&mut doc_freq_buffer).expect("Error Seeking from Buffer");
        (&doc_freq_buffer[..]).read_u32::<BigEndian>().expect("Error Seeking from Buffer") // Return the document frequency
    }

    fn get_terms_for_document(&self, doc_id: u32) -> HashSet<String> {
        let terms = self.get_vocab();
        let mut results = HashSet::new();
        // results.push((doc_id, term_frequency_vbe, term_score, tf_idf_term_score, okapi_term_score, wacky_term_score, positions));
        for term in &terms {
            let postings = self.get_postings_no_positions(term).unwrap();
            for posting in postings {
                let (doc_id_term, _, _, _, _, _) = posting;
                if doc_id == doc_id_term {
                    results.insert(term.clone());
                }
            }
        }
        results
    }

    fn get_term_frequency(&self, term: &str) -> u32 {
        let mut postings = &self.postings;
        let postings_position = self.binary_search_vocabulary(term);

        if postings_position == -1 {
            return 0; 
        }
        
        postings.seek(SeekFrom::Start(postings_position as u64)).expect("(189)");
        let mut doc_freq_buffer = [0; 4]; // Four bytes of 0.
        postings.read_exact(&mut doc_freq_buffer).expect("191");
        let document_frequency = (&doc_freq_buffer[..]).read_u32::<BigEndian>().expect("192");
        let mut doc_id = 0;
        let mut term_frequency_accumulator = 0;
        for _ in 0..document_frequency {
            let (doc_id_vbe, doc_id_offset) = variable_byte::decode(postings).expect("196");
            postings.seek(SeekFrom::Current(-(5 - doc_id_offset as i64))).expect("197");

            doc_id += doc_id_vbe;

            postings.seek(SeekFrom::Current(32));

            let (term_frequency_vbe, term_freq_offset) = variable_byte::decode(postings).unwrap();
            postings.seek(SeekFrom::Current(-(5 - term_freq_offset as i64)));

            term_frequency_accumulator += term_frequency_vbe;

            for _ in 0..term_frequency_vbe {
                let (postings_pos_vbe, postings_pos_offset) = variable_byte::decode(postings).unwrap();
                postings.seek(SeekFrom::Current(-(5 - postings_pos_offset as i64)));
            }
        }
        term_frequency_accumulator
    }

    fn get_total_term_frequency(&self) -> u32 {
        let vocabulary = self.get_vocab();

        let mut total_term_frequency = 0;
        for term in vocabulary {
            total_term_frequency += self.get_term_frequency(&term); 
        }

        total_term_frequency

    }

    fn get_postings_no_positions(&self, term: &str) -> Result<Vec<(u32, u32, f64, f64, f64, f64)>, &'static str> {
        let postings_position = self.binary_search_vocabulary(term);
        match postings_position >= 0 {
            true => Ok(self.read_postings_from_file_no_positions(&self.postings, postings_position)),
            false => Err("Postings position is less than 0."),
        }
    }

    fn get_postings(&self, term: &str) -> Result<Vec<(u32, u32, f64, f64, f64, f64, Vec<u32>)>, &'static str> {
        let postings_position = self.binary_search_vocabulary(term);
        match postings_position >= 0 {
            true => Ok(self.read_postings_from_file(&self.postings, postings_position)),
            false => Err("Postings position is less than 0."),
        }
    }

    fn get_document_weights(&self, doc_id: u32) -> Result<(f64, f64, u64, u64, f64), &'static str> {
        match doc_id >= 0 {
            true => Ok(self.read_doc_weights_from_file(&self.doc_weights, doc_id)),
            false => Err("Document id not found."),
        }
    }

    fn get_vocab(&self) -> HashSet<String> {

        let mut vocab_file = File::open(format!("{}/{}", self.path, "vocab.bin")).unwrap();

        let mut vocab_dict : HashSet<String> = HashSet::new();

        let mut contents = String::new();

        vocab_file.read_to_string(&mut contents).expect("Error reading file");

        let vocab_table = &self.vocab_table;

        let mut first_pos : u64 = 0;
        let mut second_pos : u64 = 0;
        for (index, position) in vocab_table.iter().enumerate() {
            if index % 2 != 0 {
                continue; 
            }

            if index == 0 {
                continue;
            }

            first_pos = second_pos;
            second_pos = *position;

            let term = &contents[first_pos as usize..second_pos as usize];
            vocab_dict.insert(term.to_owned());
        }
        let term = &contents[second_pos as usize..];
        vocab_dict.insert(term.to_owned());

        vocab_dict

    }

    fn contains_term(&self, term: &str) -> bool {
        return self.binary_search_vocabulary(term) != -1; 
    }

    fn binary_search_vocabulary(&self, term: &str) -> i64 {
        let mut vocab_list = &self.vocab_list;
        let mut i = 0;
        let mut j = self.vocab_table.len() / 2 - 1;
        while i <= j {
            let m = (i + j) / 2;
            let vocab_list_position = self.vocab_table.get(m * 2).unwrap();
            let mut term_length = 0;
            if m == self.vocab_table.len() / 2 - 1 {
                term_length = vocab_list.metadata().unwrap().len() as u64 - self.vocab_table[m * 2];
            }
            else {
                term_length = self.vocab_table.get((m + 1) * 2).unwrap() - vocab_list_position;
            }

            vocab_list.seek(SeekFrom::Start(*vocab_list_position as u64)).unwrap();

            let mut buffer = vec![0; term_length as usize];
            vocab_list.read_exact(&mut buffer).expect("Error reading from file");

            let file_term = String::from_utf8(buffer).unwrap();

            let compare_value = term.cmp(&file_term);

            match compare_value {
                Ordering::Equal => return *(self.vocab_table.get(m * 2 + 1)).unwrap() as i64,
                Ordering::Less => j = m - 1,
                Ordering::Greater => i = m + 1
            }
        }
        -1
    }

    fn read_vocab_table(index_name: &str) -> Vec<u64> {
        let mut table_file = File::open(format!("{}/{}", index_name, "vocab_table.bin")).unwrap();
        let mut vocab_size_buffer= [0; 4];
        table_file.read_exact(&mut vocab_size_buffer).expect("Error reading from file");
        
        let mut table_index = 0;
        let mut vocab_table : Vec<u64> = Vec::new(); 
        let mut vocab_pos_buffer = [0; 8];
        loop {
            match table_file.read_exact(&mut vocab_pos_buffer) {
                Ok(_) => {
                    vocab_table.push((&vocab_pos_buffer[..]).read_u64::<BigEndian>().unwrap());
                    table_index += 1;
                },
                Err(_) => break
            }
        }
        vocab_table
    }

    fn get_term_count(&self) -> u32 {
        return self.vocab_table.len() as u32 / 2;
    }
    fn get_num_documents(&self) -> Result<u32, &'static str> {
        let path = self.get_path();
        let id_file_filename = format!("{}/{}", path, "id_file.bin");

        let mut id_file_file = File::open(id_file_filename).unwrap();

        let mut id_file_contents = String::new();
        id_file_file.read_to_string(&mut id_file_contents).expect("Failed to read id file");

        let id_file : HashMap<u32, String> = serde_json::from_str(&id_file_contents).unwrap();

        let num_documents = id_file.len();

        match num_documents > 0 {
            true => Ok(num_documents as u32),
            false => Err("Error: No documents found"),
        }
    }
}
