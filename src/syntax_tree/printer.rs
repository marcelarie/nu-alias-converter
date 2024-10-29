use tree_sitter::TreeCursor;

/// Prints the syntax tree structure starting from the current cursor position
///
/// * `cursor` - A mutable reference to a TreeCursor, which allows traversal of the syntax tree
/// * `source` - A byte slice containing the source code
/// * `depth` - The current depth in the tree, used for indentation
pub fn print_tree(cursor: &mut TreeCursor, source: &[u8], depth: usize) {
    let node = cursor.node();
    let kind = node.kind();
    let start = node.start_position();
    let end = node.end_position();

    // Print the current node
    println!(
        "{:indent$}{} [{}, {}] - [{}, {}]",
        "",
        kind,
        start.row,
        start.column,
        end.row,
        end.column,
        indent = depth * 2
    );

    // Print the node's text if it's a leaf node (no children/last node)
    if node.child_count() == 0 {
        let text = &source[node.byte_range()];
        if let Ok(text) = std::str::from_utf8(text) {
            println!(
                "{:indent$}Text: \"{}\"",
                "",
                text,
                indent = (depth + 1) * 2
            );
        }
    }

    // Recursively print child nodes
    if cursor.goto_first_child() {
        // Print the first child
        print_tree(cursor, source, depth + 1);
        // Print all sibilings of the first child
        while cursor.goto_next_sibling() {
            print_tree(cursor, source, depth + 1);
        }
        // Move the cursor back to the parent node
        cursor.goto_parent();
    }
}
