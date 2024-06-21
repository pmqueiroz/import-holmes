use inspect_core::{inspect_module};
use clap::Parser;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod read_module;
mod log;
mod table;

#[derive(Parser, Debug)]
struct Args{
    #[arg(short = 'g', long = "glob")]
    glob: Option<String>,
    #[arg(short = 'p', long = "path")]
    path: Option<String>,
    #[arg(long = "sort", default_value = "none")]
    sort_strategy: String,
}

fn main() {
    let args = Args::parse();
    let path = resolve_path(args.path);
    let sort_strategy = resolve_sort_strategy(args.sort_strategy);

    if !path.exists() {
        let exit_message = format!("Path {} does not exist", path.display());
        log::fatal(&exit_message, Some(1));
    }

    if !read_module::package_exists(&path) {
        let exit_message = format!("File package.json not found in {} make sure it's a node project", path.display());
        log::fatal(&exit_message, Some(1));
    }

    // implement module filter in core
    let _package = read_module::read_package_json(&path);
    
    let files = read_module::get_module_files(&path, args.glob);

    let inspects: Vec<inspect_core::Inspect> = files
        .par_iter()
        .map(|path| {
            let contents = fs::read_to_string(path)
                .expect("Should have been able to read the file");
            inspect_module(&contents)
        })
        .flatten()
        .collect();

    let deduped = inspect_core::dedupe_inspects(inspects);
    let sorted = inspect_core::sort_by(deduped, sort_strategy);

    table::inspects(sorted);
}

fn resolve_sort_strategy(sort_by: String) -> inspect_core::SortBy {
    match sort_by.as_str() {
        "referenced" => inspect_core::SortBy::Referenced,
        "occurrences" => inspect_core::SortBy::Occurrences,
        "none" => inspect_core::SortBy::None,
        _ => {
            let exit_message = format!("Invalid sort by value: {}", sort_by);
            log::fatal(&exit_message, Some(1));

            inspect_core::SortBy::None
        },
    }
}

fn resolve_path(path_opt: Option<String>) -> PathBuf {
    match path_opt {
        Some(path_str) => {
            let path = Path::new(&path_str);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join(path)
            }
        },
        None => {
            env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        },
    }
}
