use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;
use std::collections::HashMap;
use glob::{glob_with, MatchOptions};

#[derive(Deserialize, Serialize, Debug)]
pub struct Package {
   pub dependencies: HashMap<String, String>,
   #[serde(rename = "devDependencies")]
   pub dev_dependencies: HashMap<String, String>
}

pub fn read_package_json(cwd: PathBuf) -> Package {
   let file_path = cwd.join("package.json");

   let file = std::fs::File::open(file_path).expect("Failed to read package.json file");

   let data: Package = serde_json::from_reader(file).expect("Failed to read package.json file");

   data
}

pub fn get_module_files() -> Vec<PathBuf> {
   let options = MatchOptions {
      case_sensitive: true,
      require_literal_separator: true,
      ..Default::default()
   };

   // build glob patterns based on options
   let paths: Vec<PathBuf> = glob_with("**/*.ts", options).unwrap().filter_map(Result::ok).collect();

   paths
}
