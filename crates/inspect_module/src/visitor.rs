use swc_ecma_ast::*;
use swc_ecma_visit::Visit;

#[derive(Debug)]
pub struct Inspect {
   pub specifier: String,
   pub module_name: String
}

struct ImportVisitor {
    inspects: Vec<Inspect>
}

impl Visit for ImportVisitor {
    fn visit_module_item(&mut self, item: &ModuleItem) {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item {
            let inspect = generate_inspect(import_decl);
            self.inspects.extend(inspect);
        }

        swc_ecma_visit::visit_module_item(self, item);
    }
}

fn generate_inspect(node: &ImportDecl) -> Vec<Inspect> {
    let mut inspects = Vec::new();

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
                Inspect {
                    specifier: imported_value,
                    module_name: node.src.value.to_string(),
                }
            }
            ImportSpecifier::Default(default_specifier) => {
                Inspect {
                    specifier: default_specifier.local.sym.to_string(),
                    module_name: node.src.value.to_string(),
                }
            }
            ImportSpecifier::Namespace(_) => {
                Inspect {
                    specifier: "*".to_string(),
                    module_name: node.src.value.to_string(),
                }
            }
        }
    });

    inspects.extend(statements);

    inspects
}

pub fn get_program_inspects(program: Program) -> Vec<Inspect> {
   let mut visitor = ImportVisitor {
    inspects: Vec::new()
   };

   visitor.visit_program(&program);

   return visitor.inspects;
}
