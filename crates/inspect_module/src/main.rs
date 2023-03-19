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
    "#;

    let program = parser::parse_program(source_code);
    let imports = visitor::get_import_decl(program);
    
    println!("Collected imports: {:?}", imports);
}
