use inspect_core::FinalInspect;

pub fn inspects(inspects: Vec<FinalInspect>) {
  match serde_json::to_string(&inspects) {
    Ok(json) => println!("{}", json),
    Err(e) => eprintln!("Error serializing inspects: {}", e),
  }
}
