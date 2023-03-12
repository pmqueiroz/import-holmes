import { ModuleItem, ImportDeclaration } from '@swc/core'

const importDeclarationFilter = (node: ModuleItem): node is ImportDeclaration =>
  node.type === 'ImportDeclaration'

export const getImportDeclarationNodes = (nodes: ModuleItem[]) => {
  return nodes.filter(importDeclarationFilter)
}
