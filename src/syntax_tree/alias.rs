// TODO: Handle each error of this function with a enum (AliasError)
// return: -> Result<(String, String), AliasError> {
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

    let node = cursor.node();

    if node.child_count() != 2 {
        return None;
    }

    cursor.goto_first_child();

    let alias_name_node = cursor.node();
    let alias_name = alias_name_node
        .utf8_text(source)
        .unwrap()
        .trim_end_matches('=');

    cursor.goto_next_sibling();

    let alias_content_node = cursor.node();
    let alias_content = alias_content_node
        .utf8_text(source)
        .unwrap()
        // TODO: Check all the cases for quotting in the Bash reference manual:
        // https://www.gnu.org/software/bash/manual/html_node/Quoting.html
        // and implement a unquote content fn
        .trim_matches('\'');

    Some((alias_name.to_string(), alias_content.to_string()))
}

pub fn find_aliases(
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
        } // TODO: Implement alias detection inside functions
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
