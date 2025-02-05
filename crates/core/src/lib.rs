extern crate serde;

use serde::{Deserialize, Serialize};
use std::fmt;
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FinalInspect {
  pub specifier: String,
  pub module_name: String,
  pub aliases: Vec<String>,
  pub referenced: usize,
  pub occurrences: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
  TypeScript,
  Kotlin,
}

impl Language {
  pub fn from_str(s: &str) -> Option<Self> {
    if s.eq_ignore_ascii_case("typescript") {
      Some(Language::TypeScript)
    } else if s.eq_ignore_ascii_case("kotlin") {
      Some(Language::Kotlin)
    } else {
      None
    }
  }
}

impl fmt::Display for Language {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Language::TypeScript => write!(f, "TypeScript"),
      Language::Kotlin => write!(f, "Kotlin"),
    }
  }
}

pub trait Inspector: Send + Sync {
  fn inspect(&self, content: String) -> Vec<Inspect>;

  fn to_final_inspects(&self, inspects: Vec<Inspect>) -> Vec<FinalInspect>;

  fn get_dependencies(&self, cwd: &PathBuf) -> Vec<String>;

  fn get_files(&self, cwd: &PathBuf, include: Vec<String>) -> Vec<String>;
}
