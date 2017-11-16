use std::cmp::Ordering;

pub struct DocumentAccumulator {
   m_doc_id: u32, 
   m_accumulator: f64,
}

impl DocumentAccumulator {
    pub fn new(doc_id: u32, accumulator: f64) -> DocumentAccumulator {
        DocumentAccumulator {
            m_doc_id: doc_id,
            m_accumulator: accumulator
        }
    }
    pub fn get_doc_id(&self) -> u32 {
        self.m_doc_id 
    }

    pub fn get_accumulator(&self) -> f64 {
        self.m_accumulator 
    }
}

impl Ord for DocumentAccumulator {
    
    fn cmp(&self, other: &DocumentAccumulator) -> Ordering {
       self.m_accumulator.partial_cmp(&other.m_accumulator).unwrap()
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
        self.m_accumulator == other.m_accumulator 
    }
}
