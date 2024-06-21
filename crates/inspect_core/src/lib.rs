extern crate swc_common;
extern crate swc_ecma_parser;
extern crate swc_ecma_ast;
extern crate swc_ecma_visit;

use std::collections::HashMap;

mod parser;
mod visitor;

pub use visitor::Inspect;
use visitor::RawInspect;

pub fn inspect_module(source_code: &str) -> Vec<visitor::Inspect> {
    let program = parser::parse_program(source_code);
    let inspects = visitor::get_program_inspects(program);

    inspects
}

pub fn dedupe_inspects(inspects: Vec<Inspect>) -> Vec<Inspect> {
    let mut merged_map: HashMap<(String, String), usize> = HashMap::new();

    for inspect in inspects {
        let key = (inspect.raw.specifier.clone(), inspect.raw.module_name.clone());
        let count = merged_map.entry(key).or_insert(0);
        *count += inspect.referenced;
    }

    let merged_inspects: Vec<Inspect> = merged_map
        .into_iter()
        .map(|((specifier, module_name), referenced)| Inspect {
            raw: RawInspect {
                specifier,
                module_name,
            },
            referenced,
        })
        .collect();

    merged_inspects
}

pub fn sort_by_referenced(inspects: &mut Vec<Inspect>) {
    inspects.sort_by_key(|inspect| inspect.referenced);
}
