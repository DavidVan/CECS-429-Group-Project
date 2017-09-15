use std::collections::HashMap;

pub struct InvertedIndex {
    pub mIndex : HashMap<String, i32>,
}

impl InvertedIndex {
    pub fn new(&mut self) {
        self.mIndex = HashMap::new(); 
    }
    pub fn addTerm(&self, term : &str, docID : i32)
    {
        println!("Adding term {}{}", term, docID); 

        if (self.mIndex.contains_key(term))
        {
            if (!self.mIndex.get(term).contains(docID)) {
                self.mIndex.get(term).push(docID); 
            } 
        }
        else
        {
             
        }
    }

    pub fn getPostings(&self, term : &str) -> Vec<i32> {
        println!("Getting postings for term {}", term);

        let x = vec![1,2,3];
        return x;
    }

    pub fn getTermCount(&self) -> usize {
        self.mIndex.len() 
    }
}
