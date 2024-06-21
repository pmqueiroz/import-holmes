use std::collections::HashMap;
use swc_ecma_ast::*;
use swc_ecma_visit::Visit;

#[derive(Debug)]
pub struct RawInspect {
   pub specifier: String,
   pub module_name: String
}

#[derive(Debug)]
pub struct  Inspect {
    pub raw: RawInspect,
    pub referenced: usize
}

#[derive(Debug)]
struct ImportVisitor {
    raw_inspects: Vec<RawInspect>,
    reference_counts: HashMap<String, usize>,
    closing_reference_counts: HashMap<String, usize>,
}

impl Visit for ImportVisitor {
    fn visit_module_item(&mut self, item: &ModuleItem) {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item {
            let inspect = generate_inspect(import_decl);
            self.raw_inspects.extend(inspect);
        }

        swc_ecma_visit::visit_module_item(self, item);
    }

    fn visit_jsx_closing_element(&mut self, closing_jsx: &JSXClosingElement) {
        
        match &closing_jsx.name {
            JSXElementName::Ident(ident) => {
                let ident_name = ident.sym.to_string();

                if let Some(count) = self.closing_reference_counts.get_mut(&ident_name) {
                    *count += 1;
                } else {
                    self.closing_reference_counts.insert(ident_name, 1);
                }
            },
            _ => {
                // Ignore JSXMemberExpr and JSXNamespacedName variants
            }
        }

        swc_ecma_visit::visit_jsx_closing_element(self, closing_jsx);
    }

    fn visit_ident(&mut self, ident: &Ident) {
        if let Some(count) = self.reference_counts.get_mut(&ident.sym.to_string()) {
            *count += 1;
        } else {
            self.reference_counts.insert(ident.sym.to_string(), 1);
        }

        swc_ecma_visit::visit_ident(self, ident);
    }
}

fn generate_inspect(node: &ImportDecl) -> Vec<RawInspect> {
    let mut raw_inspects = Vec::new();

    let statements = node.specifiers.iter().map(|specifier| {
        match specifier {
            ImportSpecifier::Named(named_specifier) => {
                let imported_value = named_specifier.imported.as_ref().map(|ident| {
                    match ident {
                        ModuleExportName::Ident(ident) => {
                            ident.sym.to_string()
                        }
                        ModuleExportName::Str(str) => {
                            str.value.to_string()
                        }
                    }
                })
                        .unwrap_or_else(|| named_specifier.local.sym.to_string());
                RawInspect {
                    specifier: imported_value,
                    module_name: node.src.value.to_string(),
                }
            }
            ImportSpecifier::Default(default_specifier) => {
                RawInspect {
                    specifier: default_specifier.local.sym.to_string(),
                    module_name: node.src.value.to_string(),
                }
            }
            ImportSpecifier::Namespace(_) => {
                RawInspect {
                    specifier: "*".to_string(),
                    module_name: node.src.value.to_string(),
                }
            }
        }
    });

    raw_inspects.extend(statements);

    raw_inspects
}

pub fn get_program_inspects(program: Program) -> Vec<Inspect> {
   let mut visitor = ImportVisitor {
        raw_inspects: Vec::new(),
        reference_counts: HashMap::new(),
        closing_reference_counts: HashMap::new(),
   };

   visitor.visit_program(&program);

   let mut inspects = Vec::new();

   for raw_inspect in visitor.raw_inspects {
        let referenced = visitor.reference_counts.get(&raw_inspect.specifier)
            // subtract 1 to skip self import ident
            .map_or(0, |value| value - 1);

        let real_referenced = referenced - visitor.closing_reference_counts.get(&raw_inspect.specifier)
            .copied()
            .unwrap_or(0);

        inspects.push(
            Inspect {
                raw: raw_inspect,
                referenced: real_referenced
            }
        )
   }

   inspects
}
    