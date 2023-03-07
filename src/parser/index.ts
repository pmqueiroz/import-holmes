import { parse } from 'espree'

import { getImportDeclarations } from './helpers'

const parseOptions = { 
   ecmaVersion: "latest",
   sourceType: "module",
   ecmaFeatures: { 
      jsx: true
   }
}

export const parseModule = (code: string) => {
   const programAst = parse(code, parseOptions)

   return getImportDeclarations(programAst)
}
