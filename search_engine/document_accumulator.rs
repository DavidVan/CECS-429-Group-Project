use std::cmp::Ordering;

pub struct DocumentAccumulator {
   doc_id: u64, 
   accumulator: f64,
}

impl DocumentAccumulator {
    pub fn get_doc_id() -> u64 {
        doc_id 
    }

    pub fn get_accumulator() -> f64 {
        accumulator 
    }
}

impl Ord for DocumentAccumulator {
    fn cmp(&self, other: &DocumentAccumulator) -> Ordering {
       self.accumulator.cmp(other.accumulator) 
    }
}

impl PartialOrd for DocumentAccumulator {
    fn partial_cmp(&self, other: &DocumentAccumulator) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
