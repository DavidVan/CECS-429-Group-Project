use std::collections::HashMap;
use std::collections::HashSet;

pub struct KGramIndex {
    mIndex: HashMap<String, Vec<String>>,
    mSet: HashSet<String>,
}

impl KGramIndex {
    pub fn new() -> KGramIndex {
        KGramIndex { mIndex: HashMap::new(), mSet: HashSet::new() }
    }

    pub fn checkIndex(&mut self, term: &str) {
        if !self.mSet.contains(term) {
            self.mSet.insert(term.to_string()); 
            let term_copy = format!("${}$", term);
            let mut buffer = [' '; 3];
            let mut counter = 0;
            for c in term_copy.chars() {
                if buffer[2] != ' ' {
                    buffer[0] = buffer[1].clone();
                    buffer[1] = buffer[2].clone();
                    buffer[2] = c.clone();
                } else {
                    buffer[counter] = c.clone();
                }
                let buffer_string = buffer.iter().cloned().collect::<String>();
                let buffer_first_half = &buffer[0..2].iter().cloned().collect::<String>();
                let buffer_second_half = &buffer[1..3].iter().cloned().collect::<String>();
                let buffer_first_char = buffer[counter].clone().to_string();

                if buffer[2] != ' ' {
                    self.addIndex(buffer_string.as_str(), term);
                }
                if buffer[1] != ' ' {
                    self.addIndex(buffer_first_half.trim(), term);
                    self.addIndex(buffer_second_half.trim(), term);
                }
                if buffer[counter] != ' ' && buffer[counter] != '$' {
                    self.addIndex(buffer_first_char.as_str(), term);
                }
                counter = (counter + 1) % buffer.len();
            }
        }
    }

    fn addIndex(&mut self, gram: &str, term: &str) {
        if self.mIndex.contains_key(gram) {
            let mut gram_terms = self.mIndex.get_mut(gram).expect("Error retrieving gram");
            if (!gram_terms.contains(&term.to_string())) {
                gram_terms.push(term.to_string());
            }
        } else {
            let mut terms = Vec::new();
            terms.push(term.to_string());
            self.mIndex.insert(gram.to_string(), terms);
        }
    }

    pub fn getKGrams(&self) -> Vec<&String> {
        let mut k_grams = Vec::new();

        for k_gram in self.mIndex.keys() {
            k_grams.push(k_gram);
        }

        return k_grams;
    }
}
