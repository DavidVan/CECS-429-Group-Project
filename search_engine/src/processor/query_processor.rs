use index::positional_inverted_index::PositionalInvertedIndex;
use parser::document_parser;
use parser::query_parser::QueryParser;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::*;

/*
 * Processes a query and returns results containing the files fulfilling the query
 *
 * # Arguments
 *
 * *`input` - The query inputted and will be processed
 * *`index` - The Positional Inverted Index that will be used
 * *`id_file` - HashMap containing the associations of the document id and file
 *
 * # Returns
 *
 * The HashSet containing the files fulfilling the query
 */
pub fn process_query(
    input: &str,
    index: &PositionalInvertedIndex,
    id_file: &HashMap<u32, String>,
) -> HashSet<String> {
    let parser = QueryParser::new();
    let processed_query = QueryParser::process_query(&parser, input);
    println!("Processed Query: {:?}", processed_query);

    let mut results: HashSet<String> = HashSet::new();
    let mut or_results = Vec::new();
    let mut and_entries_precursor_string_vec = Vec::new(); // Dirty hack to get around lifetimes...
    for query in processed_query {
        and_entries_precursor_string_vec.clear(); // Need to clear it here and only here...
        // println!("Query For: {}", query);
        let mut and_entries = Vec::new();
        let and_entries_precursor: Vec<&str> = query.split_whitespace().collect();
        for item in and_entries_precursor {
            and_entries_precursor_string_vec.push(String::from(item));
        }
        let mut and_entries_precursor_iter = and_entries_precursor_string_vec.iter();
        let mut entry_builder: Vec<String> = Vec::new();
        while let Some(entry) = and_entries_precursor_iter.next() {
            if entry.starts_with("\"") || entry.starts_with("-\"") {
                let mut modified_entry : String = match entry.starts_with("\"") {
                    true => {
                        let new_query = entry.chars().skip(1).collect();
                        new_query
                    },
                    false => {
                        let mut prefix = String::from("-");
                        let rest_of_query : String = entry.chars().skip(2).collect();
                        prefix.push_str(rest_of_query.as_str());
                        prefix
                    }
                };
                entry_builder.push(modified_entry);
                while let Some(next_entry) = and_entries_precursor_iter.next() {
                    if next_entry.ends_with("\"") {
                        modified_entry = next_entry.chars().take(next_entry.len() - 1).collect();
                        entry_builder.push(modified_entry);
                        and_entries.push(entry_builder.join(" "));
                        entry_builder.clear();
                        break;
                    }
                }
                continue;
            }
            and_entries.push(String::from(entry.clone()));
        }
        // Should check if NEAR/K is in this query... if so, call function to handle... add to and
        // results....
        let mut and_results = Vec::new();
        let mut not_results = Vec::new();

        if query.contains("NEAR/") {
            let near_k_results: Vec<u32> = near_query(query.clone(), index);
            let mut near_k_inner_results = HashSet::new();
            for result in near_k_results {
                let file_path = id_file.get(&result).unwrap().to_string();
                let file: &Path = file_path.as_ref();
                let file_name = file.file_name();
                near_k_inner_results.insert(String::from(file_name.unwrap().to_str().unwrap()));
            }
            and_results.push(near_k_inner_results);
        } else {
            for entry in and_entries {
                // println!("AND ENTRY DAVID {}", entry);
                let not_query = entry.starts_with("-");
                let phrase_literal_vec: Vec<&str> = entry.split_whitespace().collect();
                let phrase_literal = phrase_literal_vec.len() > 1;
                println!("Phrase literal for {}? {}", entry, phrase_literal);
                println!("Not query for {}? {}", entry, not_query);
                if phrase_literal && not_query {
                    println!("Handling phrase literal and not query");
                    // strip out "-" letter... then split whitespace maybe... or not if function
                    // takes a string
                    // call function to get doc id. get file name, add to not list...
                }
                else if phrase_literal && !not_query {
                    // call function to process
                    // read results into and results vec (might have to get file name)
                    let phrase_literal_results : Vec<u32> = Vec::new();
                    let mut phrase_literal_inner_results = HashSet::new();
                    for result in phrase_literal_results {
                        let file_path = id_file.get(&result).unwrap().to_string();
                        let file: &Path = file_path.as_ref();
                        let file_name = file.file_name();
                        phrase_literal_inner_results.insert(String::from(
                            file_name
                                .unwrap()
                                .to_str()
                                .unwrap(),
                        ));
                    }
                    and_results.push(phrase_literal_inner_results);
                // call function to process
                // read results into and results vec (might have to get file name)
                } else {
                    let normalized_tokens = document_parser::normalize_token(entry.to_string());
                    for normalized_token in normalized_tokens {
                        if !index.contains_term(normalized_token.as_str()) {
                            break;
                        }
                        let postings = index.get_postings(normalized_token.as_str());
                        let mut and_inner_results = HashSet::new();
                        for posting in postings {
                            if not_query {
                                let file_path =
                                    id_file.get(&posting.get_doc_id()).unwrap().to_string();
                                let file: &Path = file_path.as_ref();
                                let file_name = file.file_name();
                                not_results.push(String::from(
                                    file_name.unwrap().to_str().unwrap(),
                                ));
                            } else {
                                let file_path =
                                    id_file.get(&posting.get_doc_id()).unwrap().to_string();
                                let file: &Path = file_path.as_ref();
                                let file_name = file.file_name();
                                and_inner_results.insert(String::from(
                                    file_name.unwrap().to_str().unwrap(),
                                ));
                            }
                        }
                        if !not_query {
                            and_results.push(and_inner_results);
                        }
                    }
                }
            }
        }
        // Let's handle the AND logic...
        let mut and_results_iter = and_results.iter();
        let first_and_result = match and_results_iter.next() {
            Some(result) => result,
            None => continue,
        };
        let mut intersection = HashSet::new();
        for item in first_and_result {
            intersection.insert(item.clone());
        }
        while let Some(and_result) = and_results_iter.next() {
            let intersection_result: HashSet<_> =
                and_result.intersection(&intersection).cloned().collect();
            intersection.clear();
            for item in intersection_result {
                intersection.insert(item);
            }
        }
        for not_entry in not_results {
            if intersection.contains(&not_entry) {
                intersection.remove(&not_entry);
            }
        }
        or_results.push(intersection);
    }
    // Let's handle the OR logic...
    let mut or_results_iter = or_results.iter();
    let first_or_result = match or_results_iter.next() {
        Some(result) => result,
        None => {
            return HashSet::new();
        }
    };
    let mut union = HashSet::new();
    for item in first_or_result {
        union.insert(item.clone());
    }
    while let Some(or_result) = or_results_iter.next() {
        let union_result: HashSet<_> = or_result.union(&union).cloned().collect();
        union.clear();
        for item in union_result {
            union.insert(item);
        }
    }
    for x in union {
        results.insert(x);
    }


    results
}

