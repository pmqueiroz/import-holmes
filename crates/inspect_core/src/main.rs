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
        import { useRef as useCleiton } from 'react' 

        const cleiton = () => {
            const [catapimbas, cataporas] = useState()
        }
    "#;

    let program = parser::parse_program(source_code);
    let inspects = visitor::get_program_inspects(program);
    
    for inspect in inspects {
        println!("specifier: {} module_name: {} referenced: {}", inspect.raw.specifier, inspect.raw.module_name, inspect.referenced);
    }
}
