import { parse } from 'espree'

import { getImportDeclarationsNodes } from './helpers'
import type { ImportStatement, ParseModuleOptions } from '../types'
import { ImportDeclaration, ImportSpecifier } from 'estree'
import { generateFilters } from './filters'

const parseOptions = { 
   ecmaVersion: "latest",
   sourceType: "module",
   ecmaFeatures: { 
      jsx: true
   }
}

const getImportStatements = (nodes: ImportDeclaration[]) => nodes.reduce((acc, curr) => {
   const statements: ImportStatement[] = curr.specifiers.map(specifier => {
      return {
         specifier: (specifier as ImportSpecifier).imported?.name || specifier.local.name,
         moduleName: String(curr.source.value) || ""
      }
   })

   return [...acc, ...statements]
}, [] as ImportStatement[])


export const parseModule = (code: string, options: ParseModuleOptions = {}): ImportStatement[] => {
   const programAst = parse(code, parseOptions)
   const importNodes = getImportDeclarationsNodes(programAst)
   const statements = getImportStatements(importNodes)
   const filters = generateFilters(options)

   return filters.reduce((acc, currFilter) => {
      return currFilter(acc)
   }, statements)
}
