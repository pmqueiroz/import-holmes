use inspect_core::Inspect;

pub fn inspects(inspects: Vec<Inspect>) {
  match serde_json::to_string(&inspects) {
    Ok(json) => println!("{}", json),
    Err(e) => eprintln!("Error serializing inspects: {}", e),
  }
}
