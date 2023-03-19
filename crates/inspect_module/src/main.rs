extern crate swc_common;
extern crate swc_ecma_parser;
extern crate swc_ecma_ast;
extern crate swc_ecma_visit;

mod parser;
mod visitor;

fn main() {
    let source_code = r#"
        import { useState } from 'react'
        import React from 'react'
        import { useState as useCleiton } from 'react'
    "#;

    let program = parser::parse_program(source_code);
    let inspects = visitor::get_program_inspects(program);
    
    for inspect in inspects {
        println!("specifier: {} module_name: {}", inspect.specifier, inspect.module_name);
    }
}
