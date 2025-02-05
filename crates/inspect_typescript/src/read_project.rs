extern crate globwalk;

use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
  pub dependencies: HashMap<String, String>,
  #[serde(rename = "devDependencies")]
  pub dev_dependencies: HashMap<String, String>,
}

pub fn read_package_json(cwd: &PathBuf) -> Package {
  let file_path = cwd.join("package.json");

  if !file_path.exists() {
    eprintln!(
      "File package.json not found in {} make sure it's a node project",
      cwd.display()
    );
    std::process::exit(1);
  }

  let file =
    std::fs::File::open(file_path).expect("Failed to read package.json file");

  let data: Package =
    serde_json::from_reader(file).expect("Failed to read package.json file");

  data
}

pub fn get_module_files(cwd: &PathBuf, include: Vec<String>) -> Vec<String> {
  let mut paths: Vec<String> = Vec::new();
  let mut patterns = include.clone();

  let mut ignore_patterns = vec![
    "node_modules/*".to_string(),
    "**/*.{spec,test}.{ts,tsx}".to_string(),
    "**/*.d.ts".to_string(),
  ];

  ignore_patterns.iter_mut().for_each(|s| s.insert(0, '!'));

  patterns.extend(ignore_patterns);

  let glob_paths: Vec<globwalk::DirEntry> =
    globwalk::GlobWalkerBuilder::from_patterns(cwd.clone(), &patterns)
      .build()
      .unwrap()
      .into_iter()
      .filter_map(Result::ok)
      .collect();

  for path in glob_paths {
    if let Some(pathname) = path.path().to_str() {
      paths.push(pathname.to_string());
    }
  }

  paths
}

pub fn get_dependencies(package: &Package) -> Vec<String> {
  let dependencies = package
    .dependencies
    .keys()
    .cloned()
    .collect::<Vec<String>>();

  dependencies
}
