use index::positional_inverted_index::PositionalPosting;
use index::disk_inverted_index::DiskInvertedIndex;
use index::disk_inverted_index::IndexReader;
use index::k_gram_index::KGramIndex;
use parser::document_parser;
use parser::query_parser::QueryParser;
use processor::document_accumulator::DocumentAccumulator;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::path::*;

pub fn rocchio_calculation_for_class(index: &DiskInvertedIndex) -> f64 {

    


    return 0.0f64;
}

fn calculate_normalized_vector_for_document() ->f64 {
    return 0.0f64;
}