pub fn fatal(cause: &str, code: Option<i32>) {
  let code = code.unwrap_or(0);
  eprintln!("{}", cause);

  std::process::exit(code);
}
