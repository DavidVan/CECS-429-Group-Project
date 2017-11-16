use byteorder::{ReadBytesExt, BigEndian};
use btree::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::cmp::Ordering;
use std::mem::size_of;

pub struct DiskInvertedIndex<'a> {
    path: &'a str,
    vocab_list: File,
    btree_map: BTree<String, i64>,
    pub postings: File,
    vocab_table: Vec<u64>,
}

pub trait IndexReader {
    fn read_postings_from_file(&self, postings: &File, postings_position: i64) -> Vec<(u32, u32, Vec<u32>)>; // Document ID, tf_td, Positions
    fn read_postings_from_file_no_positions(&self, postings: &File, postings_position: i64) -> Vec<(u32, u32)>; // Document ID, tf_td
    fn get_path(&self) -> String;
    fn get_postings(&self, term: &str) -> Result<Vec<(u32, u32, Vec<u32>)>, &'static str>;
    fn get_postings_no_positions(&self, term: &str) -> Result<Vec<(u32, u32)>, &'static str>;
    fn contains_term(&self, term: &str) -> bool;
    fn get_document_frequency(&self, term: &str) -> u32;
    fn get_term_score(&self, term: &str, doc_id_wanted: u32) -> Option<f64>;
    fn get_term_frequency(&self, term: &str, doc_id_wanted: u32) -> Option<u32>;
    fn binary_search_vocabulary(&self, term: &str) -> i64;
    fn btree_search_vocabulary(&self, term: &str) -> i64;
    fn read_vocab_table(index_name: &str) -> Vec<u64>;
    fn get_term_count(&self) -> u32;
}

impl<'a> DiskInvertedIndex<'a> {
    pub fn new(path: &'a str) -> DiskInvertedIndex {
        DiskInvertedIndex {
            path: path.clone(),
            vocab_list: File::open(format!("{}/{}", path, "vocab.bin")).unwrap(),
            btree_map: BTree::new(&String::from(format!("{}/{}", path, "btree")), size_of::<String>(), size_of::<(i64, i64)>()).unwrap(),
            postings: File::open(format!("{}/{}", path, "postings.bin")).unwrap(),
            vocab_table: DiskInvertedIndex::read_vocab_table(path),
        }
    }
}

impl<'a> IndexReader for DiskInvertedIndex<'a> {
    fn read_postings_from_file(&self, mut postings: &File, postings_position: i64) -> Vec<(u32, u32, Vec<u32>)> {
        let mut results: Vec<(u32, u32, Vec<u32>)> = Vec::new();
        postings.seek(SeekFrom::Start(postings_position as u64)).unwrap();
        let mut doc_freq_buffer = [0; 4]; // Four bytes of 0.
        postings.read_exact(&mut doc_freq_buffer).unwrap();
        println!("Document Frequency: {}", (&doc_freq_buffer[..]).read_u32::<BigEndian>().unwrap());
        let document_frequency = (&doc_freq_buffer[..]).read_u32::<BigEndian>().unwrap();
        let mut doc_id = 0;
        for _ in 0..document_frequency {
            let mut doc_id_buffer = [0; 4];
            postings.read_exact(&mut doc_id_buffer).expect("Error reading buffer");
            println!("Document Id: {}", (&doc_id_buffer[..]).read_u32::<BigEndian>().unwrap());
            doc_id += (&doc_id_buffer[..]).read_u32::<BigEndian>().unwrap();

            let mut term_score_buffer = [0; 8];
            postings.read_exact(&mut term_score_buffer).expect("Error reading buffer");
            println!("Term Score: {}", (&term_score_buffer[..]).read_f64::<BigEndian>().unwrap());

            let mut tf_idf_term_score_buffer = [0; 8];
            postings.read_exact(&mut tf_idf_term_score_buffer).expect("Error reading buffer");
            println!("Term Score: {}", (&tf_idf_term_score_buffer[..]).read_f64::<BigEndian>().unwrap());

            let mut okapi_term_score_buffer = [0; 8];
            postings.read_exact(&mut okapi_term_score_buffer).expect("Error reading buffer");
            println!("Term Score: {}", (&okapi_term_score_buffer[..]).read_f64::<BigEndian>().unwrap());

            let mut wacky_term_score_buffer = [0; 8];
            postings.read_exact(&mut wacky_term_score_buffer).expect("Error reading buffer");
            println!("Term Score: {}", (&wacky_term_score_buffer[..]).read_f64::<BigEndian>().unwrap());

            let mut term_freq_buffer = [0; 4];
            postings.read_exact(&mut term_freq_buffer).expect("Error reading buffer");
            println!("Term Frequency: {}", (&term_freq_buffer[..]).read_u32::<BigEndian>().unwrap());
            let term_frequency = (&term_freq_buffer[..]).read_u32::<BigEndian>().unwrap();

            let mut positions_buffer = [0; 4];
            let mut postings_accumulator = 0;
            let mut positions = Vec::new();
            for j in 0..term_frequency {
                (&self.postings).read_exact(&mut positions_buffer).expect("Error reading from Buffer");
                postings_accumulator += (&positions_buffer[..]).read_u32::<BigEndian>().unwrap();
                println!("Current position: {} for term frequency occurance {}", postings_accumulator, j);
                positions.push(postings_accumulator);
            }
            
            results.push((doc_id, term_frequency, positions));
        }
        results 
    }

