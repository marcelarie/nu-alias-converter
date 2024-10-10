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

// pub fn validate_nu_parsing(content: &String) -> bool {
//     // TODO:
// }

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
}
