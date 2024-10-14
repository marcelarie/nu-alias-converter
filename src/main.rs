mod command;
mod syntax_tree;

use command::arguments::CliArgs;
use std::fs;
use syntax_tree::find_aliases;
use tree_sitter::Parser;

fn main() {
    let args = CliArgs::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let code = fs::read_to_string(args.file_path).expect("Error reading file");

    let mut parser = Parser::new();
    let language = tree_sitter_bash::LANGUAGE;

    parser
        .set_language(&language.into())
        .expect("Error loading Bash language");

    let tree = parser.parse(&code, None).expect("Error parsing code");

    let mut cursor = tree.walk();

    let aliases = find_aliases(&mut cursor, code.as_bytes());
    for alias in aliases {
        if alias.is_valid_nushell {
            println!("alias {} = {}", alias.name, alias.content);
        } else {
            println!(
                "# alias {} = {} # Invalid nushell alias",
                alias.name, alias.content
            );
        }
    }
}
