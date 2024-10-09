mod syntax_tree;

use syntax_tree::print_tree;
use tree_sitter::Parser;

fn main() {
    let code = "echo \"hello world!\"";

    let mut parser = Parser::new();
    let language = tree_sitter_bash::LANGUAGE;

    parser
        .set_language(&language.into())
        .expect("Error loading Bash language");

    let tree = match parser.parse(code, None) {
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
