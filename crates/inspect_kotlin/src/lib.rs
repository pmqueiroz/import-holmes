extern crate tree_sitter;

use tree_sitter::{Parser, TreeCursor};

pub fn inspect_file(source_code: &str) {
  let mut parser = Parser::new();
  parser
    .set_language(&tree_sitter_kotlin::language())
    .expect("Error loading kotlin grammar");
  let tree = parser.parse(source_code, None).unwrap();

  let root_node = tree.root_node();
  let mut cursor = root_node.walk();

  fn visit_imports(cursor: &mut TreeCursor, source_code: &str) {
    loop {
      let node = cursor.node();
      if node.kind() == "import_header" {
        let import_text = node.utf8_text(source_code.as_bytes()).unwrap();
        println!("Found import: {}", import_text);
      }

      if cursor.goto_first_child() {
        visit_imports(cursor, source_code);
        cursor.goto_parent();
      }

      if !cursor.goto_next_sibling() {
        break;
      }
    }
  }

  visit_imports(&mut cursor, source_code);
}
