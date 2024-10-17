use crate::syntax_tree::alias::Alias;
use std::fs;
use tree_sitter::Parser;

use super::find_aliases;

pub fn process_file(file_path: &str) -> Vec<Alias> {
    let code = fs::read_to_string(file_path).expect("Error reading file");

    let mut parser = Parser::new();
    let language = tree_sitter_bash::LANGUAGE;

    parser
        .set_language(&language.into())
        .expect("Error loading Bash language");

    let tree = parser.parse(&code, None).expect("Error parsing code");

    let mut cursor = tree.walk();

    find_aliases(&mut cursor, code.as_bytes())
}
