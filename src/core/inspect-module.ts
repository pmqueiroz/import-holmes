import { parse, ParseOptions, ImportDeclaration, NamedImportSpecifier } from '@swc/core'

import { getImportDeclarationNodes } from '../helpers/get-import-declaration-nodes'
import type { ImportHolmesInspect, ParseModuleOptions } from '../types'
import { generateFilters } from '../helpers/generate-filters'

const parseOptions: ParseOptions = {
  syntax: 'typescript',
  tsx: true
}

const getImportHolmesInspects = (nodes: ImportDeclaration[]) =>
  nodes.reduce((acc, curr) => {
    const statements: ImportHolmesInspect[] = curr.specifiers.map(specifier => {
      return {
        specifier: (specifier as NamedImportSpecifier).imported?.value || specifier.local.value,
        moduleName: String(curr.source.value) || ''
      }
    })

    return [...acc, ...statements]
  }, [] as ImportHolmesInspect[])

export const inspectModule = async (
  code: string,
  options: ParseModuleOptions = {}
): Promise<ImportHolmesInspect[]> => {
  const programAst = await parse(code, parseOptions)
  const importNodes = getImportDeclarationNodes(programAst)
  const statements = getImportHolmesInspects(importNodes)
  const filters = generateFilters(options)

  return filters.reduce((acc, currFilter) => {
    return currFilter(acc)
  }, statements)
}
