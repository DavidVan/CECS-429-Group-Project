use std::collections::HashMap;

/*
 * Structure that will contain the Positional Posting of a term in the 
 * Positional Inverted Index
 */
pub struct PositionalPosting {
    /*
     * Document ID of Positional Posting
     */
    mDocID: u32,

    /*
     * The list of positions for each posting
     */
    mPositions: Vec<u32>,
}

/*
 * Contains operations for Positional Posting
 */
impl PositionalPosting {

    /*
     * Constructor of PositionalPosting
     */
    pub fn new(docID: u32) -> PositionalPosting {
        PositionalPosting {
            mDocID: docID,
            mPositions: Vec::new()
        }
    }

    /*
     * Returns document ID clone to preserve data integrity
     */
    pub fn getDocID(&self) -> u32 {
        self.mDocID.clone()
    }

    pub fn getPositions(&self)  -> Vec<u32> {
        self.mPositions.clone() 
    }

    fn addPosition(&mut self, pos: u32) {
        self.mPositions.push(pos);
    }

    fn getLastPosition(&self) -> u32 {
        let pos: u32 = self.mPositions
            .last()
            .expect("Not a valid position")
            .clone();
        pos
    }
}

pub struct PositionalInvertedIndex {
    mIndex: HashMap<String, Vec<PositionalPosting>>,
}

impl PositionalInvertedIndex {
    pub fn new() -> PositionalInvertedIndex {
        PositionalInvertedIndex { mIndex: HashMap::new() }
    }

    pub fn addTerm(&mut self, term: &str, docID: u32, pos: u32) {
        if self.mIndex.contains_key(term) {
            let mut mIndex = &mut self.mIndex;
            {
                let mut positional_postings = mIndex.get_mut(term).expect("No term found");

                let num_of_docIDs = positional_postings.len();
                let mut last_posting = positional_postings.get_mut(num_of_docIDs - 1).expect(
                    "Could not get posting",
                );

                if last_posting.getDocID() == docID {
                    let last_position = last_posting.getLastPosition();
                    if last_position != pos {
                        last_posting.addPosition(pos);
                    }
                }
            }
            {
                if (mIndex.get(term).unwrap().last().unwrap().getDocID() != docID) {
                    let mut new_posting = PositionalPosting::new(docID);
                    new_posting.addPosition(pos);
                    mIndex.get_mut(term).expect("term not found").push(
                        new_posting,
                    );
                }
            }
        } else {
            let mut new_posting = PositionalPosting {
                mDocID: docID,
                mPositions: Vec::new(),
            };
            new_posting.addPosition(pos);
            let mut positional_postings = Vec::new();
            positional_postings.push(new_posting);
            self.mIndex.insert(term.to_string(), positional_postings);
        }
    }

    pub fn contains_term(&self, term: &str) -> bool {
        self.mIndex.contains_key(term)
    }

    pub fn get_postings(&self, term: &str) -> &Vec<PositionalPosting> {
        self.mIndex.get(term).unwrap()
    }

    pub fn get_term_count(&self) -> usize {
        self.mIndex.len()
    }

    pub fn get_dictionary(&self) -> Vec<&String> {
        let mut dictionary = Vec::new();

        for term in self.mIndex.keys() {
            dictionary.push(term)
        }
        dictionary.sort();
        return dictionary;
    }
}
