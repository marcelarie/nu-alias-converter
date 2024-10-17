use std::collections::HashSet;

use crate::command::arguments::DEBUG_MODE_GLOBAL;

/// Represents the result of parsing the `.aliasignore` file.
/// * `alias_ignores` - A set of alias names to ignore.
/// * `command_ignores` - A list of command names to ignore.
pub struct AliasIgnoreResult {
    pub alias_ignores: HashSet<String>, 
    pub command_ignores: Vec<String>, 
}

/// Parses the `.aliasignore` file and returns the result.
pub fn get_ignore_set() -> Option<AliasIgnoreResult> {
    let ignore_file = std::fs::read_to_string(".aliasignore");
    let should_debug = *DEBUG_MODE_GLOBAL.get().unwrap_or(&false);

    match ignore_file {
        Ok(file_content) => {
            let mut alias_ignores = HashSet::new();
            let mut command_ignores = Vec::new();

            for line in file_content.lines() {
                let first_char = line.chars().next();
                let alias_name = line.trim_start_matches('!').to_string();

                if first_char == Some('!') {
                    if should_debug {
                        println!(
                            "IGNORE_CMD({}): Command ignored from alias list",
                            alias_name
                        );
                    }
                    command_ignores.push(alias_name);
                } else {
                    if should_debug {
                        println!(
                            "IGNORE({}): Alias ignored from alias list",
                            alias_name
                        );
                    }

                    alias_ignores.insert(alias_name);
                }
            }

            Some(AliasIgnoreResult {
                alias_ignores,
                command_ignores,
            })
        }
        Err(_) => None,
    }
}
