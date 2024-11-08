use crate::command::arguments::DEBUG_MODE_GLOBAL;
use crate::syntax_tree::alias::Alias;
use std::cell::RefCell;
use std::rc::Rc;
use std::{fs, path::PathBuf};
use tree_sitter::Parser;

use super::find_aliases;

// Validate the file path
// * `file_path` - The path to the file to be validated
// # Returns
// A boolean value indicating whether the file path is valid or not
fn validate_file_path(file_path: &PathBuf) -> bool {
    let valid_extension = file_path
        .extension()
        .map(|ext| ext == "sh" || ext == "bash" || ext == "zsh")
        .unwrap_or(false);

    let full_path = file_path.to_string_lossy();

    let valid_full_path = full_path == "/etc/profile"
        || full_path == "/etc/bash.bashrc"
        || full_path == "/etc/zsh/zshrc"
        || full_path == "/etc/zsh/zprofile"
        || full_path == "/etc/zshrc"
        || full_path == "/etc/bashrc";

    let valid_name = file_path
        .file_name()
        .map(|name| {
            let name = name.to_string_lossy();

            if name.starts_with(".bash_history") {
                return false;
            }

            name.starts_with(".bash")
                || name.starts_with(".zsh")
                || name.ends_with(".env")
                || name == ".profile"
                || name == ".aliases"
                || name == ".shellrc"
        })
        .unwrap_or(false);

    valid_full_path || valid_extension || valid_name
}

/// Processes a single file to extract aliases.
///
/// * `parser` - A reference-counted, refcell-wrapped Parser instance.
/// * `file_path` - The path to the file to be processed.
///
/// # Returns
/// A vector of `Alias` instances extracted from the file.
pub fn process_file(parser: Rc<RefCell<Parser>>, file_path: PathBuf) -> Vec<Alias> {
    let should_debug = *DEBUG_MODE_GLOBAL.get().unwrap_or(&false);

    if !validate_file_path(&file_path) {
        return Vec::new();
    }
    if should_debug {
        println!("Processing valid file: {}", file_path.display().to_string());
    }
    match fs::read_to_string(&file_path) {
        Ok(code) => {
            let mut parser = parser.borrow_mut();
            let tree = parser.parse(&code, None).expect("Error parsing code");
            let mut cursor = tree.walk();
            find_aliases(&mut cursor, code.as_bytes())
        }
        Err(e) => {
            if should_debug {
                eprintln!(
                    "ERROR_READING({}): {:?}",
                    file_path.display().to_string(),
                    e
                );
            }
            Vec::new()
        }
    }
}

/// Processes all files in a directory to extract aliases.
///
/// * `parser` - A reference-counted, refcell-wrapped Parser instance.
/// * `dir_path` - The path to the directory to be processed.
///
/// # Returns
/// A vector of `Alias` instances extracted from all files in the directory.
pub fn process_dir(parser: Rc<RefCell<Parser>>, dir_path: PathBuf) -> Vec<Alias> {
    let files = fs::read_dir(dir_path).expect("Error reading directory");

    let mut all_aliases = Vec::new();

    for file in files {
        let file = file.expect("Error reading file");
        let path = file.path();

        if path.is_file() {
            let aliases = process_file(parser.clone(), path);
            all_aliases.extend(aliases);
        }
    }

    all_aliases
}

/// Processes a file or directory path to extract aliases.
///
/// * `file_path` - The path to the file or directory to be processed.
///
/// # Returns
/// A vector of `Alias` instances extracted from the file or directory.
pub fn process_path(file_path: PathBuf) -> Vec<Alias> {
    let mut parser = Parser::new();
    let language = tree_sitter_bash::LANGUAGE;

    parser
        .set_language(&language.into())
        .expect("Error loading Bash language");

    let parser = Rc::new(RefCell::new(parser));

    if file_path.is_dir() {
        process_dir(parser, file_path)
    } else {
        process_file(parser, file_path)
    }
}

// Write boilerplate tests
#[cfg(test)]
mod tests {
    mod should_pass {
        use super::super::*;

        #[test]
        fn extracts_aliases_from_file() {
            let file_path = PathBuf::from("./src/test/examples/.bash_aliases");
            let aliases = process_path(file_path);
            assert_eq!(aliases.len(), 7);
            assert_eq!(aliases[1].name, "ll");
            assert_eq!(aliases[1].content, "ls -l");
            assert_eq!(aliases[2].name, "abc!");
            assert_eq!(aliases[2].content, "echo String with special characters");
            assert_eq!(aliases[4].name, "gitlog");
            assert_eq!(
                aliases[4].content,
                "git log --graph --oneline --decorate --all"
            );
        }

        #[test]
        fn extracts_aliases_from_directory() {
            let dir_path = PathBuf::from("./src/test/examples/aliases_dir/");
            let aliases = process_path(dir_path);
            assert_eq!(aliases.len(), 5);
            assert_eq!(aliases[0].name, "brc");
            assert_eq!(aliases[0].content, "cat \"$HOME\"/.bashrc");
            assert_eq!(aliases[1].name, "brcs");
            assert_eq!(aliases[1].content, "source \"$HOME\"/.bashrc");
            assert_eq!(aliases[2].name, "ls");
            assert_eq!(aliases[2].content, "ls --color=auto");
            assert_eq!(aliases[3].name, "ll");
            assert_eq!(aliases[3].content, "ls -l");
            assert_eq!(aliases[4].name, "la");
            assert_eq!(aliases[4].content, "ls -A");
        }

        #[test]
        fn validates_file_paths() {
            let valid_file_path_profile = PathBuf::from("/etc/profile");
            let valid_file_path_bashrc = PathBuf::from("~/.bashrc");
            let invalid_file_path = PathBuf::from("/etc/hosts");

            assert!(validate_file_path(&valid_file_path_profile));
            assert!(validate_file_path(&valid_file_path_bashrc));
            assert!(!validate_file_path(&invalid_file_path));
        }
    }

    mod should_fail {
        use super::super::*;

        #[test]
        fn unknown_flags() {
            let file_path = PathBuf::from("./src/test/examples/.bash_aliases");
            let aliases = process_path(file_path);
            assert_eq!(aliases[0].name, "ls");
            assert_eq!(aliases[0].content, "ls --color=auto");
            assert!(!aliases[0].is_valid_nushell);
            assert_eq!(
                aliases[0].error_messages[0],
                "The `ls` command doesn't have flag `color`."
            );
            assert_eq!(aliases[3].name, "la");
            assert_eq!(aliases[3].content, "ls -A");
            assert!(!aliases[3].is_valid_nushell);
            assert_eq!(
                aliases[3].error_messages[0],
                "The `ls` command doesn't have flag `-A`."
            );
        }

        #[test]
        fn undefined_variables() {
            let file_path = PathBuf::from("./src/test/examples/.bash_aliases");
            let aliases = process_path(file_path);
            assert_eq!(aliases[5].name, "invalid_nushell_alias");
            assert_eq!(aliases[5].content, "echo $HOME");
            assert!(!aliases[5].is_valid_nushell);
            assert_eq!(aliases[5].error_messages[0], "Variable not found.");
        }

        #[test]
        fn alias_to_parser_keyword() {
            let file_path = PathBuf::from("./src/test/examples/.bash_aliases");
            let aliases = process_path(file_path);
            assert_eq!(aliases[6].name, "node15");
            assert_eq!(aliases[6].content, "source /usr/share/nvm/init-nvm.sh");
            assert!(!aliases[6].is_valid_nushell);
            assert_eq!(
                aliases[6].error_messages[0],
                "Can't create alias to parser keyword."
            );
        }
    }
}
