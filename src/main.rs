mod command;
mod config;
mod output;
mod syntax_tree;

use command::arguments::CliArgs;
use config::alias_ignore::{self, AliasIgnoreResult};
use output::writer::process_and_write_aliases;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use syntax_tree::process_file::process_path;

fn main() {
    let args = CliArgs::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    let file_path = PathBuf::from(&args.file_path);

    if !Path::new(&args.file_path).exists() {
        eprintln!("Error: File path {} does not exist", args.file_path);
        std::process::exit(1);
    }

    let alias_ignore_result =
        alias_ignore::get_ignore_set().unwrap_or_else(|| AliasIgnoreResult {
            alias_ignores:   HashSet::new(),
            command_ignores: Vec::new(),
        });

    let aliases = process_path(file_path);
    process_and_write_aliases(aliases, alias_ignore_result, &args);
}
