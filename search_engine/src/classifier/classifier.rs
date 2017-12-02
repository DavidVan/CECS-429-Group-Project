use index::disk_inverted_index::DiskInvertedIndex;

pub trait Classifier<'a> {
    fn classify() -> &'a str;
}
