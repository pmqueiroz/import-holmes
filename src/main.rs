use inspect_core::{inspect_module, FinalInspect, Output};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::fs;

mod config;
#[macro_use]
mod log;
mod json;
mod read_project;
mod table;

fn main() {
  let config = config::get_config();
  let package = read_project::read_package_json(&config.path);
  let dependencies = read_project::get_dependencies(&package);
  let modules_filter = config.module.clone().unwrap_or(dependencies);

  let files = read_project::get_module_files(&config.path, config.include);

  let inspects: Vec<inspect_core::Inspect> = files
    .par_iter()
    .map(|path| {
      let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
      inspect_module(&contents)
    })
    .flatten()
    .filter(|inspect| modules_filter.contains(&inspect.raw.module_name))
    .collect();

  let inspects: Vec<inspect_core::Inspect> =
    if let Some(specifiers) = &config.specifiers {
      inspects
        .into_iter()
        .filter(|inspect| specifiers.contains(&inspect.raw.specifier))
        .collect()
    } else {
      inspects
    };

  let final_inspects = inspect_core::get_final_inspects(inspects);
  let sorted = inspect_core::sort_by(final_inspects, config.sort_strategy);

  output_result(sorted, config.output);
}

fn output_result(inspects: Vec<FinalInspect>, output: Output) {
  match output {
    Output::Json => json::inspects(inspects),
    Output::Table => table::inspects(inspects),
  }
}
