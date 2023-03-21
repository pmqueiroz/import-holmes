extern crate globwalk;

use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;
use std::collections::HashMap;

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

pub fn get_module_files(arg_glob: Option<String>) -> Vec<String> {
   let mut paths: Vec<String> = Vec::new();
   let glob_pattern = arg_glob.unwrap_or("**/*.{ts,tsx}".to_string());

   let glob_paths = globwalk::glob(glob_pattern).unwrap().filter_map(Result::ok);

   for path in glob_paths {
      if let Some(pathname) = path.path().to_str() {
         paths.push(pathname.to_string());
      }
   }

   paths
}
