use std::collections::HashSet;

use crate::command::arguments::DEBUG_MODE_GLOBAL;

pub struct AliasIgnoreResult {
    pub alias_ignores: HashSet<String>, // To store alias names to ignore
    pub command_ignores: Vec<String>, // To store commands from ! that appear inside aliases
}

pub fn get_ignore_set() -> Option<AliasIgnoreResult> {
    let ignore_file = std::fs::read_to_string(".aliasignore");

    match ignore_file {
        Ok(file_content) => {
            let mut alias_ignores = HashSet::new();
            let mut command_ignores = Vec::new();

            for line in file_content.lines() {
                let first_char = line.chars().next();
                let alias_name = line.trim_start_matches('!').to_string();

                if first_char == Some('!') {
                    if *DEBUG_MODE_GLOBAL.get().unwrap_or(&false) {
                        println!(
                            "IGNORE_CMD({}): Command ignored from alias list",
                            alias_name
                        );
                    }
                    command_ignores.push(alias_name);
                } else {
                    if *DEBUG_MODE_GLOBAL.get().unwrap_or(&false) {
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
