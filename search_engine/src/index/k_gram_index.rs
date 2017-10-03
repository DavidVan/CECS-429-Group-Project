use std::collections::HashMap;

/*
 * Structure that represents KGram Index
 */
pub struct KGramIndex {

    /*
     * HashMap that contains a 'gram' and maps it to terms 
     * containing that gram
     */
    m_index: HashMap<String, Vec<String>>,
}

/*
 * Contains implemented operations of KGramIndex
 */
impl KGramIndex {
    /*
     * Constructs a KGramIndex
     *
     * # Returns
     *
     *  Newly constructed KGramIndex
     */
    pub fn new() -> KGramIndex {
        KGramIndex {
            m_index: HashMap::new(),
        }
    }

    /*
     * Checks a term 
     *
     * # Arguments
     * 
     * *`term` - The term to be checked
     */
    pub fn check_term(&mut self, term: &str) {
        // Appends '$' to beginning and end of term
        let term_copy = format!("${}$", term);
        let mut buffer = [' '; 3];
        let mut counter = 0;
        for c in term_copy.chars() {

            /*
             * Adds new character to buffer and shift all characters
             * to the left, thereby removing existing first character
             */
            if buffer[2] != ' ' {
                buffer[0] = buffer[1].clone();
                buffer[1] = buffer[2].clone();
                buffer[2] = c.clone();
            } else {
                buffer[counter] = c.clone();
            }
            // println!("{:?}", buffer);
            let buffer_string = buffer.iter().cloned().collect::<String>();

            let mut buffer_first_half = buffer_string.clone();
            buffer_first_half.pop();

            let mut buffer_second_half = buffer_string.clone();
            buffer_second_half.remove(0);
            
            let mut buffer_last_char = buffer_string.clone();
            buffer_last_char.remove(0);
            buffer_last_char.remove(0);
            
            let mut buffer_first_char = buffer_string.clone();
            buffer_first_char.pop();
            buffer_first_char.pop();

            if self.m_index.contains_key(&buffer_string) {
                // continue;
            }

            if buffer[2] != ' ' {
                self.add_index(buffer_string.as_str(), term);
                self.add_index(&buffer_last_char, term);
            }
            if buffer[1] != ' ' {
                self.add_index(&buffer_first_half, term);
                self.add_index(&buffer_second_half, term);
            }
            if buffer[0] != ' ' && buffer[0] != '$' {
                self.add_index(&buffer_first_char, term);
            }
            counter = (counter + 1) % buffer.len();
        }
    }

    /*
     * Adds a gram and associates it with a term to the index
     *
     * # Arguments
     *
     * *`gram` - The gram to be added
     * *`term` - The term to be associated with the gram
     */
    fn add_index(&mut self, gram: &str, term: &str) {
        if self.m_index.contains_key(gram) {
            let mut gram_terms = self.m_index.get_mut(gram).expect("Error retrieving gram");
            if !gram_terms.contains(&term.to_string()) {
                gram_terms.push(term.to_string());
            }
        } else {
            let mut terms = Vec::new();
            terms.push(term.to_string());
            self.m_index.insert(gram.to_string(), terms);
        }
    }

    /*
     * Acquires the k_grams of the index
     *
     * # Returns
     * 
     * The K_grams in a vector
     */
    pub fn get_k_grams(&self) -> Vec<&String> {
        let mut k_grams = Vec::new();

        for k_gram in self.m_index.keys() {
            k_grams.push(k_gram);
        }

        return k_grams;
    }
}
