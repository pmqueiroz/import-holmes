extern crate core;
extern crate rayon;
extern crate serde;
extern crate serde_json;
extern crate swc_common;
extern crate swc_ecma_ast;
extern crate swc_ecma_parser;
extern crate swc_ecma_visit;

use core::{Inspect, Inspector};
use std::path::PathBuf;

mod parser;
mod read_project;
mod visitor;

pub struct TypescriptInspector;

impl Inspector for TypescriptInspector {
  fn inspect(&self, content: String) -> Vec<Inspect> {
    inspect_module(&content)
  }

  fn get_modules_filter(&self, cwd: &PathBuf) -> Vec<String> {
    let package = read_project::read_package_json(cwd);

    read_project::get_dependencies(&package)
  }

  fn get_files(&self, cwd: &PathBuf, include: Vec<String>) -> Vec<String> {
    read_project::get_module_files(cwd, include)
  }
}

pub fn inspect_module(source_code: &str) -> Vec<Inspect> {
  let program = parser::parse_program(source_code);
  let inspects = visitor::get_program_inspects(program);

  inspects
}
