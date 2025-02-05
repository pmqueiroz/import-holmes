use core::{Inspect, RawInspect};
use std::collections::HashMap;
use tree_sitter::{Node, Tree, TreeCursor};

struct ImportVisitor {
  raw_inspects: Vec<RawInspect>,
  reference_counts: HashMap<String, usize>,
}

impl ImportVisitor {
  pub fn new() -> Self {
    Self {
      raw_inspects: Vec::new(),
      reference_counts: HashMap::new(),
    }
  }

  pub fn visit_program(&mut self, cursor: &mut TreeCursor, source_code: &str) {
    loop {
      let node = cursor.node();

      if node.kind() == "import_header" {
        self.visit_import(&node, source_code);
      }

      if node.kind() == "simple_identifier" {
        self.visit_ident(&node, source_code);
      }

      if cursor.goto_first_child() {
        self.visit_program(cursor, source_code);
        cursor.goto_parent();
      }

      if !cursor.goto_next_sibling() {
        break;
      }
    }
  }

  fn visit_import(&mut self, node: &Node, source_code: &str) {
    let import_identifier = node
      .child(1)
      .map(|n| n.utf8_text(source_code.as_bytes()).unwrap().to_string())
      .unwrap_or_default();

    let (module_name, specifier) = split_module_specifier(import_identifier);
    let import_alias = node
      .child(2)
      .filter(|n| n.kind() == "import_alias")
      .map(|n| n.utf8_text(source_code.as_bytes()).unwrap().to_string())
      .map(|alias| alias.trim_start_matches("as ").to_string());

    let raw_inspect = RawInspect {
      specifier: specifier.clone(),
      local_specifier: import_alias.unwrap_or(specifier),
      module_name,
    };

    self.raw_inspects.push(raw_inspect);
  }

  fn visit_ident(&mut self, node: &Node, source_code: &str) {
    if let Ok(ident_text) = node.utf8_text(source_code.as_bytes()) {
      let counter = self
        .reference_counts
        .entry(ident_text.to_string())
        .or_insert(0);
      *counter += 1;
    }
  }
}

fn split_module_specifier(input: String) -> (String, String) {
  let parts: Vec<&str> = input.split('.').collect();
  if parts.len() > 1 {
    let module_name = parts[..parts.len() - 1].join(".");
    let specifier = parts.last().unwrap().to_string();
    (module_name, specifier)
  } else {
    (input, String::new())
  }
}

pub fn get_program_inspects(tree: Tree, source_code: &str) -> Vec<Inspect> {
  let root_node = tree.root_node();
  let mut cursor = root_node.walk();

  let mut visitor = ImportVisitor::new();
  visitor.visit_program(&mut cursor, source_code);

  let mut inspects = Vec::new();
  for raw_inspect in visitor.raw_inspects.clone() {
    let import_offset = if raw_inspect.specifier == raw_inspect.local_specifier
    {
      visitor
        .raw_inspects
        .iter()
        .filter(|item| item.specifier == raw_inspect.specifier)
        .count()
    } else {
      1
    };

    let referenced = visitor
      .reference_counts
      .get(&raw_inspect.local_specifier)
      .map_or(0, |value| value.saturating_sub(import_offset));

    inspects.push(Inspect {
      raw: raw_inspect,
      referenced,
      occurrences: 1,
    });
  }

  inspects
}
