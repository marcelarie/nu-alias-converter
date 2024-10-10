// Unquote a string (remove the quotes if it has)
// 'foo' -> foo
// "foo" -> foo
// foo -> foo
// "\'foo\'" -> 'foo'
// TODO: Add tests
fn unquote_string(string: &str) -> String {
    let has_single_quotes = string.starts_with('\'') && string.ends_with('\'');
    let has_double_quotes = string.starts_with('"') && string.ends_with('"');

    if has_single_quotes || has_double_quotes {
        string[1..string.len() - 1].to_string()
    } else {
        string.to_string()
    }
}

enum AliasError {
    MissingCommandName,
    MissingAliasName,
    MissingArguments,
    InvalidArgumentCount,
    InvalidUtf8Text,
}

// https://www.gnu.org/software/bash/manual/html_node/Quoting.html
// TODO: Handle each error of this function with a enum (AliasError)
// return: -> Result<(String, String), AliasError> {
fn extract_alias(
    node: tree_sitter::Node,
    source: &[u8],
) -> Result<(String, String), AliasError> {
    let mut cursor = node.walk();

    // Find the command_name node
    if !cursor.goto_first_child() || cursor.node().kind() != "command_name" {
        return Err(AliasError::MissingCommandName);
    }

    // Check if it's an alias command
    let command_name = cursor.node().utf8_text(source).unwrap();
    if command_name != "alias" {
        return Err(AliasError::MissingAliasName);
    }

    // Go to the argument node
    if !cursor.goto_next_sibling() {
        return Err(AliasError::MissingArguments);
    }

    let node = cursor.node();

    if node.child_count() != 2 {
        return Err(AliasError::InvalidArgumentCount);
    }

    cursor.goto_first_child();

    let alias_name_node = cursor.node();
    let alias_name = alias_name_node
        .utf8_text(source)
        .unwrap()
        .trim_end_matches('=');

    cursor.goto_next_sibling();

    let alias_content = match cursor.node().utf8_text(source) {
        Ok(alias_content) => alias_content,
        Err(_) => return Err(AliasError::InvalidUtf8Text),
    };

    let unquoted_alias_content = unquote_string(alias_content);

    Ok((alias_name.to_string(), unquoted_alias_content))
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
            if let Ok(alias) = extract_alias(node, source) {
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
