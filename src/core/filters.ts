import { ImportStatement, ParseModuleOptions } from "../types"
import curry from 'lodash.curry'

type FilterOptions = Pick<ParseModuleOptions, 'modulesFilter' | 'specifiersFilter'>

const filterByModuleNames = curry((modules: string[], statements: ImportStatement[]) => {
   return statements.filter(statement => modules.includes(statement.moduleName))
})

const filterBySpecifiers = curry((specifiers: string[], statements: ImportStatement[]) => {
   return statements.filter(statement => specifiers.includes(statement.specifier))
})

/**
 * @todo fix this type
*/
const optionFilterMap: Record<keyof FilterOptions, typeof filterByModuleNames> = {
   modulesFilter: filterByModuleNames,
   specifiersFilter: filterBySpecifiers
}

export const generateFilters = (options: FilterOptions) => {
   return Object.keys(options).map(optKey => {
      const filter = optionFilterMap[optKey as keyof FilterOptions]
      const entries = [options[optKey as keyof FilterOptions]!].flat()
      
      return filter(entries)
   })
}
