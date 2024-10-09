mod syntax_tree;

use std::fs;

use syntax_tree::print_tree;
use tree_sitter::Parser;

fn main() {
    const TEST_FILE_PATH: &str = "./src/test/examples/.bash_aliases";
    let code = fs::read_to_string(TEST_FILE_PATH).expect("Error reading file");

    let mut parser = Parser::new();
    let language = tree_sitter_bash::LANGUAGE;

    parser
        .set_language(&language.into())
        .expect("Error loading Bash language");

    let tree = match parser.parse(code.clone(), None) {
        Some(tree) => tree,
        None => {
            println!("Error parsing code");
            return;
        }
    };

    let mut cursor = tree.walk();

    println!("Parsed Tree:");
    print_tree(&mut cursor, code.as_bytes(), 0);
}
