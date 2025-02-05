use tree_sitter::{Parser, Tree};

pub fn parse(source_code: &str) -> Tree {
  let mut parser = Parser::new();
  parser
    .set_language(&tree_sitter_kotlin::language())
    .expect("Error loading kotlin grammar");
  let tree = parser.parse(source_code, None).unwrap();

  tree
}
