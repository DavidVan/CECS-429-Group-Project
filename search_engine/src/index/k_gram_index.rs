use std::collections::HashMap;

pub struct KGramIndex {
    mIndex: HashMap<String, Vec<String>>,
}

impl KGramIndex {
    pub fn new() -> KGramIndex {
        KGramIndex { mIndex: HashMap::new() }
    }

    pub fn checkIndex(&mut self, term: &str) {
        let term_copy = format!("${}$", term);
        let mut buffer = [' '; 3];
        let mut counter = 0;
        println!("{:?}", buffer);
        for c in term_copy.chars() {
            println!("Counter {}", counter);
            println!("Character {}", c.to_string());
            if buffer[2] != ' ' {
                buffer[0] = buffer[1].clone();
                buffer[1] = buffer[2].clone();
                buffer[2] = c.clone();
            } else {
                buffer[counter] = c.clone();
            }
            println!("{:?}", buffer);
            if buffer[2] != ' ' {
                let k_gram = buffer.iter().cloned().collect::<String>();
                self.addIndex(k_gram.as_str(), term);
            }
            if buffer[1] != ' ' {
                let first_half = &buffer[0..2].iter().cloned().collect::<String>();
                let second_half = &buffer[1..3].iter().cloned().collect::<String>();
                self.addIndex(first_half.trim(), term);
                self.addIndex(second_half.trim(), term);
            }
            if buffer[counter] != ' ' && buffer[counter] != '$' {
                self.addIndex(buffer[counter].clone().to_string().as_str(), term);
            }
            counter = (counter + 1) % buffer.len();
        }

    }

    fn addIndex(&mut self, gram: &str, term: &str) {
        if self.mIndex.contains_key(gram) {
            let mut gram_terms = self.mIndex.get_mut(gram).expect("Error retrieving gram");
            if (!gram_terms.contains(&term.to_string())) {
                gram_terms.push(term.to_string());
            }
        } else {
            println!("Inserting {} to k gram", gram);
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
