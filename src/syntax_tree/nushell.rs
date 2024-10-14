use nu_parser::parse;
use nu_protocol::engine::{EngineState, StateWorkingSet};
use tree_sitter::Parser;
use tree_sitter_nu::LANGUAGE;

// use nu_source::{Span, Tag};

pub fn validate_nu_tree_sitter_code(content: &String) -> bool {
    let mut parser = Parser::new();
    let nu_lang = LANGUAGE.into();

    parser
        .set_language(&nu_lang)
        .expect("Error loading Nu parser");

    parser.parse(content, None).is_some()
}

pub fn validate_alias_with_nu_parser(name: &str, content: &str) -> bool {
    let engine_state = EngineState::new();
    let mut working_set = StateWorkingSet::new(&engine_state);

    let alias_declaration = format!("alias {} = {}", name, content);

    let _ =
        parse(&mut working_set, None, alias_declaration.as_bytes(), true);

    println!("{:?}", working_set.parse_errors );

    working_set.parse_errors.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_nu_tree_sitter_code_valid_input() {
        let valid_nu_code = "alias ll = ls -alF".to_string();
        let result = validate_nu_tree_sitter_code(&valid_nu_code);
        assert!(
            result,
            "Expected valid Nu code to return true, but got false"
        );
    }

    #[test]
    fn test_validate_alias_with_nu_parser_valid_input() {
        let valid_alias_name = "ll";
        let valid_alias_content = "ls";

        let result = validate_alias_with_nu_parser(
            valid_alias_name,
            valid_alias_content,
        );

        assert!(result, "Expected valid alias to return true, but got false");
    }

    #[test]
    fn test_validate_alias_with_nu_parser_invalid_input() {
        let invalid_alias_name = "homer";
        let invalid_alias_content = "echo $HOME";
        let result = validate_alias_with_nu_parser(
            invalid_alias_name,
            invalid_alias_content,
        );
        assert!(
            !result,
            "Expected invalid alias to return false, but got true"
        );
    }
}
