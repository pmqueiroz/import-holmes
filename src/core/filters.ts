import { ImportStatement, ParseModuleOptions } from "../types"
import curry from 'lodash.curry'

type FilterOptions = Pick<ParseModuleOptions, 'moduleName' | 'specifier'>

const filterByModuleName = curry((moduleName: string, statements: ImportStatement[]) => {
   return statements.filter(statement => statement.moduleName === moduleName)
})

const filterBySpecifier = curry((specifier: string, statements: ImportStatement[]) => {
   return statements.filter(statement => statement.specifier === specifier)
})

/**
 * @todo fix this type
*/
const optionFilterMap: Record<keyof FilterOptions, typeof filterByModuleName> = {
   moduleName: filterByModuleName,
   specifier: filterBySpecifier
}

export const generateFilters = (options: FilterOptions) => {
   return Object.keys(options).map(optKey => {
      const filter = optionFilterMap[optKey as keyof FilterOptions]

      return filter(options[optKey as keyof FilterOptions]!)
   })
}
