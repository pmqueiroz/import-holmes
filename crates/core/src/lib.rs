extern crate globwalk;
extern crate serde;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Output {
  Json,
  Table,
}

#[derive(Debug)]
pub enum SortBy {
  Referenced,
  Occurrences,
  None,
}

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

  fn get_modules_filter(&self, cwd: &PathBuf) -> Vec<String>;

  fn get_files(&self, cwd: &PathBuf, include: Vec<String>) -> Vec<String>;
}

pub fn to_final_inspects(inspects: Vec<Inspect>) -> Vec<FinalInspect> {
  let mut merged_map: HashMap<(String, String), (usize, usize, Vec<String>)> =
    HashMap::new();

  for inspect in inspects {
    let key = (
      inspect.raw.specifier.clone(),
      inspect.raw.module_name.clone(),
    );
    let (total_referenced, total_occurrences, aliases) =
      merged_map.entry(key).or_insert((0, 0, Vec::new()));
    *total_referenced += inspect.referenced;
    *total_occurrences += inspect.occurrences;
    if inspect.raw.local_specifier != inspect.raw.specifier
      && !aliases.contains(&inspect.raw.local_specifier)
    {
      aliases.push(inspect.raw.local_specifier.clone());
    }
  }

  let final_inspects: Vec<FinalInspect> = merged_map
    .into_iter()
    .map(
      |((specifier, module_name), (referenced, occurrences, aliases))| {
        FinalInspect {
          specifier,
          aliases,
          module_name,
          referenced,
          occurrences,
        }
      },
    )
    .collect();

  final_inspects
}

pub fn sort_by(inspects: Vec<FinalInspect>, by: SortBy) -> Vec<FinalInspect> {
  match by {
    SortBy::Referenced => sort_by_referenced(inspects),
    SortBy::Occurrences => sort_by_occurrences(inspects),
    SortBy::None => inspects,
  }
}

fn sort_by_occurrences(inspects: Vec<FinalInspect>) -> Vec<FinalInspect> {
  let mut sorted_inspects = inspects;
  sorted_inspects.sort_by_key(|inspect| std::cmp::Reverse(inspect.occurrences));
  sorted_inspects
}

fn sort_by_referenced(inspects: Vec<FinalInspect>) -> Vec<FinalInspect> {
  let mut sorted_inspects = inspects;
  sorted_inspects.sort_by_key(|inspect| std::cmp::Reverse(inspect.referenced));
  sorted_inspects
}

pub fn glob(cwd: &PathBuf, patterns: Vec<String>) -> Vec<String> {
  let mut paths: Vec<String> = Vec::new();

  let glob_paths: Vec<globwalk::DirEntry> =
    globwalk::GlobWalkerBuilder::from_patterns(cwd.clone(), &patterns)
      .build()
      .unwrap()
      .into_iter()
      .filter_map(Result::ok)
      .collect();

  for path in glob_paths {
    if let Some(pathname) = path.path().to_str() {
      paths.push(pathname.to_string());
    }
  }
  paths
}
