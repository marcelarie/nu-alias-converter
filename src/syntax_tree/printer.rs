use tree_sitter::TreeCursor;

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

    // Print the node's text if it's a leaf node
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
        print_tree(cursor, source, depth + 1);
        while cursor.goto_next_sibling() {
            print_tree(cursor, source, depth + 1);
        }
        cursor.goto_parent();
    }
}
