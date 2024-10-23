use inspect_core::FinalInspect;
use prettytable::{color, format, Attr, Cell, Row, Table};

pub fn inspects(inspects: Vec<FinalInspect>) {
  let mut table = Table::new();

  table.set_format(get_unicode_format());

  let titles = vec![
    "Specifier",
    "Aliases",
    "Module",
    "Occurrences",
    "Referenced",
  ];

  table.set_titles(generate_title_row(&titles));

  for inspect in &inspects {
    table.add_row(Row::new(vec![
      Cell::new(&inspect.specifier),
      Cell::new(&if inspect.aliases.is_empty() {
        "-".to_string()
      } else {
        inspect.aliases.join(", ")
      }),
      Cell::new(&inspect.module_name),
      Cell::new(&inspect.occurrences.to_string()),
      Cell::new(&inspect.referenced.to_string()),
    ]));
  }

  table.printstd();
}

fn generate_title_row(titles: &[&str]) -> Row {
  let title_cells = titles
    .iter()
    .map(|&title| {
      Cell::new(title)
        .with_style(Attr::Bold)
        .with_style(Attr::ForegroundColor(color::BLUE))
    })
    .collect();

  Row::new(title_cells)
}

fn get_unicode_format() -> format::TableFormat {
  format::FormatBuilder::new()
    .column_separator('│')
    .borders('│')
    .separators(
      &[format::LinePosition::Top],
      format::LineSeparator::new('─', '┬', '┌', '┐'),
    )
    .separators(
      &[format::LinePosition::Intern],
      format::LineSeparator::new('─', '┼', '├', '┤'),
    )
    .separators(
      &[format::LinePosition::Bottom],
      format::LineSeparator::new('─', '┴', '└', '┘'),
    )
    .padding(1, 1)
    .build()
}
