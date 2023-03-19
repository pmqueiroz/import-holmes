extern crate swc_common;
extern crate swc_ecma_parser;
extern crate swc_ecma_ast;
extern crate swc_ecma_visit;

mod parser;
mod visitor;
mod inspect;

fn main() {
    let source_code = r#"
        import { useState } from 'react'
        import React from 'react'
        import { useState as useCleiton } from 'react'
    "#;

    let program = parser::parse_program(source_code);
    let imports = visitor::get_import_decl(program);
    let inspects = inspect::generate_inspects(&imports);
    
    for inspect in inspects {
        println!("specifier: {} module_name: {}", inspect.specifier, inspect.module_name);
    }
}
