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

    m_enable: bool,
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
            m_enable: false,
        }
    }

    /*
     * Checks a term 
     *
     * # Arguments
     * 
     * *`term` - The term to be checked
     */
    pub fn check_term<S: Into<String>>(&mut self, term: S) {

        // Appends '$' to beginning and end of term
        let term_copy : String = format!("${}$", term.into());
        // TODO: iterate i = 0 to length - 3
        for i in 0..(term_copy.len() - 2) {

            let buffer_string : S = term_copy[i..(i+3)];

            
            let buffer_first_half : S = &term_copy[..2];
            let buffer_second_half : S = &term_copy[1..];
            let buffer_first_char : S = &term_copy[0];
            let buffer_mid_char : S = &term_copy[1];
            let buffer_last_char : S = &term_copy[2];

            self.add_index(&buffer_string, term);
            self.add_index(&buffer_first_half, term);
            self.add_index(&buffer_second_half, term);
            self.add_index(&buffer_mid_char, term);

            self.add_index(&buffer_first_char, term);
            self.add_index(&buffer_last_char, term);

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
    fn add_index<S: Into<String>>(&mut self, gram: &str, term: S) {
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

    /*
     * Returns the status of the k_gram_index
     *
     * # Returns
     * 
     * True if K_Gram index is enabled, false otherwise
     */
    pub fn is_enabled(&self) -> bool {
        self.m_enable
    }

    /*
     * Toggles the KGram Index such that it is enabled
     */
    pub fn enable_k_gram(&mut self)  {
        self.m_enable = true; 
    }

    /*
     * Toggles the KGram Index such that it is disabled 
     */
    pub fn disable_k_gram(&mut self)  {
        self.m_enable = false; 
    }
}
