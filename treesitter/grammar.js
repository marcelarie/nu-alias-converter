/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: "bash_aliases_converter",

  rules: {
    // TODO: add the actual grammar rules
    source_file: $ => "hello"
  }
});
