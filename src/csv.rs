use crate::InspectSummary;

pub fn inspects(summary: InspectSummary) {
  let headers = vec![
    "Specifier",
    "Aliases",
    "Module",
    "Occurrences",
    "Referenced",
  ];
  println!("{}", headers.join(","));

  for inspect in &summary.inspects {
    let aliases = if inspect.aliases.is_empty() {
      "-".to_string()
    } else {
      inspect.aliases.join(", ")
    };

    let row = vec![
      escape_csv_field(&inspect.specifier),
      escape_csv_field(&aliases),
      escape_csv_field(&inspect.module_name),
      escape_csv_field(&inspect.occurrences.to_string()),
      escape_csv_field(&inspect.referenced.to_string()),
    ];

    println!("{}", row.join(","));
  }
}

fn escape_csv_field(field: &str) -> String {
  if field.contains(',') || field.contains('"') || field.contains('\n') {
    let escaped = field.replace('"', "\"\"");
    format!("\"{}\"", escaped)
  } else {
    field.to_string()
  }
}
