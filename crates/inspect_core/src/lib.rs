extern crate swc_common;
extern crate swc_ecma_parser;
extern crate swc_ecma_ast;
extern crate swc_ecma_visit;

use std::collections::HashMap;

mod parser;
mod visitor;

pub use visitor::Inspect;
use visitor::RawInspect;

pub enum SortBy {
    Referenced,
    Occurrences,
    None
}

pub fn inspect_module(source_code: &str) -> Vec<visitor::Inspect> {
    let program = parser::parse_program(source_code);
    let inspects = visitor::get_program_inspects(program);

    inspects
}

pub fn dedupe_inspects(inspects: Vec<Inspect>) -> Vec<Inspect> {
    let mut merged_map: HashMap<(String, String), (usize, usize)> = HashMap::new();

    for inspect in inspects {
        let key = (
            inspect.raw.specifier.clone(),
            inspect.raw.module_name.clone(),
        );
        let (total_referenced, total_occurrences) = merged_map.entry(key).or_insert((0, 0));
        *total_referenced += inspect.referenced;
        *total_occurrences += inspect.occurrences;
    }

    let merged_inspects: Vec<Inspect> = merged_map
        .into_iter()
        .map(|((specifier, module_name), (referenced, occurrences))| Inspect {
            raw: RawInspect {
                specifier,
                module_name,
            },
            referenced,
            occurrences,
        })
        .collect();

    merged_inspects
}

pub fn sort_by(inspects: Vec<Inspect>, by: SortBy) -> Vec<Inspect> {
    match by {
        SortBy::Referenced => sort_by_referenced(inspects),
        SortBy::Occurrences => sort_by_occurrences(inspects),
        SortBy::None => inspects,
    }
}

fn sort_by_occurrences(inspects: Vec<Inspect>) -> Vec<Inspect> {
    let mut sorted_inspects = inspects;
    sorted_inspects.sort_by_key(|inspect| std::cmp::Reverse(inspect.occurrences));
    sorted_inspects
}

fn sort_by_referenced(inspects: Vec<Inspect>) -> Vec<Inspect> {
    let mut sorted_inspects = inspects;
    sorted_inspects.sort_by_key(|inspect| std::cmp::Reverse(inspect.referenced));
    sorted_inspects
}
