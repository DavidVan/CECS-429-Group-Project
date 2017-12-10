use std::collections::HashSet;
use std::collections::HashMap;
use index::disk_inverted_index::DiskInvertedIndex;
use index::disk_inverted_index::IndexReader;
use classifier::classifier::Classifier;

pub struct BayesianClassifier<'a> {
    index_disputed: &'a DiskInvertedIndex<'a>,
    index_hamilton: &'a DiskInvertedIndex<'a>,
    index_jay: &'a DiskInvertedIndex<'a>,
    index_madison: &'a DiskInvertedIndex<'a>,
}

impl<'a> BayesianClassifier<'a> {
    pub fn new(index_disputed: &'a DiskInvertedIndex, index_hamilton: &'a DiskInvertedIndex, index_jay: &'a DiskInvertedIndex, index_madison: &'a DiskInvertedIndex) -> BayesianClassifier<'a> {
        BayesianClassifier {
            index_disputed: index_disputed,
            index_hamilton: index_hamilton,
            index_jay: index_jay,
            index_madison: index_madison,
        }
    }

    pub fn build_discriminating_vocab_set(&self) -> HashMap<&str, Vec<(u32, u32, u32, u32)>> {
        let test = self.get_all_vocab();
        for x in &test {
            println!("test: {}", x);
        }
        println!("Size of all vocab: {}", test.len());
        HashMap::new()
    }

    fn get_total_num_documents(&self) -> Result<u32, &'static str> {
        let hamilton_num = self.index_hamilton.get_num_documents().expect("No Documents found!"); 
        let madison_num= self.index_madison.get_num_documents().expect("No Documents found!"); 
        let jay_num = self.index_jay.get_num_documents().expect("No Documents found!"); 
        let disputed_num = self.index_disputed.get_num_documents().expect("No Documents found!");

        let mut total_num = 0;
        total_num += hamilton_num;
        total_num += madison_num;
        total_num += jay_num;
        total_num += disputed_num;

        match total_num > 0 {
            true => Ok(total_num),
            false => Err("Error: No Documents Found in Index"),
        }
    }
}

impl<'a> Classifier<'a> for BayesianClassifier<'a> {
    fn classify(&self) -> &'a str {
        "hello"
    }
    fn get_all_vocab(&self) -> HashSet<String> {
        let vocabulary_disputed = self.index_disputed.get_vocab();
        let vocabulary_hamilton = self.index_hamilton.get_vocab();
        let vocabulary_jay = self.index_jay.get_vocab();
        let vocabulary_madison = self.index_madison.get_vocab();

        let first_union: HashSet<_> = vocabulary_disputed.union(&vocabulary_hamilton).collect();
        let mut first_union_final: HashSet<String> = HashSet::new();
        for vocab in first_union {
            first_union_final.insert(vocab.clone());
        }

        let second_union: HashSet<_> = first_union_final.union(&vocabulary_jay).collect();
        let mut second_union_final: HashSet<String> = HashSet::new();
        for vocab in second_union {
            second_union_final.insert(vocab.clone());
        }

        let third_union: HashSet<_> = second_union_final.union(&vocabulary_madison).collect();
        let mut third_union_final: HashSet<String> = HashSet::new();
        for vocab in third_union {
            third_union_final.insert(vocab.clone());
        }

        third_union_final
    }

}
