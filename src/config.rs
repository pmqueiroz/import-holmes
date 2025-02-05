use clap::Parser;
use inspect_typescript::{Output, SortBy};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::fatal;

const CONFIG_FILE_NAME: &str = ".holmesrc.json";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  #[arg(short = 'g', long = "glob", help = "Glob to select files to inspect")]
  pub glob: Option<String>,
  #[arg(short = 'p', long = "path", help = "Path to search files to inspect")]
  pub path: Option<String>,
  #[arg(
    short = 's',
    long = "specifiers",
    help = "Identifiers to filter from imports"
  )]
  pub specifier: Option<String>,
  #[arg(long = "sort", help = "Sort by strategy")]
  pub sort_strategy: Option<String>,
  #[arg(
    short = 'm',
    long = "module",
    help = "Filter inspection by module's name"
  )]
  pub filter_module: Option<String>,
  #[arg(short = 'o', long = "output", help = "Output type")]
  pub output: Option<String>,
}

impl Args {
  pub fn parse() -> Self {
    <Self as Parser>::parse()
  }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonConfig {
  module: Option<Vec<String>>,
  specifiers: Option<Vec<String>>,
  include: Option<Vec<String>>,
  exclude: Option<Vec<String>>,
  #[serde(rename = "sortStrategy")]
  sort_strategy: Option<String>,
  output: Option<String>,
}

#[derive(Debug)]
pub struct Config {
  pub module: Option<Vec<String>>,
  pub specifiers: Option<Vec<String>>,
  pub include: Vec<String>,
  pub exclude: Vec<String>,
  pub path: PathBuf,
  pub sort_strategy: SortBy,
  pub output: Output,
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
    specifiers: None,
    include: vec!["**/*.{ts,tsx}".to_string()],
    exclude: vec![
      "node_modules/**".to_string(),
      "**/*.{spec,test}.{ts,tsx}".to_string(),
      "**/*.d.ts".to_string(),
    ],
    path: PathBuf::from("."),
    sort_strategy: SortBy::None,
    output: Output::Table,
  }
}

fn merge_configs(default_config: Config, json_config: JsonConfig) -> Config {
  let json_sort_strategy = resolve_sort_strategy(json_config.sort_strategy);
  let json_output = resolve_output(json_config.output);

  Config {
    module: json_config.module.or(default_config.module),
    specifiers: json_config.specifiers.or(default_config.specifiers),
    include: json_config.include.unwrap_or(default_config.include),
    exclude: json_config.exclude.unwrap_or(default_config.exclude),
    path: default_config.path,
    sort_strategy: json_sort_strategy.unwrap_or(default_config.sort_strategy),
    output: json_output.unwrap_or(default_config.output),
  }
}

fn apply_args_priority(config: Config, args: Args, path: PathBuf) -> Config {
  let arg_sort_strategy = resolve_sort_strategy(args.sort_strategy);
  let arg_output = resolve_output(args.output);
  let arg_specifiers = resolve_specifiers(args.specifier);

  Config {
    include: arg_string_to_vec(args.glob).unwrap_or(config.include),
    module: arg_string_to_vec(args.filter_module).or(config.module),
    specifiers: arg_specifiers.or(config.specifiers),
    exclude: config.exclude,
    path,
    sort_strategy: arg_sort_strategy.unwrap_or(config.sort_strategy),
    output: arg_output.unwrap_or(config.output),
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

fn resolve_output(output: Option<String>) -> Option<Output> {
  match output {
    Some(ref s) if s.eq_ignore_ascii_case("json") => Some(Output::Json),
    Some(ref s) if s.eq_ignore_ascii_case("table") => Some(Output::Table),
    _ => None,
  }
}

fn resolve_specifiers(
  specifiers_string: Option<String>,
) -> Option<Vec<String>> {
  specifiers_string
    .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
}
