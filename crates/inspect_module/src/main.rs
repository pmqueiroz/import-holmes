extern crate swc_common;
extern crate swc_ecma_parser;
extern crate swc_ecma_ast;
extern crate swc_ecma_visit;

use swc_common::{BytePos};
use swc_ecma_parser::{lexer::Lexer, Parser, TsConfig, Syntax, StringInput};
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit};

struct ImportVisitor {
    imports: Vec<ImportDecl>
}

impl Visit for ImportVisitor {
    fn visit_module_item(&mut self, item: &ModuleItem) {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item {
            self.imports.push(import_decl.clone());
        }

        swc_ecma_visit::visit_module_item(self, item);
    }
}

fn main() {
    let source_code = r#"
        import { useState } from 'react'
        import React from 'react'
    "#;

    let lexer = Lexer::new(
        Syntax::Typescript(TsConfig::default()),
        Default::default(),
        StringInput::new(source_code, BytePos::DUMMY, BytePos::DUMMY),
        None,   
    );

    let mut parser = Parser::new_from(lexer);

    let program = parser.parse_program().expect("Failed to parse program");

    let mut visitor = ImportVisitor {
        imports: Vec::new()
    };

    visitor.visit_program(&program);
    
    println!("Collected imports: {:?}", visitor.imports);
}
