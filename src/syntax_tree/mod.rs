pub mod alias;
pub mod nushell;
pub mod printer;
pub mod process_file;
pub mod traverser;

#[allow(unused)]
pub use printer::print_tree;
#[allow(unused)]
pub use traverser::traverse_tree;

pub use alias::find_aliases;
// #[allow(unused)]
// pub use nushell::validate_nu_tree_sitter_code;
#[allow(unused)]
pub use nushell::validate_alias_with_nu_parser;
