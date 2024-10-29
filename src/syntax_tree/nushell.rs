use nu_parser::parse;
use nu_protocol::engine::{EngineState, StateWorkingSet};
// use tree_sitter::Parser;
// use tree_sitter_nu::LANGUAGE;

use crate::command::arguments;

// TODO: Check if this is useful
// #[allow(unused)]
// pub fn validate_nu_tree_sitter_code(content: &String) -> bool {
//     let mut parser = Parser::new();
//     let nu_lang = LANGUAGE.into();
//
//     parser
//         .set_language(&nu_lang)
//         .expect("Error loading Nu parser");
//
//     parser.parse(content, None).is_some()
// }

// Other engine state generation methods from nushell:
//     let engine_state = nu_cmd_lang::create_default_context();
//     #[cfg(feature = "plugin")]
//     let engine_state = nu_cmd_plugin::add_plugin_command_context(engine_state);
//     let engine_state = nu_command::add_shell_command_context(engine_state);
//     let engine_state = nu_cmd_extra::add_extra_command_context(engine_state);
//     let engine_state = nu_cli::add_cli_context(engine_state);
//     nu_explore::add_explore_context(engine_state)
fn create_nu_engine_state() -> EngineState {
    let engine_state = nu_command::add_shell_command_context(
        nu_cmd_lang::create_default_context(),
    );

    engine_state
}

pub struct AliasValidationResult {
    pub is_valid:       bool,
    pub error_messages: Vec<String>,
}

pub fn validate_alias_with_nu_parser(
    name: &str,
    content: &str,
) -> AliasValidationResult {
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
        let mut error_messages = Vec::new();
        for error in &working_set.parse_errors {
            if arguments::is_debug_mode() {
                println!("ERROR({}): {:?}", name, error);
            }
            error_messages.push(error.to_string());
        }
        AliasValidationResult {
            is_valid: false,
            error_messages,
        }
    } else {
        AliasValidationResult {
            is_valid:       true,
            error_messages: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn validate_nu_alias_with_tree_sitter_valid_input() {
    //     let valid_nu_code = "alias ll = ls -alF".to_string();
    //     let result = validate_nu_tree_sitter_code(&valid_nu_code);
    //     assert!(
    //         result,
    //         "Expected valid Nu code to return true, but got false"
    //     );
    // }

    #[test]
    fn validate_nu_alias_with_parser_valid_input() {
        let valid_alias_name = "ll";
        let valid_alias_content = "ls";

        let result = validate_alias_with_nu_parser(
            valid_alias_name,
            valid_alias_content,
        );

        assert!(
            result.is_valid,
            "Expected valid alias to return true, but got false"
        );
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
            !result.is_valid,
            "Expected invalid alias to return false, but got true"
        );
    }

    #[test]
    fn validate_nu_alias_with_parser_invalid_source_content() {
        let invalid_alias_name = "node15";
        let invalid_alias_content = "source /usr/share/nvm/init-nvm.sh";
        let result = validate_alias_with_nu_parser(
            invalid_alias_name,
            invalid_alias_content,
        );
        assert!(
            !result.is_valid,
            "Expected invalid alias to return false, but got true"
        );
    }

    // TODO: Fix this test
    // #[test]
    // fn validate_nu_alias_with_parser_invalid_non_zero_exit_code() {
    //     let invalid_alias_name = "zkn";
    //     let invalid_alias_content = "cd ~/notes; ~/scripts/zk-new";
    //     let result = validate_alias_with_nu_parser(
    //         invalid_alias_name,
    //         invalid_alias_content,
    //     );
    //     assert!(
    //         !result.is_valid,
    //         "Expected invalid alias to return false, but got true"
    //     );
    // }
}
