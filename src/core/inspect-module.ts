import { parse, ParseOptions, ImportDeclaration, NamedImportSpecifier, Module } from '@swc/core'

import { getImportDeclarationNodes } from '../helpers/get-import-declaration-nodes'
import type {
  ImportHolmesInspect,
  ImportHolmesInspectReferenced,
  ParseModuleOptions
} from '../types'
import { generateFilters } from '../helpers/generate-filters'
import { implementReferences } from '../helpers/implement-references'

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
  { print = console, fileName, ...restOptions }: ParseModuleOptions = {}
): Promise<ImportHolmesInspectReferenced[]> => {
  let programAst: Module
  try {
    programAst = await parse(code, parseOptions)
  } catch (error) {
    /**
     * @todo track file name
     */
    print.error(`error while parsing file <${fileName || 'x'}>`)
    return []
  }
  const importNodes = getImportDeclarationNodes(programAst)

  const statements = getImportHolmesInspects(importNodes)
  const filters = generateFilters(restOptions)
  const filteredStatements = filters.reduce((acc, currFilter) => currFilter(acc), statements)
  const withReferencesCount = filteredStatements.map(stt => implementReferences(stt, programAst))
  return withReferencesCount
}
