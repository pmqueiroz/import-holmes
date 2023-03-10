import { Module, ModuleItem, ImportDeclaration } from '@swc/core'

const importDeclarationFilter = (node: ModuleItem) => node.type === 'ImportDeclaration'

export const getImportDeclarationsNodes = (program: Module) => {
  return program.body.filter(importDeclarationFilter) as ImportDeclaration[]
}
