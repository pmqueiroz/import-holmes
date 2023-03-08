export interface ImportStatement {
   moduleName: string
   specifier: string
}

export interface ParseModuleOptions {
   modulesFilter?: string | string[]
   specifiersFilter?: string | string[]
}
