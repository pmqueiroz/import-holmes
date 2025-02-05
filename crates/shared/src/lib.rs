extern crate serde;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawInspect {
  pub specifier: String,
  pub local_specifier: String,
  pub module_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inspect {
  pub raw: RawInspect,
  pub referenced: usize,
  pub occurrences: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FinalInspect {
  pub specifier: String,
  pub module_name: String,
  pub aliases: Vec<String>,
  pub referenced: usize,
  pub occurrences: usize,
}

pub trait Inspector: Send + Sync {
  fn inspect(&self, content: String) -> Vec<Inspect>;

  fn to_final_inspects(&self, inspects: Vec<Inspect>) -> Vec<FinalInspect>;

  fn get_dependencies(&self, cwd: &PathBuf) -> Vec<String>;
}
