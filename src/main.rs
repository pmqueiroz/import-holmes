use inspect_core::inspect_module;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::fs;

mod config;
mod log;
mod read_module;
mod table;

fn main() {
  let config = config::get_config();

  if !read_module::package_exists(&config.path) {
    let exit_message = format!(
      "File package.json not found in {} make sure it's a node project",
      config.path.display()
    );
    log::fatal(&exit_message, Some(1));
  }

  let package = read_module::read_package_json(&config.path);
  let files = read_module::get_module_files(&config.path, config.include);
  let dependencies = package
    .dependencies
    .keys()
    .cloned()
    .collect::<Vec<String>>();
  let modules_filter = config.module.clone().unwrap_or(dependencies);

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

  let deduped = inspect_core::dedupe_inspects(inspects);
  let sorted = inspect_core::sort_by(deduped, config.sort_strategy);

  table::inspects(sorted);
}
