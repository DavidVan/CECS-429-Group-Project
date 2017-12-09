use std::collections::HashSet;
use std::collections::HashMap;
use index::disk_inverted_index::DiskInvertedIndex;
use index::disk_inverted_index::IndexReader;
use classifier::classifier::Classifier;

pub struct Bayesian<'a> {
    index_disputed: &'a DiskInvertedIndex<'a>,
    index_hamilton: &'a DiskInvertedIndex<'a>,
    index_jay: &'a DiskInvertedIndex<'a>,
    index_madison: &'a DiskInvertedIndex<'a>,
}

impl<'a> Bayesian<'a> {
    fn new(index_disputed: &'a DiskInvertedIndex, index_hamilton: &'a DiskInvertedIndex, index_jay: &'a DiskInvertedIndex, index_madison: &'a DiskInvertedIndex) -> Bayesian<'a> {
        Bayesian {
            index_disputed: index_disputed,
            index_hamilton: index_hamilton,
            index_jay: index_jay,
            index_madison: index_madison,
        }
    }

    fn build_discriminating_vocab_set(&self) -> HashMap<&str, Vec<(u32, u32, u32, u32)>> {

        HashMap::new()
    }
}

impl<'a> Classifier<'a> for Bayesian<'a> {
    fn classify(&self) -> &'a str {
        "hello"
    }
    fn get_all_vocab(&self) -> HashSet<&'a str> {
        let vocabulary_disputed = self.index_disputed.get_vocab();
        let vocabulary_hamilton = self.index_hamilton.get_vocab();
        let vocabulary_jay = self.index_jay.get_vocab();
        let vocabulary_madison = self.index_madison.get_vocab();

        let first_union: HashSet<_> = vocabulary_disputed.union(&vocabulary_hamilton).collect();
        let mut first_union_final: HashSet<&str> = HashSet::new();
        for vocab in first_union {
            first_union_final.insert(vocab);
        }

        let second_union: HashSet<_> = first_union_final.union(&vocabulary_jay).collect();
        let mut second_union_final: HashSet<&str> = HashSet::new();
        for vocab in second_union {
            second_union_final.insert(vocab);
        }

        let third_union: HashSet<_> = second_union_final.union(&vocabulary_madison).collect();
        let mut third_union_final: HashSet<&str> = HashSet::new();
        for vocab in third_union {
            third_union_final.insert(vocab);
        }

        third_union_final
    }
}
