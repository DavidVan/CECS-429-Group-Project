use std::cmp::Ordering;

pub struct DocumentAccumulator {
   doc_id: u64, 
   accumulator: f64,
}

impl DocumentAccumulator {
    pub fn new(docId: u64, acc: f64) -> DocumentAccumulator {
        DocumentAccumulator {
            doc_id: docId,
            accumulator: acc
        }
    }
    pub fn get_doc_id(&self) -> u64 {
        self.doc_id 
    }

    pub fn get_accumulator(&self) -> f64 {
        self.accumulator 
    }
}

impl Ord for DocumentAccumulator {
    
    fn cmp(&self, other: &DocumentAccumulator) -> Ordering {
       self.accumulator.partial_cmp(&other.accumulator).unwrap()
    }
}

impl Eq for DocumentAccumulator {

}

impl PartialOrd for DocumentAccumulator {
    fn partial_cmp(&self, other: &DocumentAccumulator) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DocumentAccumulator {
    fn eq(&self, other: &DocumentAccumulator) -> bool {
        self.accumulator == other.accumulator 
    }
}
