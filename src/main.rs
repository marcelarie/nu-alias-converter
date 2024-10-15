mod command;
mod syntax_tree;

use command::arguments::CliArgs;
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};
use syntax_tree::find_aliases;
use tree_sitter::Parser;

fn main() {
    let args = CliArgs::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    let file_exists = Path::new(&args.file_path).exists();

    if !file_exists {
        eprintln!("Error: File path {} does not exist", args.file_path);
        std::process::exit(1);
    }

    let code = fs::read_to_string(&args.file_path).expect("Error reading file");

    let mut parser = Parser::new();
    let language = tree_sitter_bash::LANGUAGE;

    parser
        .set_language(&language.into())
        .expect("Error loading Bash language");

    let tree = parser.parse(&code, None).expect("Error parsing code");

    let mut cursor = tree.walk();

    let aliases = find_aliases(&mut cursor, code.as_bytes());

    let output_file_path = "alias.nu";
    let file =
        File::create(&output_file_path).expect("Error creating output file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "# Aliases auto generated by nu-alias-converter\n")
        .expect("Error writing to file");

    let has_valid_aliases = aliases.iter().any(|alias| alias.is_valid_nushell);

    if !has_valid_aliases {
        println!(
            "No valid Nushell aliases found in the Bash file '{}'",
            &args.file_path
        );
        if !args.no_comments {
            println!(
                "A file '{}' was generated with comments for invalid aliases",
                output_file_path
            );
        }
        return;
    }

    for alias in aliases {
        if alias.is_valid_nushell {
            writeln!(writer, "alias {} = {}", alias.name, alias.content)
                .expect("Error writing to file");
        } else if !args.no_comments {
            writeln!(
                writer,
                "# alias {} = {} # Errors: {}",
                alias.name,
                alias.content,
                alias.error_messages.join(", ")
            )
            .expect("Error writing to file");
        }
    }

    println!("Nushell aliases written to file '{}'", output_file_path);

    writer.flush().expect("Error flushing the buffer");
}