    fn read_postings_from_file_no_positions(&self, mut postings: &File, postings_position: i64) -> Vec<(u32, u32)> {
        let mut results: Vec<(u32, u32)> = Vec::new();
        postings.seek(SeekFrom::Start(postings_position as u64)).unwrap();
        let mut doc_freq_buffer = [0; 4]; // Four bytes of 0.
        postings.read_exact(&mut doc_freq_buffer).unwrap();
        println!("Document Frequency: {}", (&doc_freq_buffer[..]).read_u32::<BigEndian>().unwrap());
        let document_frequency = (&doc_freq_buffer[..]).read_u32::<BigEndian>().unwrap();
        let mut doc_id = 0;
        for _ in 0..document_frequency {
            let mut doc_id_buffer = [0; 4];
            postings.read_exact(&mut doc_id_buffer).expect("Error reading buffer");
            println!("Document Id: {}", (&doc_id_buffer[..]).read_u32::<BigEndian>().unwrap());
            doc_id += (&doc_id_buffer[..]).read_u32::<BigEndian>().unwrap();

            let mut term_score_buffer = [0; 8];
            postings.read_exact(&mut term_score_buffer).expect("Error reading buffer");
            println!("Term Score: {}", (&term_score_buffer[..]).read_f64::<BigEndian>().unwrap());

            let mut tf_idf_term_score_buffer = [0; 8];
            postings.read_exact(&mut tf_idf_term_score_buffer).expect("Error reading buffer");
            println!("Term Score: {}", (&tf_idf_term_score_buffer[..]).read_f64::<BigEndian>().unwrap());

            let mut okapi_term_score_buffer = [0; 8];
            postings.read_exact(&mut okapi_term_score_buffer).expect("Error reading buffer");
            println!("Term Score: {}", (&okapi_term_score_buffer[..]).read_f64::<BigEndian>().unwrap());

            let mut wacky_term_score_buffer = [0; 8];
            postings.read_exact(&mut wacky_term_score_buffer).expect("Error reading buffer");
            println!("Term Score: {}", (&wacky_term_score_buffer[..]).read_f64::<BigEndian>().unwrap());

            let mut term_freq_buffer = [0; 4];
            postings.read_exact(&mut term_freq_buffer).expect("Error reading buffer");
            println!("Term Frequency: {}", (&term_freq_buffer[..]).read_u32::<BigEndian>().unwrap());
            let term_frequency = (&term_freq_buffer[..]).read_u32::<BigEndian>().unwrap();

            results.push((doc_id, term_frequency));
            
            postings.seek(SeekFrom::Current((term_frequency * 4) as i64)).expect("Error Seeking From File"); // Skip reading term positions... We only need doc ids.
        }
        results 
    }

    fn get_path(&self) -> String {
        self.path.clone().to_string()
    }

    fn get_document_frequency(&self, term: &str) -> u32 {
        let postings_position = self.binary_search_vocabulary(term);
        (&self.postings).seek(SeekFrom::Start(postings_position as u64)).expect("Error Seeking from Buffer");
        let mut doc_freq_buffer = [0; 4];
        (&self.postings).read_exact(&mut doc_freq_buffer).expect("Error Seeking from Buffer");
        (&doc_freq_buffer[..]).read_u32::<BigEndian>().expect("Error Seeking from Buffer") // Return the document frequency
    }

