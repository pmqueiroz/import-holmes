extern crate swc_common;
extern crate swc_ecma_parser;
extern crate swc_ecma_ast;
extern crate swc_ecma_visit;

mod parser;
mod visitor;

pub fn inspect_module(source_code: &str) -> Vec<visitor::Inspect> {
    let program = parser::parse_program(source_code);
    let inspects = visitor::get_program_inspects(program);

    inspects
}
