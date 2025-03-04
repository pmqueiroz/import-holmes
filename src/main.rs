use core::{
  sort_by, to_final_inspects, FinalInspect, Inspect, Inspector, Language,
  Output,
};
use inspect_kotlin::KotlinInspector;
use inspect_typescript::TypescriptInspector;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

mod config;
#[macro_use]
mod log;
mod csv;
mod json;
mod table;

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectSummary {
  inspects: Vec<FinalInspect>,
  total_files_count: usize,
  total_imports_count: usize,
  unique_imports_count: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
  let config = config::get_config();

  let language = Language::TypeScript;

  let inspectors: HashMap<Language, Box<dyn Inspector>> = vec![
    (
      Language::TypeScript,
      Box::new(TypescriptInspector) as Box<dyn Inspector>,
    ),
    (
      Language::Kotlin,
      Box::new(KotlinInspector) as Box<dyn Inspector>,
    ),
  ]
  .into_iter()
  .collect();

  let inspector = inspectors
    .get(&config.language)
    .ok_or_else(|| format!("config.language '{}' not supported.", language))?;

  let dependencies = inspector.get_modules_filter(&config.path);
  let modules_filter = config.module.clone().unwrap_or(dependencies);

  let files = inspector.get_files(&config.path, config.include);
  let total_files_count = files.iter().count();

  let inspects: Vec<Inspect> = files
    .par_iter()
    .map(|path| {
      let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

      inspector.inspect(contents)
    })
    .flatten()
    .filter(|inspect| {
      modules_filter == ["*"]
        || modules_filter
          .iter()
          .any(|filter| inspect.raw.module_name.starts_with(filter))
    })
    .collect();

  let inspects: Vec<Inspect> = if let Some(specifiers) = &config.specifiers {
    inspects
      .into_iter()
      .filter(|inspect| specifiers.contains(&inspect.raw.specifier))
      .collect()
  } else {
    inspects
  };

  let total_imports_count = inspects.iter().count();

  let final_inspects = to_final_inspects(inspects);
  let unique_imports_count = final_inspects.iter().count();
  let sorted = sort_by(final_inspects, config.sort_strategy);

  let summary = InspectSummary {
    inspects: sorted,
    total_files_count,
    total_imports_count,
    unique_imports_count,
  };

  output_result(summary, config.output);

  Ok(())
}

fn output_result(summary: InspectSummary, output: Output) {
  match output {
    Output::Json => json::inspects(summary),
    Output::Table => table::inspects(summary),
    Output::Csv => csv::inspects(summary),
  }
}
