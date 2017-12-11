use std::time::{Duration, Instant};
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
            println!("Looks like there was a NaN! Term is: {}. Class is: {:?}.", term, class);
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
        println!("Length of all vocabulary: {}", all_vocabulary.len());

        let mut priority_queue: BinaryHeap<TermClassScore> = BinaryHeap::new();
        let time = Instant::now();
        for term in &all_vocabulary {
            match self.calculate_mutual_information_score(term.clone()) {
                Ok(score) => {
                    let (score_hamilton, score_jay, score_madison) = score;
                    match TermClassScore::new(score_hamilton, term.clone(), DocumentClass::Hamilton) {
                        Some(score) => {
                            priority_queue.push(score);
                        },
                        None => {
                            // Do nothing.
                        },
                    };
                    match TermClassScore::new(score_jay, term.clone(), DocumentClass::Jay) {
                        Some(score) => {
                            priority_queue.push(score);
                        },
                        None => {
                            // Do nothing.
                        },
                    };
                    match TermClassScore::new(score_madison, term.clone(), DocumentClass::Madison) {
                        Some(score) => {
                            priority_queue.push(score);
                        },
                        None => {
                            // Do nothing.
                        },
                    };
                },
                Err(error) => panic!("There was an error calculating the score for term {}. The error is: {}", term, error),
            };
        }
        println!("Time taken to build discriminating vocab: {} seconds. Total number of things in priority_queue: {}", time.elapsed().as_secs(), priority_queue.len());
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

    fn get_n_00(&self, term: &str) -> Result<(u32, u32, u32), &'static str> { // N00, total number of documents that DO NOT contain term t and NOT in class c.
        let hamilton_total_num = self.index_hamilton.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let jay_total_num = self.index_jay.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let madison_total_num = self.index_madison.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 

        let hamilton_doc_freq_for_term = self.index_hamilton.get_document_frequency(term);

        let jay_doc_freq_for_term = self.index_jay.get_document_frequency(term);

        let madison_doc_freq_for_term = self.index_madison.get_document_frequency(term);

        let hamilton_num_without_term = hamilton_total_num - hamilton_doc_freq_for_term; 
        let jay_num_without_term = jay_total_num - jay_doc_freq_for_term;
        let madison_num_without_term = madison_total_num - madison_doc_freq_for_term;

        let n_00_hamilton = jay_num_without_term + madison_num_without_term;
        let n_00_jay = hamilton_num_without_term + madison_num_without_term;
        let n_00_madison = hamilton_num_without_term + jay_num_without_term;

        let n_00 = (n_00_hamilton, n_00_jay, n_00_madison);

        Ok(n_00)
    }

    fn get_n_01(&self, term: &str) -> Result<(u32, u32, u32), &'static str> { // N01, total number of documents that DO contain term t and NOT in class c.
        let hamilton_doc_freq_for_term = self.index_hamilton.get_document_frequency(term);

        let jay_doc_freq_for_term = self.index_jay.get_document_frequency(term);

        let madison_doc_freq_for_term = self.index_madison.get_document_frequency(term);

        let n_01_hamilton = jay_doc_freq_for_term + madison_doc_freq_for_term;
        let n_01_jay = hamilton_doc_freq_for_term + madison_doc_freq_for_term;
        let n_01_madison = madison_doc_freq_for_term + jay_doc_freq_for_term;

        let n_01 = (n_01_hamilton, n_01_jay, n_01_madison);

        Ok(n_01)
    }

    fn get_n_10(&self, term: &str) -> Result<(u32, u32, u32), &'static str> { // N10, total number of documents that DO NOT contain term t but IS in class c.
        let hamilton_total_num = self.index_hamilton.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let jay_total_num = self.index_jay.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 
        let madison_total_num = self.index_madison.get_num_documents().expect("No Documents found!"); // Nc, total number of documents for class. 

        let hamilton_doc_freq_for_term = self.index_hamilton.get_document_frequency(term);

        let jay_doc_freq_for_term = self.index_jay.get_document_frequency(term);

        let madison_doc_freq_for_term = self.index_madison.get_document_frequency(term);

        let hamilton_num_without_term = hamilton_total_num - hamilton_doc_freq_for_term; 
        let jay_num_without_term = jay_total_num - jay_doc_freq_for_term;
        let madison_num_without_term = madison_total_num - madison_doc_freq_for_term;

        let n_10_hamilton = hamilton_num_without_term;
        let n_10_jay = jay_num_without_term;
        let n_10_madison = madison_num_without_term;

        let n_10 = (n_10_hamilton, n_10_jay, n_10_madison);

        Ok(n_10)
    }

    fn get_n_11(&self, term: &str) -> Result<(u32, u32, u32), &'static str> { // N11, total number of documents that DO contain term t and IS in class c.
        let hamilton_doc_freq_for_term = self.index_hamilton.get_document_frequency(term);

        let jay_doc_freq_for_term = self.index_jay.get_document_frequency(term);

        let madison_doc_freq_for_term = self.index_madison.get_document_frequency(term);

        let n_11_hamilton = hamilton_doc_freq_for_term;
        let n_11_jay = jay_doc_freq_for_term;
        let n_11_madison = madison_doc_freq_for_term;

        let n_11 = (n_11_hamilton, n_11_jay, n_11_madison);

        Ok(n_11)
    }

    fn get_n_0X(&self, term: &str) -> Result<(u32, u32, u32, (u32, u32, u32), (u32, u32, u32)), &'static str> { // N0X, N00 + N01
        let n_00 = match self.get_n_00(term) {
            Ok(n_00) => n_00,
            Err(_) => panic!("Something happened when calculating N00!"),
        };
        let n_01 = match self.get_n_01(term) {
            Ok(n_01) => n_01,
            Err(_) => panic!("Something happened when calculating N01!"),
        };

        let (n_00_hamilton, n_00_jay, n_00_madison) = n_00;
        let (n_01_hamilton, n_01_jay, n_01_madison) = n_01;

        let n_0X_hamilton = n_00_hamilton + n_01_hamilton;
        let n_0X_jay = n_00_jay + n_01_jay;
        let n_0X_madison = n_00_madison + n_01_madison;

        let n_0X = (n_0X_hamilton, n_0X_jay, n_0X_madison, n_00, n_01);

        Ok(n_0X)
    }

    fn get_n_X0(&self, term: &str) -> Result<(u32, u32, u32, (u32, u32, u32), (u32, u32, u32)), &'static str> { // NX0, N00 + N10
        let n_00 = match self.get_n_00(term) {
            Ok(n_00) => n_00,
            Err(_) => panic!("Something happened when calculating N00!"),
        };
        let n_10 = match self.get_n_10(term) {
            Ok(n_10) => n_10,
            Err(_) => panic!("Something happened when calculating N10!"),
        };

        let (n_00_hamilton, n_00_jay, n_00_madison) = n_00;
        let (n_10_hamilton, n_10_jay, n_10_madison) = n_10;

        let n_X0_hamilton = n_00_hamilton + n_10_hamilton;
        let n_X0_jay = n_00_jay + n_10_jay;
        let n_X0_madison = n_00_madison + n_10_madison;

        let n_X0 = (n_X0_hamilton, n_X0_jay, n_X0_madison, n_00, n_10);

        Ok(n_X0)
    }

    fn get_n_1X(&self, term: &str) -> Result<(u32, u32, u32, (u32, u32, u32), (u32, u32, u32)), &'static str> { // N1X, N10 + N11
        let n_10 = match self.get_n_10(term) {
            Ok(n_10) => n_10,
            Err(_) => panic!("Something happened when calculating N10!"),
        };
        let n_11 = match self.get_n_11(term) {
            Ok(n_11) => n_11,
            Err(_) => panic!("Something happened when calculating N11!"),
        };

        let (n_10_hamilton, n_10_jay, n_10_madison) = n_10;
        let (n_11_hamilton, n_11_jay, n_11_madison) = n_11;

        let n_1X_hamilton = n_10_hamilton + n_11_hamilton;
        let n_1X_jay = n_10_jay + n_11_jay;
        let n_1X_madison = n_10_madison + n_11_madison;

        let n_1X = (n_1X_hamilton, n_1X_jay, n_1X_madison, n_10, n_11);

        Ok(n_1X)
    }

    fn get_n_X1(&self, term: &str) -> Result<(u32, u32, u32, (u32, u32, u32), (u32, u32, u32)), &'static str> { // NX1, N01 + N11
        let n_01 = match self.get_n_01(term) {
            Ok(n_01) => n_01,
            Err(_) => panic!("Something happened when calculating N01!"),
        };
        let n_11 = match self.get_n_11(term) {
            Ok(n_11) => n_11,
            Err(_) => panic!("Something happened when calculating N11!"),
        };

        let (n_01_hamilton, n_01_jay, n_01_madison) = n_01;
        let (n_11_hamilton, n_11_jay, n_11_madison) = n_11;

        let n_X1_hamilton = n_01_hamilton + n_11_hamilton;
        let n_X1_jay = n_01_jay + n_11_jay;
        let n_X1_madison = n_01_madison + n_11_madison;

        let n_X1 = (n_X1_hamilton, n_X1_jay, n_X1_madison, n_01, n_11);

        Ok(n_X1)
    }

    fn calculate_mutual_information_score(&self, term: String) -> Result<(f64, f64, f64), &'static str> {
        let n_hamilton = self.index_hamilton.get_num_documents().expect("No Documents found!"); 
        let n_jay = self.index_jay.get_num_documents().expect("No Documents found!"); 
        let n_madison = self.index_madison.get_num_documents().expect("No Documents found!"); 
        let n = n_hamilton + n_jay + n_madison;

        let (n_0X_hamilton, n_0X_jay, n_0X_madison, n_00, n_01) = match self.get_n_0X(&term) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_0X! Error: {}", error),
        };
        let (n_X0_hamilton, n_X0_jay, n_X0_madison, _, n_10) = match self.get_n_X0(&term) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_X0! Error: {}", error),
        };
        let (n_1X_hamilton, n_1X_jay, n_1X_madison, _, n_11) = match self.get_n_1X(&term) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_1X! Error: {}", error),
        };
        let (n_X1_hamilton, n_X1_jay, n_X1_madison, _, _) = match self.get_n_X1(&term) {
            Ok(value) => value,
            Err(error) => panic!("Something happened when calculating N_X1! Error: {}", error),
        };

        let (n_00_hamilton, n_00_jay, n_00_madison) = n_00;
        let (n_01_hamilton, n_01_jay, n_01_madison) = n_01;
        let (n_10_hamilton, n_10_jay, n_10_madison) = n_10;
        let (n_11_hamilton, n_11_jay, n_11_madison) = n_11;
        
        // Debug Purposes
        //
        // let n_0X = (n_0X_hamilton, n_0X_jay, n_0X_madison);
        // let n_X0 = (n_X0_hamilton, n_X0_jay, n_X0_madison);
        // let n_1X = (n_1X_hamilton, n_1X_jay, n_1X_madison);
        // let n_X1 = (n_X1_hamilton, n_X1_jay, n_X1_madison);
        //
        // println!("N: {:?}", n);
        // println!("N00: {:?}", n_00);
        // println!("N01: {:?}", n_01);
        // println!("N10: {:?}", n_10);
        // println!("N11: {:?}", n_11);
        // println!("N0X: {:?}", n_0X);
        // println!("NX0: {:?}", n_X0);
        // println!("N1X: {:?}", n_1X);
        // println!("NX1: {:?}", n_X1);
        //
        /////////////////

        let first_term_hamilton = (n_11_hamilton as f64 / n as f64) * ((n * n_11_hamilton) as f64 / (n_1X_hamilton * n_X1_hamilton) as f64).log2();
        let second_term_hamilton = (n_10_hamilton as f64 / n as f64) * ((n * n_10_hamilton) as f64 / (n_1X_hamilton * n_X0_hamilton) as f64).log2();
        let third_term_hamilton = (n_01_hamilton as f64 / n as f64) * ((n * n_01_hamilton) as f64 / (n_0X_hamilton * n_X1_hamilton) as f64).log2();
        let fourth_term_hamilton = (n_00_hamilton as f64 / n as f64) * ((n * n_00_hamilton) as f64 / (n_0X_hamilton * n_X0_hamilton) as f64).log2();

        let first_term_jay = (n_11_jay as f64 / n as f64) * ((n * n_11_jay) as f64 / (n_1X_jay * n_X1_jay) as f64).log2();
        let second_term_jay = (n_10_jay as f64 / n as f64) * ((n * n_10_jay) as f64 / (n_1X_jay * n_X0_jay) as f64).log2();
        let third_term_jay = (n_01_jay as f64 / n as f64) * ((n * n_01_jay) as f64 / (n_0X_jay * n_X1_jay) as f64).log2();
        let fourth_term_jay = (n_00_jay as f64 / n as f64) * ((n * n_00_jay) as f64 / (n_0X_jay * n_X0_jay) as f64).log2();

        let first_term_madison = (n_11_madison as f64 / n as f64) * ((n * n_11_madison) as f64 / (n_1X_madison * n_X1_madison) as f64).log2();
        let second_term_madison = (n_10_madison as f64 / n as f64) * ((n * n_10_madison) as f64 / (n_1X_madison * n_X0_madison) as f64).log2();
        let third_term_madison = (n_01_madison as f64 / n as f64) * ((n * n_01_madison) as f64 / (n_0X_madison * n_X1_madison) as f64).log2();
        let fourth_term_madison = (n_00_madison as f64 / n as f64) * ((n * n_00_madison) as f64 / (n_0X_madison * n_X0_madison) as f64).log2();

        let score_hamilton = first_term_hamilton + second_term_hamilton + third_term_hamilton + fourth_term_hamilton;
        let score_jay = first_term_jay + second_term_jay + third_term_jay + fourth_term_jay;
        let score_madison = first_term_madison + second_term_madison + third_term_madison + fourth_term_madison;

        let score = (score_hamilton, score_jay, score_madison);

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
