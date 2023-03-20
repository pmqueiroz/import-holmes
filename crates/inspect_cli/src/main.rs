use inspect_core::inspect_module;
use clap::Parser;
use std::env;

mod read_module;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'm', long = "module")]
    module: Option<String>,
}

fn main() {
    let args = Args::parse();

    println!("{:#?}", args);

    let cwd = env::current_dir().unwrap();

    let package = read_module::read_package_json(cwd);

    println!("{:#?}", package);
    
    let files = read_module::get_module_files();
    println!("{:#?}", files);

    let source_code = r#"
        import { useState } from 'react'
        import React from 'react'
        import { useRef as useCleiton } from 'react' 

        const cleiton = () => {
            const [catapimbas, cataporas] = useState()
        }
    "#;

    let inspects = inspect_module(source_code);

    println!("{:#?}", inspects);
}
