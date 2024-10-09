import XCTest
import SwiftTreeSitter
import TreeSitterBashAliasesConverter

final class TreeSitterBashAliasesConverterTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_bash_aliases_converter())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading BashAliasesConverter grammar")
    }
}
