import { ImportHolmesInspect, ParseModuleOptions } from '../types'
import curry from 'lodash.curry'

type FilterOptions = Pick<ParseModuleOptions, 'modulesFilter' | 'specifiersFilter'>

const filterByModuleNames = curry(
  (modules: string[], statements: Omit<ImportHolmesInspect, 'referenced'>[]) =>
    statements.filter(statement => modules.includes(statement.moduleName))
)

const filterBySpecifiers = curry(
  (specifiers: string[], statements: Omit<ImportHolmesInspect, 'referenced'>[]) =>
    statements.filter(statement => specifiers.includes(statement.specifier))
)

/**
 * @todo fix this type
 */
const optionFilterMap: Record<keyof FilterOptions, typeof filterByModuleNames> = {
  modulesFilter: filterByModuleNames,
  specifiersFilter: filterBySpecifiers
}

export const generateFilters = (
  options: FilterOptions
): ((
  stt: Omit<ImportHolmesInspect, 'referenced'>[]
) => Omit<ImportHolmesInspect, 'referenced'>[])[] => {
  return Object.keys(options)
    .filter(optKey => Boolean(options[optKey as keyof FilterOptions]))
    .map(optKey => {
      const filter = optionFilterMap[optKey as keyof FilterOptions]
      const entries = [options[optKey as keyof FilterOptions]!].flat()

      return filter(entries)
    })
}
