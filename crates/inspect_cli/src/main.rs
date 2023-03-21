use inspect_core::inspect_module;
use clap::Parser;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::env;
use std::fs;

mod read_module;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'g', long = "glob")]
    glob: Option<String>,
}

fn main() {
    let args = Args::parse();
    let cwd = env::current_dir().unwrap();
    // implement module filter in core
    let _package = read_module::read_package_json(cwd);
    
    let files = read_module::get_module_files(args.glob);

    let inspects: Vec<inspect_core::Inspect> = files
        .par_iter()
        .map(|path| {
            let contents = fs::read_to_string(path)
                .expect("Should have been able to read the file");

            inspect_module(&contents)
        })
        .flatten()
        .collect();


    println!("{:#?}", inspects);
}
