use tree_sitter::{Node, TreeCursor};

pub fn walk_root_cursor_to_node(cursor: &mut TreeCursor, node: Node) {
    println!("walk_root_cursor_to_node() target node: {node:#?}");
    while cursor.node() != node {
        println!(
            "walk_root_cursor_to_node() current cursor node: {:#?}",
            cursor.node()
        );
        if cursor.node().kind() == "member_expression" {
            let mut cursor = cursor.node().walk();
            for child in cursor.node().children(&mut cursor) {
                println!("member_expression child: {child:#?}");
            }
        }
        cursor.goto_first_child_for_byte(node.start_byte()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::ops;

    use tree_sitter::Parser;

    use super::*;

    fn test_walk(target_node_byte_range: ops::Range<usize>, target_node_text: &str) {
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_javascript::language())
            .unwrap();
        let source_text = "var a = function() { return 12; }.bind(b)";
        let tree = parser.parse(source_text, None).unwrap();
        let target_node = tree
            .root_node()
            .descendant_for_byte_range(target_node_byte_range.start, target_node_byte_range.end)
            .unwrap();
        assert_eq!(
            target_node.utf8_text(source_text.as_bytes()).unwrap(),
            target_node_text
        );
        let mut root_cursor = tree.walk();
        walk_root_cursor_to_node(&mut root_cursor, target_node);
        assert_eq!(root_cursor.node(), target_node);
    }

    #[test]
    fn test_walk_root_cursor_to_end_curly_brace_node() {
        test_walk(32..33, "}");
    }

    #[test]
    fn test_walk_root_cursor_to_dot_node() {
        test_walk(33..34, ".");
    }
}
