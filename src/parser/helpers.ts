import type { Node, Program } from 'estree'

const importDeclarationFilter = (node: Node) => node.type === 'ImportDeclaration'

export const getImportDeclarations = (program: Program) => {
   return program.body.filter(importDeclarationFilter)
}
