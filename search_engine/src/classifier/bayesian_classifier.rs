use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use index::disk_inverted_index::DiskInvertedIndex;
use index::disk_inverted_index::IndexReader;
use classifier::classifier::Classifier;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum DocumentClass {
    Hamilton,
    Jay,
    Madison,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TermClassScore {
    score: f64,
    term: String,
    class: DocumentClass,
}

impl TermClassScore {
    fn new(score: f64, term: String, class: DocumentClass) -> Option<TermClassScore> {
        if score.is_nan() {
            None
        }
        else {
            Some(TermClassScore {
                score: score,
                term: term,
                class: class,
           })
        }
    }
}

impl Eq for TermClassScore {

}

impl Ord for TermClassScore {
    fn cmp(&self, other: &TermClassScore) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}

pub struct BayesianClassifier<'a> {
    index_hamilton: &'a DiskInvertedIndex<'a>,
    index_jay: &'a DiskInvertedIndex<'a>,
    index_madison: &'a DiskInvertedIndex<'a>,
}

impl<'a> BayesianClassifier<'a> {
    pub fn new(index_hamilton: &'a DiskInvertedIndex, index_jay: &'a DiskInvertedIndex, index_madison: &'a DiskInvertedIndex) -> BayesianClassifier<'a> {
        BayesianClassifier {
            index_hamilton: index_hamilton,
            index_jay: index_jay,
            index_madison: index_madison,
        }
    }

    pub fn build_discriminating_vocab_set(&self, k: u32) -> Vec<TermClassScore> {
        let all_vocabulary = self.get_all_vocab();
        let classes = vec!(DocumentClass::Hamilton, DocumentClass::Jay, DocumentClass::Madison);

        let mut priority_queue: BinaryHeap<TermClassScore> = BinaryHeap::new();

        for class in classes {
            for term in &all_vocabulary {
                let score = match self.calculate_mutual_information_score(term.clone(), class) {
                    Ok(score) => match TermClassScore::new(score, term.clone(), class) {
                        Some(thing) => {
                            println!("Adding to priority queue: {:?}", thing);
                            priority_queue.push(thing);
                        },
                        None => continue
                    },
                    Err(error) => panic!("There was an error calculating the score for term {}, class {:?}. The error is: {}", term, class, error),
                };
            }
        }
        let mut discriminating_vocab = Vec::new();
        for i in 0..k {
            match priority_queue.pop() {
                Some(from_priority_queue) => {
                    discriminating_vocab.push(from_priority_queue);
                }
                None => panic!("Removing from priority queue, but nothing is in the priority queue..."),
            }
        }

        discriminating_vocab
    }

    fn get_total_num_documents(&self) -> Result<u32, &'static str> { // Nt (or just N), total number of documents for training set.
        let hamilton_total_num = self.index_hamilton.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let jay_total_num = self.index_jay.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let madison_total_num = self.index_madison.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 

        let mut total_num = 0;
        total_num += hamilton_total_num;
        total_num += jay_total_num;
        total_num += madison_total_num;

        match total_num > 0 {
            true => Ok(total_num),
            false => Err("Error: No Documents Found in Index"),
        }
    }

    fn get_n_00(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // N00, total number of documents that DO NOT contain term t and NOT in class c.
        let hamilton_total_num = self.index_hamilton.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let jay_total_num = self.index_jay.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let madison_total_num = self.index_madison.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 

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

        let hamilton_num_without_term = hamilton_total_num - num_in_hamilton_for_term; 
        let jay_num_without_term = jay_total_num - num_in_jay_for_term;
        let madison_num_without_term = madison_total_num - num_in_madison_for_term;

        let n_00 = match *class {
            DocumentClass::Hamilton => jay_num_without_term + madison_num_without_term,
            DocumentClass::Jay => hamilton_num_without_term + madison_num_without_term,
            DocumentClass::Madison => hamilton_num_without_term + jay_num_without_term,
        };

        Ok(n_00)
    }

