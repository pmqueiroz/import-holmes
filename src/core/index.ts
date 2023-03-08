import { parse, ParseOptions, ImportDeclaration, NamedImportSpecifier } from '@swc/core'

import { getImportDeclarationsNodes } from './helpers'
import type { ImportStatement, ParseModuleOptions } from '../types'
import { generateFilters } from './filters'

const parseOptions: ParseOptions = { 
   syntax: "typescript",
   tsx: true
}

const getImportStatements = (nodes: ImportDeclaration[]) => nodes.reduce((acc, curr) => {
   const statements: ImportStatement[] = curr.specifiers.map(specifier => {
      return {
         specifier: (specifier as NamedImportSpecifier).imported?.value || specifier.local.value,
         moduleName: String(curr.source.value) || ""
      }
   })

   return [...acc, ...statements]
}, [] as ImportStatement[])


export const parseModule = async (code: string, options: ParseModuleOptions = {}): Promise<ImportStatement[]> => {
   const programAst = await parse(code, parseOptions)
   const importNodes = getImportDeclarationsNodes(programAst)
   const statements = getImportStatements(importNodes)
   const filters = generateFilters(options)

   return filters.reduce((acc, currFilter) => {
      return currFilter(acc)
   }, statements)
}
