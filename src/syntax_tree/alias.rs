// use crate::command::arguments::DEBUG_MODE_GLOBAL;

use super::nushell::validate_alias_with_nu_parser;
use rayon::prelude::*;

/// Unquote a string by removing the surrounding quotes.
/// Bash standard:  
/// <https://www.gnu.org/software/bash/manual/html_node/Quoting.html>
///
/// Examples:  
/// unquote_string("foo")     -> foo  
/// unquote_string("'foo'")   -> foo  
/// unquote_string("\"foo\"") -> foo  
/// unquote_string("foo'")    -> foo'  
fn unquote_string(string: &str) -> String {
    if string.len() < 2 {
        return string.to_string();
    }

    let has_single_quotes = string.starts_with('\'') && string.ends_with('\'');
    let has_double_quotes = string.starts_with('"') && string.ends_with('"');

    if has_single_quotes || has_double_quotes {
        string[1..string.len() - 1].to_string()
    } else {
        string.to_string()
    }
}

/// Represents the possible errors that can occur when extracting an alias
enum AliasError {
    MissingCommandName,
    MissingAliasName,
    MissingArguments,
    InvalidArgumentCount,
    InvalidUtf8Text,
}

fn extract_alias_name(
    cursor: &mut tree_sitter::TreeCursor,
    source: &[u8],
) -> Result<String, AliasError> {
    match cursor.node().kind() {
        // This happens in cases like this:
        // alias ll='ls -l'
        "word" => match cursor.node().utf8_text(source) {
            Ok(alias_content) => {
                Ok(alias_content.trim_end_matches('=').to_string())
            }
            Err(_) => Err(AliasError::InvalidUtf8Text),
        },
        // This happens in cases like this:
        // alias "abc!"='echo String with special characters'
        "string" => {
            if !cursor.goto_first_child() {
                return Err(AliasError::MissingAliasName);
            }
            if !cursor.goto_next_sibling() {
                return Err(AliasError::MissingAliasName);
            }

            let string_node = cursor.node();

            cursor.goto_parent();

            if !cursor.goto_next_sibling() {
                return Err(AliasError::MissingAliasName);
            }

            match string_node.utf8_text(source) {
                Ok(alias_content) => Ok(alias_content.to_string()),
                Err(_) => Err(AliasError::InvalidUtf8Text),
            }
        }
        _ => Err(AliasError::MissingCommandName),
    }
}

/// Extracts an alias from the given syntax tree node.
/// Returns the alias name and content in a tuple.
/// * `node` - The syntax tree node to extract the alias from.
/// * `source` - The source code as a byte slice.
fn extract_alias(
    node: tree_sitter::Node,
    source: &[u8],
) -> Result<(String, String), AliasError> {
    let mut cursor = node.walk();

    if !cursor.goto_first_child() || cursor.node().kind() != "command_name" {
        return Err(AliasError::MissingCommandName);
    }

    let command_name = cursor.node().utf8_text(source).unwrap();
    if command_name != "alias" {
        return Err(AliasError::MissingAliasName);
    }

    if !cursor.goto_next_sibling() {
        return Err(AliasError::MissingArguments);
    }

    let node = cursor.node();

    if node.child_count() > 3 {
        return Err(AliasError::InvalidArgumentCount);
    }

    cursor.goto_first_child();

    let alias_name = extract_alias_name(&mut cursor, source)?;

    cursor.goto_next_sibling();

    let alias_content = match cursor.node().utf8_text(source) {
        Ok(alias_content) => alias_content,
        Err(_) => return Err(AliasError::InvalidUtf8Text),
    };

    let unquoted_alias_content = unquote_string(alias_content);

    // println!("alias_content: {}", alias_content);

    Ok((alias_name, unquoted_alias_content))
}

#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub content: String,
    pub is_valid_nushell: bool,
    pub error_messages: Vec<String>,
}

/// Find aliases in the given syntax tree.
/// Uses rayon for parallel processing.
pub fn find_aliases(
    cursor: &mut tree_sitter::TreeCursor,
    source: &[u8],
) -> Vec<Alias> {
    let mut aliases = Vec::new();
    // let should_debug = *DEBUG_MODE_GLOBAL.get().unwrap_or(&false);

    if !cursor.goto_first_child() {
        return aliases;
    }

    loop {
        let node = cursor.node();

        if node.kind() == "command" {
            if let Ok(alias) = extract_alias(node, source) {
                let (name, content) = alias;

                aliases.push(Alias {
                    name,
                    content,
                    is_valid_nushell: false, // validation will be done in parallel later
                    error_messages: Vec::new(),
                });
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

    // Parallel validation of aliases
    aliases.par_iter_mut().for_each(|alias| {
        let validate_result =
            validate_alias_with_nu_parser(&alias.name, &alias.content);

        // This might not be useful for the end user
        // if should_debug {
        //     println!(
        //         "PARSED({}): Alias is {}",
        //         alias.name,
        //         if validate_result.is_valid {
        //             "valid"
        //         } else {
        //             "invalid"
        //         }
        //     );
        // }
        alias.is_valid_nushell = validate_result.is_valid;
        alias.error_messages = validate_result.error_messages;
    });

    aliases
}

mod tests {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_unquote_string() {
        assert_eq!(unquote_string("foo"), "foo");
        assert_eq!(unquote_string("'foo'"), "foo");
        assert_eq!(unquote_string("\"foo\""), "foo");
        assert_eq!(unquote_string("foo'"), "foo'");
    }
}
