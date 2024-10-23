use crate::InspectSummary;

pub fn inspects(summary: InspectSummary) {
  match serde_json::to_string(&summary) {
    Ok(json) => println!("{}", json),
    Err(e) => eprintln!("Error serializing inspects: {}", e),
  }
}
