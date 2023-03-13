import { parse, ParseOptions, ImportDeclaration, NamedImportSpecifier, Module } from '@swc/core'

import { getImportDeclarationNodes } from '../helpers/get-import-declaration-nodes'
import type { ImportHolmesInspect, ParseModuleOptions } from '../types'
import { generateFilters } from '../helpers/generate-filters'
import { implementReferences } from '../helpers/implement-references'
import { InspectError } from './inspect-error'

const defaultParseConfig: ParseOptions = {
  syntax: 'typescript',
  tsx: true
}

const getImportHolmesInspects = (nodes: ImportDeclaration[]) =>
  nodes.reduce((acc, curr) => {
    const statements: Omit<ImportHolmesInspect, 'referenced'>[] = curr.specifiers.map(specifier => {
      return {
        specifier: (specifier as NamedImportSpecifier).imported?.value || specifier.local.value,
        moduleName: String(curr.source.value) || ''
      }
    })

    return [...acc, ...statements]
  }, [] as Omit<ImportHolmesInspect, 'referenced'>[])

export const inspectModule = async (
  code: string,
  { filename = 'unknown', parseConfig, ...restOptions }: ParseModuleOptions = {}
): Promise<ImportHolmesInspect[]> => {
  let programAst: Module
  try {
    programAst = await parse(code, Object.assign(defaultParseConfig, parseConfig))
  } catch (err) {
    const error = err as Error
    throw new InspectError({ filename, error })
  }
  const importNodes = getImportDeclarationNodes(programAst.body)

  const statements = getImportHolmesInspects(importNodes)
  const filters = generateFilters(restOptions)
  const filteredStatements = filters.reduce((acc, currFilter) => currFilter(acc), statements)
  const withReferencesCount = filteredStatements.map(stt => implementReferences(stt, programAst))
  return withReferencesCount
}
