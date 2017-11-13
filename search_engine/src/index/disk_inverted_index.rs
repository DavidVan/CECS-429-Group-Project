use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::SeekFrom;
use std::cmp::Ordering;

pub struct DiskInvertedIndex<'a> {
    path: &'a str,
    vocab_list: File,
    postings: File,
    vocab_table: Vec<u64>,
}

pub trait IndexReader {
    fn read_postings_from_file(&self, mut postings: &File, postings_position: i64) -> Vec<u32>;
    fn read_positions_from_file(&self, mut postings: &File, postings_position: i64) -> Vec<u32>;
    fn get_postings(&self, term: &str) -> Result<Vec<u32>, &'static str>;
    fn binary_search_vocabulary(&self, term: &str) -> i64;
    fn read_vocab_table(index_name: &str) -> Vec<u64>;
    fn get_term_count(&self) -> u32;
}

impl<'a> DiskInvertedIndex<'a> {
    pub fn new(path: &'a str) -> DiskInvertedIndex {
        DiskInvertedIndex {
            path: path,
            vocab_list: File::open(format!("{}/{}", path, "vocab.bin")).unwrap(),
            postings: File::open(format!("{}/{}", path, "postings.bin")).unwrap(),
            vocab_table: DiskInvertedIndex::read_vocab_table(path),
        }
    }
}

impl<'a> IndexReader for DiskInvertedIndex<'a> {
    fn read_postings_from_file(&self, mut postings: &File, postings_position: i64) -> Vec<u32> {
        let mut doc_ids: Vec<u32> = Vec::new();
        postings.seek(SeekFrom::Start(postings_position as u64)).unwrap();
        let mut doc_freq_buffer = [0; 4]; // Four bytes of 0.
        postings.read_exact(&mut doc_freq_buffer);
        println!("Document Frequency: {}", (&doc_freq_buffer[..]).read_u32::<BigEndian>().unwrap());
        let document_frequency = (&doc_freq_buffer[..]).read_u32::<BigEndian>().unwrap();
        for i in 0..document_frequency {
            let mut doc_id_buffer = [0; 4];
            postings.read_exact(&mut doc_id_buffer);
            println!("Document Id: {}", (&doc_id_buffer[..]).read_u32::<BigEndian>().unwrap());
            let doc_id = (&doc_id_buffer[..]).read_u32::<BigEndian>().unwrap();
            doc_ids.push(doc_id);

            let mut doc_score_buffer = [0; 8];
            postings.read_exact(&mut doc_score_buffer);
            println!("Document Score: {}", (&doc_score_buffer[..]).read_f64::<BigEndian>().unwrap());
            // Do something with document score here

            let mut term_freq_buffer = [0; 4];
            postings.read_exact(&mut term_freq_buffer);
            println!("Term Frequency: {}", (&term_freq_buffer[..]).read_u32::<BigEndian>().unwrap());
            let term_frequency = (&term_freq_buffer[..]).read_u32::<BigEndian>().unwrap();
            postings.seek(SeekFrom::Current((term_frequency * 4) as i64)); // Skip reading term positions... We only need doc ids.
        }
        doc_ids
    }

    fn read_positions_from_file(&self, mut postings: &File, postings_position: i64) -> Vec<u32> {
        let mut positions = Vec::new();
        postings.seek(SeekFrom::Start((postings_position + 20) as u64)).unwrap();
        let mut positions_buffer = [0; 4];
        let mut postings_accumulator = 0;
        loop {
            match postings.read_exact(&mut positions_buffer) {
                Ok(_) => {
                    postings_accumulator += (&positions_buffer[..]).read_u32::<BigEndian>().unwrap();
                    positions.push(postings_accumulator);
                }
                Err(_) => break
            }
        }
        positions
    }

    fn get_postings(&self, term: &str) -> Result<Vec<u32>, &'static str> {
        println!("get postings function called");
        let postings_position = self.binary_search_vocabulary(term);
        match postings_position >= 0 {
            true => Ok(self.read_postings_from_file(&self.postings, postings_position)),
            false => Err("Postings position is less than 0."),
        }
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
                term_length = vocab_list.metadata().unwrap().len() as u64 - self.vocab_table[m * 2];
            }
            else {
                term_length = self.vocab_table.get((m + 1) * 2).unwrap() - vocab_list_position;
            }

            vocab_list.seek(SeekFrom::Start(*vocab_list_position as u64)).unwrap();

            let mut buffer = vec![0; term_length as usize];
            vocab_list.read_exact(&mut buffer);

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
        println!("Hello from vocab table");
        
        let mut table_file = File::open(format!("{}/{}", index_name, "vocab_table.bin")).unwrap();
        let mut vocab_size_buffer= [0; 4];
        table_file.read_exact(&mut vocab_size_buffer);
        
        let mut table_index = 0;
        let mut vocab_table = vec![0; (&vocab_size_buffer[..]).read_u32::<BigEndian>().unwrap() as usize * 2];
        let mut vocab_pos_buffer = [0; 8];
        loop {
            println!("loop");
            match table_file.read_exact(&mut vocab_pos_buffer) {
                Ok(_) => {
                    vocab_table.push((&vocab_pos_buffer[..]).read_u64::<BigEndian>().unwrap());
                    //table_file.seek(SeekFrom::Current(7)); // Skip the document weights (Ld) values...
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
