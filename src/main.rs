mod syntax_tree;

use std::fs;

// use syntax_tree::print_tree;
use tree_sitter::Parser;

fn get_concatenation(cursor: &mut tree_sitter::TreeCursor) {
    let node = cursor.node();
    let kind = node.kind();

    if kind == "word" {
        println!("I am a word");
    }

    if kind == "raw_string" {
        println!("I am a raw string");
    }

    if cursor.goto_first_child() {
        get_concatenation(cursor);
        while cursor.goto_next_sibling() {
            get_concatenation(cursor);
        }
        cursor.goto_parent();
    }
}

fn get_command(cursor: &mut tree_sitter::TreeCursor) {
    let node = cursor.node();
    let kind = node.kind();

    if kind == "concatenation" {
        get_concatenation(cursor);
    }

    if cursor.goto_first_child() {
        get_command(cursor);
        while cursor.goto_next_sibling() {
            get_command(cursor);
        }
        cursor.goto_parent();
    }
}

fn get_aliases(cursor: &mut tree_sitter::TreeCursor) {
    let node = cursor.node();
    let kind = node.kind();

    if kind == "command" {
        get_command(cursor)
    }

    // Recursively get aliases
    if cursor.goto_first_child() {
        get_aliases(cursor);
        while cursor.goto_next_sibling() {
            get_aliases(cursor);
        }
        cursor.goto_parent();
    }
}

fn main() {
    const TEST_FILE_PATH: &str = "./src/test/examples/bash_aliases";
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

    // println!("Parsed Tree:");
    // print_tree(&mut cursor, code.as_bytes(), 0);
    println!("Aliases:");
    get_aliases(&mut cursor);
}
