import type { Node, Program, ImportDeclaration } from 'estree'

const importDeclarationFilter = (node: Node) => node.type === 'ImportDeclaration'

export const getImportDeclarationsNodes = (program: Program) => {
   return program.body.filter(importDeclarationFilter) as ImportDeclaration[]
}
