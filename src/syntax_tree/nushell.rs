use nu_cmd_lang::create_default_context;
use nu_command::add_shell_command_context;
use nu_parser::parse;
use nu_protocol::engine::{EngineState, StateWorkingSet};
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

fn create_nu_engine_state() -> EngineState {
    add_shell_command_context(create_default_context())
}

pub fn validate_alias_with_nu_parser(name: &str, content: &str) -> bool {
    let engine_state = create_nu_engine_state();
    let mut working_set = StateWorkingSet::new(&engine_state);

    let alias_command = format!("alias {} = {}", name, content);
    let alias_bytes = alias_command.as_bytes();

    let _ = working_set.add_file("alias.nu".into(), &alias_bytes.to_vec());

    parse(
        &mut working_set,
        Some("alias.nu"),
        alias_bytes,
        false, // Not scoped
    );

    if !working_set.parse_errors.is_empty() {
        for error in &working_set.parse_errors {
            println!("Nushell alias parsing Error: {:?}", error);
            println!("Error string: {}", error.to_string());
        }
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_nu_alias_with_tree_sitter_valid_input() {
        let valid_nu_code = "alias ll = ls -alF".to_string();
        let result = validate_nu_tree_sitter_code(&valid_nu_code);
        assert!(
            result,
            "Expected valid Nu code to return true, but got false"
        );
    }

    #[test]
    fn validate_nu_alias_with_parser_valid_input() {
        let valid_alias_name = "ll";
        let valid_alias_content = "ls";

        let result = validate_alias_with_nu_parser(
            valid_alias_name,
            valid_alias_content,
        );

        assert!(result, "Expected valid alias to return true, but got false");
    }

    #[test]
    fn validate_nu_alias_with_parser_invalid_input() {
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
