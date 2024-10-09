mod syntax_tree;

use std::fs;
use tree_sitter::Parser;

fn extract_alias(
    node: tree_sitter::Node,
    source: &[u8],
) -> Option<(String, String)> {
    let mut cursor = node.walk();

    // Find the command_name node
    if !cursor.goto_first_child() || cursor.node().kind() != "command_name" {
        return None;
    }

    // Check if it's an alias command
    let command_name = cursor.node().utf8_text(source).unwrap();
    if command_name != "alias" {
        return None;
    }

    // Go to the argument node
    if !cursor.goto_next_sibling() {
        return None;
    }

    // Extract alias name and command
    let argument = cursor.node();
    let argument_text = argument.utf8_text(source).unwrap();
    let parts: Vec<&str> = argument_text.splitn(2, '=').collect();

    if parts.len() != 2 {
        return None;
    }

    Some((
        parts[0].trim().to_string(),
        parts[1].trim().replace('\'', "").to_string(),
    ))
}

fn find_aliases(
    cursor: &mut tree_sitter::TreeCursor,
    source: &[u8],
) -> Vec<(String, String)> {
    let mut aliases = Vec::new();
    
    // Skip first node (program)
    if !cursor.goto_first_child() {
        return aliases;
    }

    loop {
        let node = cursor.node();

        if node.kind() == "command" {
            if let Some(alias) = extract_alias(node, source) {
                aliases.push(alias);
            }
        }
        // TODO: Implement alias detection inside functions
        // else if node.kind() == "function_definition" {
        //     if cursor.goto_first_child() {
        //         aliases.extend(find_aliases(cursor, source));
        //         cursor.goto_parent();
        //     }
        // }

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    aliases
}

fn main() {
    const TEST_FILE_PATH: &str = "./src/test/examples/bash_aliases";
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
