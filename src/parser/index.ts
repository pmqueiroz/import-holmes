import { parse } from 'espree'

import { getImportDeclarations } from './helpers'
import type { ImportStatement } from '../types'
import { ImportSpecifier } from 'estree'

const parseOptions = { 
   ecmaVersion: "latest",
   sourceType: "module",
   ecmaFeatures: { 
      jsx: true
   }
}

export const parseModule = (code: string): ImportStatement[] => {
   const programAst = parse(code, parseOptions)
   const importNodes = getImportDeclarations(programAst)

   return importNodes.reduce((acc, curr) => {
      const statements: ImportStatement[] = curr.specifiers.map(specifier => {
         return {
            specifier: (specifier as ImportSpecifier).imported?.name || specifier.local.name,
            moduleName: String(curr.source.value) || ""
         }
      })

      return [...acc, ...statements]
   }, [] as ImportStatement[])
}
