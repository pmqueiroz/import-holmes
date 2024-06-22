use clap::Parser;
use inspect_core::SortBy;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::fatal;

const CONFIG_FILE_NAME: &str = ".holmesrc.json";

#[derive(Parser, Debug)]
pub struct Args {
  #[arg(short = 'g', long = "glob")]
  pub glob: Option<String>,
  #[arg(short = 'p', long = "path")]
  pub path: Option<String>,
  #[arg(long = "sort")]
  pub sort_strategy: Option<String>,
  #[arg(short = 'm', long = "module")]
  pub filter_module: Option<String>,
}

impl Args {
  pub fn parse() -> Self {
    <Self as Parser>::parse()
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonConfig {
  module: Option<Vec<String>>,
  specifier: Option<Vec<String>>,
  include: Option<Vec<String>>,
  exclude: Option<Vec<String>>,
  #[serde(rename = "sortStrategy")]
  sort_strategy: Option<String>,
}

#[derive(Debug)]
pub struct Config {
  pub module: Option<Vec<String>>,
  pub specifier: Option<Vec<String>>,
  pub include: Vec<String>,
  pub exclude: Vec<String>,
  pub path: PathBuf,
  pub sort_strategy: SortBy,
}

pub fn get_config() -> Config {
  let args = Args::parse();
  let path = resolve_path(args.path.clone());

  if !path.exists() {
    fatal!("Path {} does not exist", path.display());
  }

  let default_config = get_default_config();
  let config_path = path.join(CONFIG_FILE_NAME);

  if !config_file_exits(&config_path) {
    return apply_args_priority(default_config, args, path);
  }

  let rc_config = read_config(&config_path);
  let resolved_config = merge_configs(default_config, rc_config);

  apply_args_priority(resolved_config, args, path)
}

fn config_file_exits(config_path: &PathBuf) -> bool {
  config_path.exists()
}

fn read_config(config_path: &PathBuf) -> JsonConfig {
  let config_contents =
    fs::read_to_string(config_path).expect("Could not read config file");
  serde_json::from_str(&config_contents).expect("Could not parse config file")
}

fn get_default_config() -> Config {
  Config {
    module: None,
    specifier: None,
    include: vec!["**/*.{ts,tsx}".to_string()],
    exclude: vec![
      "node_modules/**".to_string(),
      "**/*.{spec,test}.{ts,tsx}".to_string(),
      "**/*.d.ts".to_string(),
    ],
    path: PathBuf::from("."),
    sort_strategy: SortBy::None,
  }
}

fn merge_configs(default_config: Config, json_config: JsonConfig) -> Config {
  let json_sort_strategy = resolve_sort_strategy(json_config.sort_strategy);

  Config {
    module: json_config.module.or(default_config.module),
    specifier: json_config.specifier.or(default_config.specifier),
    include: json_config.include.unwrap_or(default_config.include),
    exclude: json_config.exclude.unwrap_or(default_config.exclude),
    path: default_config.path,
    sort_strategy: json_sort_strategy.unwrap_or(default_config.sort_strategy),
  }
}

fn apply_args_priority(config: Config, args: Args, path: PathBuf) -> Config {
  let arg_sort_strategy = resolve_sort_strategy(args.sort_strategy);

  Config {
    include: arg_string_to_vec(args.glob).unwrap_or(config.include),
    module: arg_string_to_vec(args.filter_module).or(config.module),
    specifier: config.specifier,
    exclude: config.exclude,
    path,
    sort_strategy: arg_sort_strategy.unwrap_or(config.sort_strategy),
  }
}

fn arg_string_to_vec(arg: Option<String>) -> Option<Vec<String>> {
  match arg {
    Some(arg_piece) => Some(arg_piece.split(',').map(String::from).collect()),
    None => None,
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
    }
    None => env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
  }
}

fn resolve_sort_strategy(sort_by: Option<String>) -> Option<SortBy> {
  match sort_by {
    Some(ref s) if s.eq_ignore_ascii_case("referenced") => {
      Some(SortBy::Referenced)
    }
    Some(ref s) if s.eq_ignore_ascii_case("occurrences") => {
      Some(SortBy::Occurrences)
    }
    Some(ref s) if s.eq_ignore_ascii_case("none") => Some(SortBy::None),
    _ => None,
  }
}