    fn get_term_score(&self, term: &str, doc_id_wanted: u32) -> Option<f64> {
        let postings_position = self.binary_search_vocabulary(term);
        (&self.postings).seek(SeekFrom::Start(postings_position as u64)).expect("Error reading from Buffer");
        let mut doc_freq_buffer = [0; 4];
        (&self.postings).read_exact(&mut doc_freq_buffer).expect("Error reading from Buffer");
        let document_frequency = (&doc_freq_buffer[..]).read_u32::<BigEndian>().expect("Error reading from Buffer");
        let mut doc_id = 0;
        for _ in 0..document_frequency {
            let mut doc_id_buffer = [0; 4];
            (&self.postings).read_exact(&mut doc_id_buffer).expect("Error reading from Buffer");
            doc_id += (&doc_id_buffer[..]).read_u32::<BigEndian>().expect("Error reading from Buffer");
            let mut term_weight_buffer = [0; 8];
            (&self.postings).read_exact(&mut term_weight_buffer).expect("Error reading from Buffer");
            let term_weight = (&term_weight_buffer[..]).read_f64::<BigEndian>().expect("Error reading from Buffer");
            if doc_id == doc_id_wanted {
                return Some(term_weight);
            }
            (&self.postings).seek(SeekFrom::Current(24)).expect("Error reading from Buffer");
            let mut term_freq_buffer = [0; 4];
            (&self.postings).read_exact(&mut term_freq_buffer).expect("Error reading from Buffer");
            let term_frequency = (&term_freq_buffer[..]).read_u32::<BigEndian>().expect("Error reading from Buffer");
            (&self.postings).seek(SeekFrom::Current((term_frequency * 4) as i64)).expect("Error Seeking from Buffer");
        }
        None
    }

    fn get_term_frequency(&self, term: &str, doc_id_wanted: u32) -> Option<u32> {
        let postings_position = self.binary_search_vocabulary(term);
        (&self.postings).seek(SeekFrom::Start(postings_position as u64)).expect("Error seeking from file)");
        let mut doc_freq_buffer = [0; 4];
        (&self.postings).read_exact(&mut doc_freq_buffer).expect("Error seeking from file)");
        let document_frequency = (&doc_freq_buffer[..]).read_u32::<BigEndian>().expect("Error seeking from file)");
        let mut doc_id = 0;
        for _ in 0..document_frequency {
            let mut doc_id_buffer = [0; 4];
            (&self.postings).read_exact(&mut doc_id_buffer).expect("Error seeking from file)");
            doc_id += (&doc_id_buffer[..]).read_u32::<BigEndian>().expect("Error seeking from file)");
            (&self.postings).seek(SeekFrom::Current(32)).expect("Error seeking from file"); // Skip term weight
            let mut term_freq_buffer = [0; 4];
            (&self.postings).read_exact(&mut term_freq_buffer).expect("Error seeking from file)");
            let term_frequency = (&term_freq_buffer[..]).read_u32::<BigEndian>().expect("Error seeking from file)");
            if doc_id == doc_id_wanted {
                return Some(term_frequency);
            }
            (&self.postings).seek(SeekFrom::Current((term_frequency * 4) as i64)).expect("Error seeking from file)");
        }
        None
    }

    fn get_postings_no_positions(&self, term: &str) -> Result<Vec<(u32, u32)>, &'static str> {
        println!("get postings function called");
        let postings_position = self.binary_search_vocabulary(term);
        match postings_position >= 0 {
            true => Ok(self.read_postings_from_file_no_positions(&self.postings, postings_position)),
            false => Err("Postings position is less than 0."),
        }
    }

    fn get_postings(&self, term: &str) -> Result<Vec<(u32, u32, Vec<u32>)>, &'static str> {
        println!("get postings function called");
        let postings_position = self.binary_search_vocabulary(term);
        match postings_position >= 0 {
            true => Ok(self.read_postings_from_file(&self.postings, postings_position)),
            false => Err("Postings position is less than 0."),
        }
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
            println!("i: {}, j: {}, m: {}", i, j, m);
            let vocab_list_position = self.vocab_table.get(m * 2).unwrap();
            let mut term_length = 0;
            if m == self.vocab_table.len() / 2 - 1 {
                println!("Vocab List File Length: {}", vocab_list.metadata().unwrap().len());
                println!("Vocab Table Position: {}", self.vocab_table[m * 2]);
                term_length = vocab_list.metadata().unwrap().len() as u64 - self.vocab_table[m * 2];
                println!("Term length when m is equal: {}", term_length);
            }
            else {
                println!("Vocab List Pos: {}", vocab_list_position);
                println!("Vocab Table Position: {}", self.vocab_table.get((m + 1) * 2).unwrap());
                term_length = self.vocab_table.get((m + 1) * 2).unwrap() - vocab_list_position;
                println!("Term length when m is not equal: {}", term_length);
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


    fn btree_search_vocabulary(&self, term: &str) -> i64 {
        // self.btree_map.get(&String::from(term))
        -1
    }

    fn read_vocab_table(index_name: &str) -> Vec<u64> {
        let mut table_file = File::open(format!("{}/{}", index_name, "vocab_table.bin")).unwrap();
        let mut vocab_size_buffer= [0; 4];
        table_file.read_exact(&mut vocab_size_buffer).expect("Error reading from file");
        
        let mut table_index = 0;
        let mut vocab_table = vec![0; (&vocab_size_buffer[..]).read_u32::<BigEndian>().unwrap() as usize * 2];
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
}
