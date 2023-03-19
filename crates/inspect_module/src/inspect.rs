use swc_ecma_ast::*;

#[derive(Debug)]
pub struct Inspect {
   pub specifier: String,
   pub module_name: String
}

pub fn generate_inspects(nodes: &[ImportDecl]) -> Vec<Inspect> {
    nodes.iter().fold(Vec::new(), |mut acc, curr| {
        let statements = curr.specifiers.iter().map(|specifier| {
            match specifier {
                ImportSpecifier::Named(named_specifier) => {
                    let imported_value = named_specifier.imported.as_ref().map(|ident| match ident {
                        ModuleExportName::Ident(ident) => {
                            ident.sym.to_string()
                        }
                        ModuleExportName::Str(str) => {
                            str.value.to_string()
                        }
                    })
                            .unwrap_or_else(|| named_specifier.local.sym.to_string());
                    Inspect {
                        specifier: imported_value,
                        module_name: curr.src.value.to_string(),
                    }
                }
                ImportSpecifier::Default(default_specifier) => {
                    Inspect {
                        specifier: default_specifier.local.sym.to_string(),
                        module_name: curr.src.value.to_string(),
                    }
                }
                ImportSpecifier::Namespace(_) => {
                    Inspect {
                        specifier: "*".to_string(),
                        module_name: curr.src.value.to_string(),
                    }
                }
            }
        });
        acc.extend(statements);
        acc
    })
}