/*
 * Function to process a NEAR/ query 
 *
 * # Arguments
 * *`query_literal` - The query literal in the form "a NEAR/? b"
 * *`index` - The Positional Inverted Index that specifies which a is within x terms of b
 *
 * # Returns
 *
 * The list of files satisfying the query
 */
pub fn near_query(query_literal: String, index: &PositionalInvertedIndex) -> Vec<u32> {
    //extract the terms from the literal
    let literals: Vec<&str> = query_literal.split(' ').collect();
    let first_term = literals[0].clone();
    let mut near = literals[1].clone().to_string();
    let second_term = literals[2].clone();

    near = near.replace("NEAR/", "");
    //extract the maximum distance
    let max_distance = near.parse::<i32>().unwrap();

    println!("first term: {}", first_term);
    let first_term_postings = index.get_postings(&first_term);
    let second_term_postings = index.get_postings(&second_term);
    let mut i = 0;
    let mut j = 0;
    let mut first_positions;
    let mut second_positions;
    let mut documents: Vec<u32> = Vec::new();
    //iterate through postings lists until a common document ID is found
    while i < first_term_postings.len() && j < second_term_postings.len() {
        if first_term_postings[i].get_doc_id() < second_term_postings[j].get_doc_id() {
            i = i + 1;
        } else if first_term_postings[i].get_doc_id() > second_term_postings[j].get_doc_id() {
            j = j + 1;
        } else if first_term_postings[i].get_doc_id() == second_term_postings[j].get_doc_id() {
            //if the two terms have a common document, retrieve the positions
            first_positions = first_term_postings[i].get_positions();
            second_positions = second_term_postings[j].get_positions();
            //check if the two terms are near each other
            if is_near(first_positions, second_positions, max_distance) {
                documents.push(first_term_postings[i].get_doc_id());
            }
            i = i + 1;
            j = j + 1;
        }
    }
    documents
}

/*
 * Function to determine if two terms are within a distance of each other
 *
 * # Arguments
 *
 * *`first_positions` - The positions of the first term within a document
 * *`second_positions` - The positions of the second term within a document
 * *`max_distance` - The maximum distance allowed between the two terms
 *
 * # Returns
 *
 * True if the positions of the first term are within distance of the positions of the second term
 * False otherwise
 */
pub fn is_near(first_positions: Vec<u32>, second_positions: Vec<u32>, max_distance: i32) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut difference: i32;
    //iterate through the positions
    while i < first_positions.len() && j < second_positions.len() {
        difference = (second_positions[j] - first_positions[i]) as i32;
        //if the distance is within the max_distance then we return true
        if difference <= max_distance && difference > 0 {
            return true;
        // if the first position comes before the second then we increment the second position vector
        } else if difference < 0 {
            j = j + 1;
        // if the second position comes more than the threshold after the first one, increment the first position vector
        } else if difference > 0 {
            i = i + 1;
        }
    }
    false
}
