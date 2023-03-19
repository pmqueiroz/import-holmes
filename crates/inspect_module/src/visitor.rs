use swc_ecma_ast::*;
use swc_ecma_visit::Visit;

struct ImportVisitor {
    imports: Vec<ImportDecl>
}

impl Visit for ImportVisitor {
    fn visit_module_item(&mut self, item: &ModuleItem) {
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = item {
            self.imports.push(import_decl.clone());
        }

        swc_ecma_visit::visit_module_item(self, item);
    }
}


pub fn get_import_decl(program: Program) -> Vec<ImportDecl> {
   let mut visitor = ImportVisitor {
      imports: Vec::new()
  };

  visitor.visit_program(&program);

  return visitor.imports;
}
