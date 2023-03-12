import { Program } from '@swc/core'
import { ImportHolmesInspect } from '../types'
import { CountImportSpecifierVisitor } from './count-import-specifier-visitor'

export const implementReferences = (
  inspect: Omit<ImportHolmesInspect, 'referenced'>,
  ast: Program
): ImportHolmesInspect => {
  const visitor = new CountImportSpecifierVisitor(inspect.specifier)
  visitor.visitProgram(ast)

  return { ...inspect, referenced: visitor.count }
}
