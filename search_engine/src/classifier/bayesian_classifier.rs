use std::collections::HashSet;
use std::collections::HashMap;
use index::disk_inverted_index::DiskInvertedIndex;
use index::disk_inverted_index::IndexReader;
use classifier::classifier::Classifier;

pub enum DocumentClass {
    Disputed,
    Hamilton,
    Jay,
    Madison,
}

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

    fn get_total_num_documents(&self) -> Result<u32, &'static str> { // Nt (or just N), total number of documents for training set.
        let disputed_total_num = self.index_disputed.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class.
        let hamilton_total_num = self.index_hamilton.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let jay_total_num = self.index_jay.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let madison_total_num= self.index_madison.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 

        let mut total_num = 0;
        total_num += disputed_total_num;
        total_num += hamilton_total_num;
        total_num += jay_total_num;
        total_num += madison_total_num;

        match total_num > 0 {
            true => Ok(total_num),
            false => Err("Error: No Documents Found in Index"),
        }
    }

    fn get_n_00(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // N00, total number of documents that DO NOT contain term t and NOT in class c.
        let disputed_total_num = self.index_disputed.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class.
        let hamilton_total_num = self.index_hamilton.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let jay_total_num = self.index_jay.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let madison_total_num= self.index_madison.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 

        let disputed_postings_for_term = self.index_disputed.get_postings_no_positions(term);
        let num_in_disputed_for_term = match disputed_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let hamilton_postings_for_term = self.index_hamilton.get_postings_no_positions(term);
        let num_in_hamilton_for_term = match hamilton_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let jay_postings_for_term = self.index_jay.get_postings_no_positions(term);
        let num_in_jay_for_term = match jay_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let madison_postings_for_term = self.index_madison.get_postings_no_positions(term);
        let num_in_madison_for_term = match madison_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let disputed_num_without_term = disputed_total_num - num_in_disputed_for_term;
        let hamilton_num_without_term = hamilton_total_num - num_in_hamilton_for_term; 
        let jay_num_without_term = jay_total_num - num_in_jay_for_term;
        let madison_num_without_term = madison_total_num - num_in_madison_for_term;

        let n_00 = match *class {
            DocumentClass::Disputed => hamilton_num_without_term + jay_num_without_term + madison_num_without_term,
            DocumentClass::Hamilton => disputed_num_without_term + jay_num_without_term + madison_num_without_term,
            DocumentClass::Jay => disputed_num_without_term + hamilton_num_without_term + madison_num_without_term,
            DocumentClass::Madison => disputed_num_without_term + hamilton_num_without_term + jay_num_without_term,
        };

        Ok(n_00)
    }

    fn get_n_01(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // N01, total number of documents that DO contain term t and NOT in class c.
        let disputed_postings_for_term = self.index_disputed.get_postings_no_positions(term);
        let num_in_disputed_for_term = match disputed_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let hamilton_postings_for_term = self.index_hamilton.get_postings_no_positions(term);
        let num_in_hamilton_for_term = match hamilton_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let jay_postings_for_term = self.index_jay.get_postings_no_positions(term);
        let num_in_jay_for_term = match jay_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let madison_postings_for_term = self.index_madison.get_postings_no_positions(term);
        let num_in_madison_for_term = match madison_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let n_01 = match *class {
            DocumentClass::Disputed => num_in_hamilton_for_term + num_in_jay_for_term + num_in_madison_for_term,
            DocumentClass::Hamilton => num_in_disputed_for_term + num_in_jay_for_term + num_in_madison_for_term,
            DocumentClass::Jay => num_in_disputed_for_term + num_in_hamilton_for_term + num_in_madison_for_term,
            DocumentClass::Madison => num_in_disputed_for_term + num_in_hamilton_for_term + num_in_jay_for_term,
        };

        Ok(n_01)
    }

    fn get_n_10(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // N10, total number of documents that DO NOT contain term t but IS in class c.
        let disputed_total_num = self.index_disputed.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class.
        let hamilton_total_num = self.index_hamilton.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let jay_total_num = self.index_jay.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let madison_total_num= self.index_madison.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 

        let disputed_postings_for_term = self.index_disputed.get_postings_no_positions(term);
        let num_in_disputed_for_term = match disputed_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let hamilton_postings_for_term = self.index_hamilton.get_postings_no_positions(term);
        let num_in_hamilton_for_term = match hamilton_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let jay_postings_for_term = self.index_jay.get_postings_no_positions(term);
        let num_in_jay_for_term = match jay_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let madison_postings_for_term = self.index_madison.get_postings_no_positions(term);
        let num_in_madison_for_term = match madison_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let disputed_num_without_term = disputed_total_num - num_in_disputed_for_term;
        let hamilton_num_without_term = hamilton_total_num - num_in_hamilton_for_term; 
        let jay_num_without_term = jay_total_num - num_in_jay_for_term;
        let madison_num_without_term = madison_total_num - num_in_madison_for_term;

        let n_10 = match *class {
            DocumentClass::Disputed => disputed_num_without_term,
            DocumentClass::Hamilton => hamilton_num_without_term,
            DocumentClass::Jay => jay_num_without_term,
            DocumentClass::Madison => madison_num_without_term,
        };

        Ok(n_10)
    }

    fn get_n_11(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // N11, total number of documents that DO contain term t and IS in class c.
        let disputed_postings_for_term = self.index_disputed.get_postings_no_positions(term);
        let num_in_disputed_for_term = match disputed_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let hamilton_postings_for_term = self.index_hamilton.get_postings_no_positions(term);
        let num_in_hamilton_for_term = match hamilton_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let jay_postings_for_term = self.index_jay.get_postings_no_positions(term);
        let num_in_jay_for_term = match jay_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let madison_postings_for_term = self.index_madison.get_postings_no_positions(term);
        let num_in_madison_for_term = match madison_postings_for_term {
            Ok(postings) => postings.len() as u32,
            Err(_) => 0,
        };

        let n_11 = match *class {
            DocumentClass::Disputed => num_in_disputed_for_term,
            DocumentClass::Hamilton => num_in_hamilton_for_term,
            DocumentClass::Jay => num_in_jay_for_term,
            DocumentClass::Madison => num_in_madison_for_term,
        };

        Ok(n_11)
    }

    fn get_n_0X(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // N0X, N00 + N01
        let n_00 = match self.get_n_00(term, class) {
            Ok(n_00) => n_00,
            Err(_) => panic!("Something happened when calculating N00!"),
        };
        let n_01 = match self.get_n_01(term, class) {
            Ok(n_01) => n_01,
            Err(_) => panic!("Something happened when calculating N01!"),
        };

        let n_0X = n_00 + n_01;

        Ok(n_0X)
    }

    fn get_n_X0(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // NX0, N00 + N10
        let n_00 = match self.get_n_00(term, class) {
            Ok(n_00) => n_00,
            Err(_) => panic!("Something happened when calculating N00!"),
        };
        let n_11 = match self.get_n_11(term, class) {
            Ok(n_11) => n_11,
            Err(_) => panic!("Something happened when calculating N11!"),
        };

        let n_X0 = n_00 + n_11;

        Ok(n_X0)
    }

    fn get_n_1X(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // N1X, N10 + N11
        let n_10 = match self.get_n_10(term, class) {
            Ok(n_10) => n_10,
            Err(_) => panic!("Something happened when calculating N10!"),
        };
        let n_11 = match self.get_n_11(term, class) {
            Ok(n_11) => n_11,
            Err(_) => panic!("Something happened when calculating N11!"),
        };

        let n_1X = n_10 + n_11;

        Ok(n_1X)
    }

    fn get_n_X1(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // NX1, N01 + N11
        let n_01 = match self.get_n_01(term, class) {
            Ok(n_01) => n_01,
            Err(_) => panic!("Something happened when calculating N01!"),
        };
        let n_11 = match self.get_n_11(term, class) {
            Ok(n_11) => n_11,
            Err(_) => panic!("Something happened when calculating N11!"),
        };

        let n_X1 = n_01 + n_11;

        Ok(n_X1)
    }
}

impl<'a> Classifier<'a> for BayesianClassifier<'a> {
    fn classify(&self, doc_id: u32) -> &'a str {
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
