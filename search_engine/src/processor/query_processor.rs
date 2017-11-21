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

pub fn process_query(
    ranked_retrieval: bool,
    scheme: &str,
    input: &str,
    index: &DiskInvertedIndex,
    kgram: &KGramIndex,
    id_file: &HashMap<u32, String>,
) -> HashSet<String> {

    if ranked_retrieval {
        return process_query_rank(scheme, input, index, kgram, id_file);
    } else { 
        return process_query_bool(input,  index, kgram, id_file); 
    }

}


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
pub fn process_query_bool(
    input: &str,
    index: &DiskInvertedIndex,
    kgram: &KGramIndex,
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
        println!("Query For: {}", query);
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
                    entry_builder.push(next_entry.to_string());
                }
                continue;
            }
            and_entries.push(String::from(entry.clone()));
        }
        // Should check if NEAR/K is in this query... if so, call function to handle... add to and
        // results....
        let mut and_results = Vec::new();
        let mut not_results = Vec::new();

        let mut new_and_entries : Vec<String> = Vec::new();

        if kgram.is_enabled() {
            for entry in and_entries {
                if entry.contains("*") {
                    let mut results = get_wildcards(&entry, kgram);
                    new_and_entries.append(&mut results);
                } else {
                    println!("NOT WILDCARD: {}", entry);
                    new_and_entries.push(entry); 
                }
            }
        } else {
            for entry in and_entries {
                new_and_entries.push(entry);
            } 
        }

        println!("Full Query: {:?}", new_and_entries);

        if query.contains("NEAR/") {
            let near_k_results: Vec<u32> = near_query(query.clone(), index);
            let mut near_k_inner_results = HashSet::new();
            for result in near_k_results {
                let file_path = id_file.get(&result).unwrap().to_string();
                let file: &Path = file_path.as_ref();
                let file_name = file.file_name();
                near_k_inner_results.insert(String::from(file_name.unwrap().to_str().unwrap()));
            }
            if near_k_inner_results.len() != 0 {
                and_results.push(near_k_inner_results);
            }
        } else {
            for entry in new_and_entries {
                println!("AND ENTRY DAVID {}", entry);
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
                    let phrase : String = entry.chars().skip(1).collect();
                    let results_to_remove: Vec<u32> = phrase_query(phrase, index);
                    for doc_id in results_to_remove {
                        let file_path =
                            id_file.get(&doc_id).unwrap().to_string();
                        let file: &Path = file_path.as_ref();
                        let file_name = file.file_name();
                        not_results.push(String::from(
                            file_name.unwrap().to_str().unwrap(),
                        ));
                    }
                }
                else if phrase_literal && !not_query {
                    // call function to process
                    // read results into and results vec (might have to get file name)
                    let phrase_literal_results: Vec<u32> = phrase_query(entry.clone(), &index);
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
                }
                else {
                    let normalized_tokens = document_parser::normalize_token(entry.to_string());
                    let stemmed_tokens = document_parser::stem_terms(normalized_tokens);
                    for stemmed_token in stemmed_tokens {
                        if !index.contains_term(stemmed_token.as_str()) {
                            println!("Breaking because index does not contain term: {}", stemmed_token);
                            break;
                        }
                        let postings = index.get_postings(stemmed_token.as_str()).expect("Failed to get postings");
                        let mut and_inner_results = HashSet::new();
                        for posting in postings {
                            let doc_id = posting.0;
                            if not_query {
                                let file_path =
                                    id_file.get(&doc_id).unwrap().to_string();
                                let file: &Path = file_path.as_ref();
                                let file_name = file.file_name();
                                not_results.push(String::from(
                                    file_name.unwrap().to_str().unwrap(),
                                ));
                            } else {
                                let file_path =
                                    id_file.get(&doc_id).unwrap().to_string();
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

fn process_query_rank(
    scheme: &str,
    input: &str,
    index: &DiskInvertedIndex,
    kgram: &KGramIndex,
    id_file: &HashMap<u32, String>,
) -> HashSet<String> {

    let parser = QueryParser::new();
    let processed_query = QueryParser::process_query(&parser, input);
    println!("Processed Query: {:?}", processed_query);

    if processed_query.len() > 1 {
        println!("Invalid Query"); 
        return HashSet::new();
    }
    let mut and_entries_precursor_string_vec = Vec::new(); // Dirty hack to get around lifetimes...
    let results: HashSet<String> = HashSet::new();
    for query in processed_query {
        and_entries_precursor_string_vec.clear(); // Need to clear it here and only here...
        println!("Query For: {}", query);
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
                    entry_builder.push(next_entry.to_string());
                }
                continue; // Why?
            }
            and_entries.push(String::from(entry.clone()));
        }
        // Should check if NEAR/K is in this query... if so, call function to handle... add to and
        // results....

        let mut new_and_entries : Vec<String> = Vec::new();

        if kgram.is_enabled() {
            for entry in and_entries {
                if entry.contains("*") {
                    let mut results = get_wildcards(&entry, kgram);
                    new_and_entries.append(&mut results);
                } else {
                    println!("NOT WILDCARD: {}", entry);
                    new_and_entries.push(entry); 
                }
            }
        } else {
            for entry in and_entries {
                new_and_entries.push(entry);
            } 
        }

        println!("Full Query: {:?}", new_and_entries);

        let mut accumulators : BinaryHeap<DocumentAccumulator> = BinaryHeap::new(); 

        let mut doc_accs : HashMap <u32, f64> = HashMap::new();
        let mut doc_lds : HashMap <u32, f64> = HashMap::new();

        let number_of_docs = id_file.len();
        println!("Number of docs: {}" , number_of_docs);
        for entry in new_and_entries {
            let normalized_tokens = document_parser::normalize_token(entry.to_string());  
            for normalized_token in normalized_tokens {
                let wqt = get_wqt(scheme, number_of_docs as u32, &normalized_token, index);
                let postings = index.get_postings_no_positions(&normalized_token).expect("Failed to get postings");
                for posting in postings {
                    let doc_id = posting.0;
                    let term_doc_frequency = posting.1;
                    let wdt = get_wdt(scheme, doc_id, &normalized_token, term_doc_frequency, index);
                    let accumulator : f64 = wqt * wdt;
                    if doc_accs.contains_key(&doc_id) {
                        *doc_accs.get_mut(&doc_id).unwrap() += accumulator;
                    } else {
                        doc_accs.insert(doc_id, accumulator); 
                    }
                    if doc_lds.contains_key(&doc_id) {
                        if scheme == "tfidf" {
                            *doc_lds.get_mut(&doc_id).unwrap() += get_ld(scheme, doc_id, &normalized_token, term_doc_frequency, index); 
                        }
                    } else {
                        let ld = get_ld(scheme, doc_id, &normalized_token, term_doc_frequency, index);
                        doc_lds.insert(doc_id, ld); 
                    }
                }
            }
        }

        for (doc, acc) in doc_accs {
            if acc > 0.0 {
                let new_acc = (acc)/(doc_lds.get(&doc).unwrap());
                let new_doc_acc : DocumentAccumulator = DocumentAccumulator::new(doc, new_acc); 
                accumulators.push(new_doc_acc);
            }
        }

        let mut counter = 0;

        while !accumulators.is_empty() && counter != 10 {
            let doc_acc = accumulators.pop().unwrap();
            let file_path = id_file.get(&(doc_acc.get_doc_id() as u32)).unwrap().to_string();
            let file: &Path = file_path.as_ref();
            let file_name = file.file_name().unwrap().to_str().unwrap();
            println!("{} - {}", file_name, doc_acc.get_accumulator());
            counter += 1;
        }
    }

    return results;

}


fn get_wqt(scheme: &str, number_of_docs: u32, token: &str, index: &DiskInvertedIndex ) -> f64 {
    if scheme == "default" {
        return ((1 + ((number_of_docs as u32)/index.get_document_frequency(&token))) as f64).ln();
    } else if scheme == "tfidf" {
        return (((number_of_docs)/index.get_document_frequency(&token)) as f64).ln();
    } else if scheme == "okapi" {
        return (0.1 as f64).max((((((number_of_docs - index.get_document_frequency(&token)) as f64) + 0.5)/((index.get_document_frequency(&token) as f64) + 0.5) as f64) as f64).ln());
    } else if scheme == "wacky" {
        return (0.0 as f64).max(((((number_of_docs - index.get_document_frequency(&token)) as f64)/(index.get_document_frequency(&token) as f64)) as f64).ln());
    } else {
        return 1.0; 
    }
}


fn get_wdt(scheme: &str, doc_id: u32, token: &str, term_doc_frequency: u32, index: &DiskInvertedIndex) -> f64 {
    if scheme == "default" {
        return 1.0 + (term_doc_frequency as f64).ln();
    } else if scheme == "tfidf" {
        return term_doc_frequency as f64;
    } else if scheme == "okapi" {
        return 2.2 * term_doc_frequency as f64;
    } else if scheme == "wacky" {
        let doc_weights = index.get_document_weights(doc_id).unwrap();
        let tftd_a = doc_weights.4;

        return (1.0 + (term_doc_frequency as f64).ln())/(1.0 + (tftd_a).ln());
    } else {
        return 1.0;
    }
}

fn get_ld(scheme: &str, doc_id: u32, token: &str, term_doc_frequency: u32, index:&DiskInvertedIndex) -> f64 {
    let doc_weights = index.get_document_weights(doc_id).unwrap();
    let doc_weight = doc_weights.1;
    let doc_length_a = doc_weights.0;
    let doc_length = doc_weights.2;
    let byte_size = doc_weights.3;
    if scheme == "default" {
        return doc_weight;
    } else if scheme == "tfidf" {
        return doc_weight;
    } else if scheme == "okapi" {
        return 1.2 * (0.25 + (0.75 * (doc_length as f64)/(doc_length_a as f64) + term_doc_frequency as f64));
    } else {
        return (byte_size as f64).sqrt();
    }
}


pub fn get_wildcards(entry: &str, kgram: &KGramIndex) -> Vec<String> {
    println!("WILDCARD: {}", entry);
    let mut results: Vec<String> = Vec::new();
    if entry.starts_with("*") {
        let mut batch_one: Vec<String> = Vec::new();
        let mut batch_two: Vec<String> = Vec::new();
        let mut final_batch : Vec<String> = Vec::new();

        // println!("Checking Batch One");
        let slice = &entry[1..];
        let mid = &entry[1..entry.len() - 1];
        let big_gram = format!("{}{}", &entry[1..], "$");
        for i in 0..(big_gram.len()) {
            if i < big_gram.len() - 2 {
                let three_gram = &big_gram[i..(i + 3)];
                // println!("Gram: {}", three_gram);
                if !three_gram.contains("*") {
                    let terms = kgram.get_terms(three_gram);
                    for term in terms {
                        // println!("Term: {}", term);
                        if !batch_one.contains(term) &&
                            (term.ends_with(slice) ||
                             term.contains(mid)) {
                            batch_one.push(term.to_string()); 
                            // println!("SUCCESS");
                        }
                    }
                }
            }
            if i < big_gram.len() - 1 {
                let two_gram = &big_gram[i..(i + 2)];
                // println!("Gram: {}", two_gram);
                if !two_gram.contains("*") {
                    let terms = kgram.get_terms(two_gram);
                    for term in terms {
                        // println!("Term: {}", term);
                        if !batch_one.contains(term) &&
                            (term.ends_with(slice) ||
                             term.contains(mid)) {
                            batch_one.push(term.to_string()); 
                            // println!("SUCCESS");
                        }
                    }
                }
            }
            // println!("Batch One: {:?}", batch_one);
        }
        // println!("Checking Batch Two");
        if entry.ends_with("*") {
            let big_gram = format!("{}{}", "$", &entry[..entry.len() - 1]);
            for i in 0..(big_gram.len()) {
                if i < big_gram.len() - 2 {
                    let three_gram = &big_gram[i..(i + 3)];
                    // println!("Gram: {}", three_gram);
                    if !three_gram.contains("*") {
                        let terms = kgram.get_terms(three_gram);
                        for term in terms {
                            // println!("Term: {}", term);
                            if !batch_two.contains(term) &&
                                term.contains(mid) {
                                batch_two.push(term.to_string()); 
                                // println!("SUCCESS");
                            }
                        }
                    }
                }
                if i < big_gram.len() - 1 {
                    let two_gram = &big_gram[i..(i + 2)];
                    // println!("Gram: {}", two_gram);
                    if !two_gram.contains("*") {
                        let terms = kgram.get_terms(two_gram);
                        for term in terms {
                            // println!("Term: {}", term);
                            if !batch_two.contains(term) &&
                                term.contains(mid) {
                                batch_two.push(term.to_string()); 
                                // println!("SUCCESS");
                            }
                        }
                    }
                }
                // println!("Batch Two: {:?}", batch_two);
            }
        } else if entry.contains("*") {
        
        }
        if batch_two.is_empty() {
            final_batch.append(&mut batch_one); 
        } else {
            final_batch = intersection(batch_one, batch_two);
        }
        results.append(&mut final_batch);
    } else if entry.ends_with("*") {
        let mut batch_one: Vec<String> = Vec::new();
        let batch_two: Vec<String> = Vec::new();
        let mut final_batch : Vec<String> = Vec::new();
        let slice = &entry[..entry.len() - 1];
        let big_gram = format!("{}{}", "$", &entry[..entry.len() - 1]);
        for i in 0..(big_gram.len()) {
            if i < big_gram.len() - 2 {
                let three_gram = &big_gram[i..(i + 3)];
                // println!("Gram: {}", three_gram);
                if !three_gram.contains("*") {
                    let terms = kgram.get_terms(three_gram);
                    for term in terms {
                        // println!("Term: {}", term);
                        if !batch_one.contains(term) &&
                            term.starts_with(slice) {
                            batch_one.push(term.to_string()); 
                            // println!("SUCCESS");
                        }
                    }
                }
            }
            if i < big_gram.len() - 1 {
                let two_gram = &big_gram[i..(i + 2)];
                // println!("Gram: {}", two_gram);
                if !two_gram.contains("*") {
                    let terms = kgram.get_terms(two_gram);
                    for term in terms {
                        // println!("Term: {}", term);
                        if !batch_one.contains(term) &&
                            term.starts_with(slice) {
                            batch_one.push(term.to_string()); 
                            // println!("SUCCESS");
                        }
                    }
                }
            }
            // println!("Batch Two: {:?}", batch_two);
        }
        if entry.contains("*") {
        
        }
        if batch_two.is_empty() {
            final_batch.append(&mut batch_one); 
        } else {
            final_batch = intersection(batch_one, batch_two);
        }
        results.append(&mut final_batch);
    } else {
        let mut halves = entry.split("*");
        
        let second_half= halves.next().unwrap();
        let first_half = halves.next().unwrap();

        let mut batch_one: Vec<String> = Vec::new();
        let mut batch_two: Vec<String> = Vec::new();

        // println!("Checking Batch One");
        let big_gram = format!("{}{}", &first_half, "$");
        for i in 0..(big_gram.len()) {
            if i < big_gram.len() - 2 {
                let three_gram = &big_gram[i..(i + 3)];
                // println!("Gram: {}", three_gram);
                if !three_gram.contains("*") {
                    let terms = kgram.get_terms(three_gram);
                    for term in terms {
                        // println!("Term: {}", term);
                        if !batch_one.contains(term) && term.ends_with(first_half) {
                            batch_one.push(term.to_string()); 
                            // println!("SUCCESS");
                        }
                    }
                }
            }
            if i < big_gram.len() - 1 {
                let two_gram = &big_gram[i..(i + 2)];
                // println!("Gram: {}", two_gram);
                if !two_gram.contains("*") {
                    let terms = kgram.get_terms(two_gram);
                    for term in terms {
                        // println!("Term: {}", term);
                        if !batch_one.contains(term) && term.ends_with(first_half) {
                            batch_one.push(term.to_string()); 
                            // println!("SUCCESS");
                        }
                    }
                }
            }
            // println!("Batch One: {:?}", batch_one);
        }

        let big_gram = format!("{}{}", "$", &second_half);
        for i in 0..(big_gram.len()) {
            if i < big_gram.len() - 2 {
                let three_gram = &big_gram[i..(i + 3)];
                // println!("Gram: {}", three_gram);
                if !three_gram.contains("*") {
                    let terms = kgram.get_terms(three_gram);
                    for term in terms {
                        // println!("Term: {}", term);
                        if !batch_two.contains(term) && term.starts_with(second_half) {
                            // println!("SUCCESS");
                        }
                    }
                }
            }
            if i < big_gram.len() - 1 {
                let two_gram = &big_gram[i..(i + 2)];
                // println!("Gram: {}", two_gram);
                if !two_gram.contains("*") {
                    let terms = kgram.get_terms(two_gram);
                    for term in terms {
                        // println!("Term: {}", term);
                        if !batch_two.contains(term) && term.starts_with(second_half) {
                            batch_two.push(term.to_string()); 
                            // println!("SUCCESS");
                        }
                    }
                }
            }
        }
        // println!("Batch Two: {:?}", batch_two);
        let mut final_batch = intersection(batch_one, batch_two);
        results.append(&mut final_batch);
    }
    return results;
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
pub fn near_query(query_literal: String, index: &DiskInvertedIndex) -> Vec<u32> {
    //extract the terms from the literal
    let literals: Vec<&str> = query_literal.split(' ').collect();
    let first_term = document_parser::stem_terms(document_parser::normalize_token(literals[0].to_string()))[0].to_string();
    let mut near = literals[1].clone().to_string();
    let second_term = document_parser::stem_terms(document_parser::normalize_token(literals[2].to_string()))[0].to_string();

    near = near.replace("NEAR/", "");
    //extract the maximum distance
    let max_distance = near.parse::<i32>().unwrap();

    println!("first term: {}", first_term);

    let mut documents: Vec<u32> = Vec::new();
    //iterate through postings lists until a common document ID is found
   
    let first_term_postings = index.get_postings(&first_term).expect("Error getting postings");
    for first_posting in first_term_postings {
        let first_doc_id = first_posting.0;
        let first_positions = first_posting.6;
        let second_term_postings = index.get_postings(&second_term).expect("Error getting postings");
        for second_posting in second_term_postings {
            let second_doc_id = second_posting.0;
            let second_positions= second_posting.6;

            if first_doc_id == second_doc_id {
                if is_near(&first_positions, &second_positions, max_distance) {
                    documents.push(first_doc_id);
                }
            }
             
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
pub fn is_near(first_positions: &Vec<u32>, second_positions: &Vec<u32>, max_distance: i32) -> bool {
    let mut i = 0;
    let mut j = 0;
    let mut difference: i32;
    //iterate through the positions
    while i < first_positions.len() && j < second_positions.len() {
        difference = (second_positions[j] as i32) - (first_positions[i] as i32);
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


pub fn phrase_query(query_literal: String, index: &DiskInvertedIndex) -> Vec<u32> {
    //extract the terms from the literal
    let literals: Vec<&str> = query_literal.split(' ').collect();
    let mut normalized_literals:Vec<String> = Vec::new(); // Also stemmed...
    //normalize the literals
    for word in literals.iter() {
        normalized_literals.push(document_parser::stem_terms(document_parser::normalize_token(word.to_string()))[0].to_string());
    }

    let mut current_disk_postings = index.get_postings(&normalized_literals[0]).expect("Failed to get postings");

    let mut current_postings: Vec<PositionalPosting> = Vec::new();

    for disk_posting in current_disk_postings {
        let mut temp_posting = PositionalPosting::new(disk_posting.0);
        for position in disk_posting.6 {
            &temp_posting.add_position(position);
        }
        current_postings.push(temp_posting);
    }

    println!("{}", &normalized_literals[0]);


    for ind in 1..normalized_literals.len() {
        let next_disk_postings = index.get_postings(&normalized_literals[ind]).expect("Failed to get postings");

        println!("{}", &normalized_literals[ind]);

        let mut next: Vec<PositionalPosting> = Vec::new();
        
        for disk_posting in next_disk_postings{
            let mut temp_posting = PositionalPosting::new(disk_posting.0);
            for position in disk_posting.6 {
                &temp_posting.add_position(position);
            }
            next.push(temp_posting);
        }
        let mut i = 0;
        let mut j = 0;

        // list of postings containing document ids that terms share in common and positions
        let mut merged:Vec<PositionalPosting> = Vec::new();

        //iterate through postings lists until a common document ID is found
        
        while i < current_postings.len() && j < next.len() {
            if current_postings[i].get_doc_id() == next[j].get_doc_id() {
                
                //if the two terms have a common document, retrieve the positions
                let positions_of_current = current_postings[i].get_positions();
                let positions_of_next = next[j].get_positions();
                //return all positions of the second term where the terms are adjacent to each other
                
                let merged_positions = adjacent_positions(&positions_of_next, &positions_of_current);
                //if none exist we can continue
                if merged_positions.is_empty() {
                    i = i + 1;
                    j = j + 1;
                    continue;
                }
                //create new positional posting to push to merged list of postings
                let mut temp_posting = PositionalPosting::new(current_postings[i].get_doc_id());
                for pos in merged_positions {
                    temp_posting.add_position(pos);
                }
                merged.push(temp_posting);
                
                i = i + 1;
                j = j + 1;

            } else if current_postings[i].get_doc_id() < next[j].get_doc_id() {
                i = i + 1;
            } else if current_postings[i].get_doc_id() > next[j].get_doc_id() {
                j = j + 1;
            } 
        }
        current_postings = merged;
    }

    let mut documents:Vec<u32> = Vec::new();
    
    for i in current_postings {
        documents.push(i.get_doc_id());
    }

    return documents;
}

pub fn adjacent_positions(term_positions: &Vec<u32>, positions: &Vec<u32>) -> Vec<u32> {
    let mut i = 0;
    let mut j = 0;
    let mut off_by_one_positions: Vec<u32> = Vec::new();
    //iterate through the positions

    while j < term_positions.len() && i < positions.len() {
        let difference = (term_positions[j]as i32) - (positions[i] as i32);
        //if the distance is within the max_distance then we return true
        if difference == 1 {
            off_by_one_positions.push(term_positions[j]);
            i = i + 1;
            j = j + 1;
        // if the first position comes before the second then we increment the second position vector
        } else if difference <= 0 {
            j = j + 1;
        // if the second position comes more than the threshold after the first one, increment the first position vector
        } else if difference > 1 {
            i = i + 1;
        }
    }
    //println!("{:?}", off_by_one_positions);
    off_by_one_positions
}

pub fn intersection<T: Clone + Ord + PartialOrd >(first: Vec<T>, second: Vec<T>) -> Vec<T> {

    let mut intersect: Vec<T> = Vec::new();
    for i in 0..first.len() {
        if i==0 || (i>0 && first[i]!=first[i-1]) { 
            let r = second.binary_search(&first[i]);
            match r { Ok(_) => intersect.push(first[i].clone()),
                     Err(_) => (), 
            
            }
        }
    }
    return intersect;
}
