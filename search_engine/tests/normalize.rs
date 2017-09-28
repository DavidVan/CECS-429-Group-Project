extern crate search_engine;
extern crate stemmer;

use stemmer::Stemmer;
use search_engine::parser::document_parser;

fn test_term() {
    let term = "Test";

    document_parser.normalize_token(term);
}
