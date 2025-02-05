extern crate core;
extern crate tree_sitter;

use core::{glob, Inspect, Inspector};
use std::path::PathBuf;

mod parser;
mod visitor;

pub struct KotlinInspector;

impl Inspector for KotlinInspector {
  fn inspect(&self, content: String) -> Vec<Inspect> {
    let tree = parser::parse(&content);
    visitor::get_program_inspects(tree, &content)
  }

  fn get_modules_filter(&self, _cwd: &PathBuf) -> Vec<String> {
    vec!["*".to_string()]
  }

  fn get_files(&self, cwd: &PathBuf, include: Vec<String>) -> Vec<String> {
    get_files(cwd, include)
  }
}

fn get_files(cwd: &PathBuf, include: Vec<String>) -> Vec<String> {
  let mut patterns = include.clone();

  let include_patterns = vec!["**/*.kt".to_string()];
  let mut ignore_patterns = vec![
    "build/*".to_string(),
    "out/*".to_string(),
    "**/*Test.kt".to_string(),
    "**/*.generated.kt".to_string(),
    "**/*.kts".to_string(),
  ];

  ignore_patterns.iter_mut().for_each(|s| s.insert(0, '!'));

  patterns.extend(ignore_patterns);
  patterns.extend(include_patterns);

  glob(cwd, patterns)
}
