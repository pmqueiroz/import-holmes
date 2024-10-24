extern crate serde;
extern crate serde_json;
extern crate swc_common;
extern crate swc_ecma_ast;
extern crate swc_ecma_parser;
extern crate swc_ecma_visit;

use std::collections::HashMap;

mod parser;
mod visitor;

pub use visitor::{FinalInspect, Inspect};

#[derive(Debug)]
pub enum SortBy {
  Referenced,
  Occurrences,
  None,
}

#[derive(Debug)]
pub enum Output {
  Json,
  Table,
}

pub fn inspect_module(source_code: &str) -> Vec<visitor::Inspect> {
  let program = parser::parse_program(source_code);
  let inspects = visitor::get_program_inspects(program);

  inspects
}

pub fn get_final_inspects(inspects: Vec<Inspect>) -> Vec<FinalInspect> {
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
