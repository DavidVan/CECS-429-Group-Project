use std::collections::HashMap;

pub struct InvertedIndex {
    pub m_index: HashMap<String, Vec<i32>>,
}

impl InvertedIndex {
    pub fn new(&mut self) {
        self.m_index = HashMap::new();
    }
    pub fn add_term(&mut self, term: &str, doc_id: i32) {
        println!("Adding term {} to {}", term, doc_id);

        if self.m_index.contains_key(term) {
            let mut p = self.m_index.get_mut(term);
            let mut posting = p.as_mut().unwrap();
            if posting[posting.len() - 1] != doc_id {
                posting.push(doc_id);
            }
            println!("Term {} added to {}", term, doc_id);
        } else {
            let mut posting = Vec::new();
            posting.push(doc_id);
            self.m_index.insert(term.to_string(), posting);
            println!("Term {} added to {}", term, doc_id);
        }
    }

    pub fn get_postings(&self, term: &str) -> &Vec<i32> {
        println!("Getting postings for term {}", term);

        self.m_index.get(term).unwrap()

    }

    pub fn get_term_count(&self) -> usize {
        self.m_index.len()
    }

    pub fn get_dictionary(&self) -> Vec<&String> {
        let mut dictionary = Vec::new();

        for term in self.m_index.keys() {
            dictionary.push(term)
        }

        return dictionary;
    }
}
