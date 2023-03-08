export interface ImportStatement {
   moduleName: string
   specifier: string
}

export interface ImportStatementWithOccur extends ImportStatement {
   occurrences: number
}

export interface ParseModuleOptions {
   modulesFilter?: string | string[]
   specifiersFilter?: string | string[]
}
