package tree_sitter_bash_aliases_converter_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_bash_aliases_converter "github.com/tree-sitter/tree-sitter-bash_aliases_converter/bindings/go"
)

func TestCanLoadGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_bash_aliases_converter.Language())
	if language == nil {
		t.Errorf("Error loading BashAliasesConverter grammar")
	}
}
