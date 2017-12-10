use std::collections::HashSet;
use index::disk_inverted_index::DiskInvertedIndex;

pub trait Classifier<'a> {
    fn classify(&self, doc_id: u32) -> &'a str;
    fn get_all_vocab(&self) -> HashSet<String>;
}