    fn get_n_01(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // N01, total number of documents that DO contain term t and NOT in class c.
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
            DocumentClass::Hamilton => num_in_jay_for_term + num_in_madison_for_term,
            DocumentClass::Jay => num_in_hamilton_for_term + num_in_madison_for_term,
            DocumentClass::Madison => num_in_hamilton_for_term + num_in_jay_for_term,
        };

        Ok(n_01)
    }

    fn get_n_10(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // N10, total number of documents that DO NOT contain term t but IS in class c.
        let hamilton_total_num = self.index_hamilton.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let jay_total_num = self.index_jay.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let madison_total_num = self.index_madison.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 

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

        let hamilton_num_without_term = hamilton_total_num - num_in_hamilton_for_term; 
        let jay_num_without_term = jay_total_num - num_in_jay_for_term;
        let madison_num_without_term = madison_total_num - num_in_madison_for_term;

        let n_10 = match *class {
            DocumentClass::Hamilton => hamilton_num_without_term,
            DocumentClass::Jay => jay_num_without_term,
            DocumentClass::Madison => madison_num_without_term,
        };

        Ok(n_10)
    }

    fn get_n_11(&self, term: &str, class: &DocumentClass) -> Result<u32, &'static str> { // N11, total number of documents that DO contain term t and IS in class c.
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

    fn calculate_mutual_information_score(&self, term: String, class: DocumentClass) -> Result<f64, &'static str> {
        let n = self.get_all_vocab().len() as u32;

        let n_00 = match self.get_n_00(&term, &class) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_00! Error: {}", error),
        };
        let n_01 = match self.get_n_01(&term, &class) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_01! Error: {}", error),
        };
        let n_10 = match self.get_n_10(&term, &class) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_10! Error: {}", error),
        };
        let n_11 = match self.get_n_11(&term, &class) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_11! Error: {}", error),
        };
        
        let n_0X = match self.get_n_0X(&term, &class) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_0X! Error: {}", error),
        };
        let n_X0 = match self.get_n_X0(&term, &class) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_X0! Error: {}", error),
        };
        let n_1X = match self.get_n_1X(&term, &class) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_1X! Error: {}", error),
        };
        let n_X1 = match self.get_n_X1(&term, &class) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_X1! Error: {}", error),
        };

        let first_term = (n_11 as f64 / n as f64) * ((n * n_11) as f64 / (n_1X * n_X1) as f64).log2();
        let second_term = (n_10 as f64 / n as f64) * ((n * n_10) as f64 / (n_1X * n_X0) as f64).log2();
        let third_term = (n_01 as f64 / n as f64) * ((n * n_01) as f64 / (n_0X * n_X1) as f64).log2();
        let fourth_term = (n_00 as f64 / n as f64) * ((n * n_00) as f64 / (n_0X * n_X0) as f64).log2();

        let score = first_term + second_term + third_term + fourth_term;

        Ok(score)
    }
}

impl<'a> Classifier<'a> for BayesianClassifier<'a> {
    fn classify(&self, doc_id: u32) -> &'a str {
        "hello"
    }
    fn get_all_vocab(&self) -> HashSet<String> {
        let vocabulary_hamilton = self.index_hamilton.get_vocab();
        let vocabulary_jay = self.index_jay.get_vocab();
        let vocabulary_madison = self.index_madison.get_vocab();

        let first_union: HashSet<_> = vocabulary_hamilton.union(&vocabulary_jay).collect();
        let mut first_union_final: HashSet<String> = HashSet::new();
        for vocab in first_union {
            first_union_final.insert(vocab.clone());
        }

        let second_union: HashSet<_> = first_union_final.union(&vocabulary_madison).collect();
        let mut second_union_final: HashSet<String> = HashSet::new();
        for vocab in second_union {
            second_union_final.insert(vocab.clone());
        }
        
        second_union_final 
    }

}
