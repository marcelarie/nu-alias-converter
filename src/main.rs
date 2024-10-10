mod syntax_tree;

use std::fs;
use syntax_tree::find_aliases;
use tree_sitter::Parser;

fn main() {
    const TEST_FILE_PATH: &str = "./src/test/examples/special_chars_bash_alias";
    let code = fs::read_to_string(TEST_FILE_PATH).expect("Error reading file");

    let mut parser = Parser::new();
    let language = tree_sitter_bash::LANGUAGE;

    parser
        .set_language(&language.into())
        .expect("Error loading Bash language");

    let tree = parser.parse(&code, None).expect("Error parsing code");

    let mut cursor = tree.walk();

    let aliases = find_aliases(&mut cursor, code.as_bytes());
    for (name, command) in aliases {
        println!("{} => {}", name, command);
    }
}
