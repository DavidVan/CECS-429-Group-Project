use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::fs::File;
use std::io::prelude::*;
use std::io::Cursor;
use index::positional_inverted_index::PositionalInvertedIndex;
use index::k_gram_index::KGramIndex;

struct IndexWriter<'a> {
    folder_path: &'a str
}

trait DiskIndex {
    fn get_folder_path(&self) -> &str;

    fn build_index(&self, index: &PositionalInvertedIndex) {
        self.build_index_for_directory(index, self.get_folder_path());
    }

    fn build_index_for_directory(&self, index: &PositionalInvertedIndex, folder: &str);
    fn build_vocab_file(&self, folder: &str, dictionary: &Vec<&String>, vocab_positions: &mut Vec<u64>);
    fn build_postings_file(&self, folder: &str, index: &PositionalInvertedIndex, dictionary: &Vec<&String>, vocab_positions: &mut Vec<u64>);
    fn build_kgram_index(&self, folder: &str, kgram: &KGramIndex);
}

impl<'a> IndexWriter<'a> {
    fn new(&self, folder_path: &'a str) -> IndexWriter {
        IndexWriter { folder_path: folder_path }
    }
}

impl<'a> DiskIndex for IndexWriter<'a> {
    fn get_folder_path(&self) -> &str {
        self.folder_path
    }

    fn build_index_for_directory(&self, index: &PositionalInvertedIndex, folder: &str) {
        let dictionary = index.get_dictionary();
        let mut vocab_positions : Vec<u64> = Vec::new();
        self.build_vocab_file(folder, &dictionary, &mut vocab_positions);
        self.build_postings_file(folder, index, &dictionary, &mut vocab_positions);
    }
    
    fn build_vocab_file(&self, folder: &str, dictionary: &Vec<&String>, vocab_positions: &mut Vec<u64>) {
        let mut vocab_list = File::create("vocab.bin").unwrap(); // Might need to enforce ASCII
        let mut vocab_position = 0;
        for vocab_word in dictionary {
            vocab_positions.push(vocab_position);
            vocab_list.write_all(vocab_word.as_bytes());
            vocab_position += vocab_word.len() as u64;
        }
    }

    fn build_postings_file(&self, folder: &str, index: &PositionalInvertedIndex, dictionary: &Vec<&String>, vocab_positions: &mut Vec<u64>) {
        let mut postings_file = File::create("postings.bin").unwrap();
        let mut vocab_table = File::create("vocab_table.bin").unwrap();
        postings_file.write_u32::<LittleEndian>(dictionary.len() as u32);
        let mut vocab_index = 0;
        for s in dictionary {
            let postings = index.get_postings(s);
            let vocab_position = *vocab_positions.get(vocab_index).unwrap(); // Location of vocab
            vocab_table.write_u64::<LittleEndian>(vocab_position);
            let postings_file_size = (&postings_file).bytes().size_hint().1.unwrap() as u64;
            vocab_table.write_u64::<LittleEndian>(postings_file_size);
            let document_frequency = postings.len() as u32;
            postings_file.write_u32::<LittleEndian>(document_frequency);
            let mut last_doc_id = 0;
            for doc_id in postings {
                let doc_id_location = doc_id.get_doc_id() - last_doc_id;
                postings_file.write_u32::<LittleEndian>(doc_id_location);
                last_doc_id = doc_id.get_doc_id();
            }
            vocab_index += 1;
        }
        
    }

    fn build_kgram_index(&self, folder: &str, kgram: &KGramIndex) {
    
    }
}
