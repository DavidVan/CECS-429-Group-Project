use std::collections::HashMap;

pub struct InvertedIndex {
    pub mIndex : HashMap<String, Vec<i32>>,
}


impl InvertedIndex {
    pub fn new(&mut self) {
        self.mIndex = HashMap::new();
    }
    pub fn addTerm(&mut self, term : &str, docID : i32)
    {
        println!("Adding term {}{}", term, docID);

        if self.mIndex.contains_key(term)
        {
            let mut p = self.mIndex.get_mut(term);
            let mut posting = p.as_mut().unwrap();
            if !posting[posting.len() - 1] == docID {
                posting.push(docID);
            }
            println!("Term {} added to {}", term, docID);
        }
        else
        {
            let mut posting = Vec::new();
            posting.push(docID);
            self.mIndex.insert(term.to_string(), posting); 
            println!("Term {} added to {}", term, docID);
        }
    }

    pub fn getPostings(&self, term : &str) -> &Vec<i32> {
        println!("Getting postings for term {}", term);

        let posting = self.mIndex.get(term).unwrap();
        return posting;

    }

    pub fn getTermCount(&self) -> usize {
        self.mIndex.len()
    }
}
