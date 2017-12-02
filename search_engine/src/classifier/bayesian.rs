use index::disk_inverted_index::DiskInvertedIndex;
use classifier::classifier::Classifier;

pub struct Bayesian<'a> {
    index: &'a DiskInvertedIndex<'a>,
}

impl<'a> Bayesian<'a> {
    fn new(index: &'a DiskInvertedIndex) -> Bayesian<'a> {
        Bayesian {
            index: index,
        }
    }
}

impl<'a> Classifier<'a> for Bayesian<'a> {
    fn classify(&self) -> &'a str {
        "hello"
    }
}
