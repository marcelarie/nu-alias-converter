use tree_sitter::Parser;
use tree_sitter_nu::LANGUAGE;

pub fn validate_nu_tree_sitter_code(content: &String) -> bool {
    let mut parser = Parser::new();
    let nu_lang = LANGUAGE.into();

    parser
        .set_language(&nu_lang)
        .expect("Error loading Nu parser");

    parser.parse(content, None).is_some()
}
