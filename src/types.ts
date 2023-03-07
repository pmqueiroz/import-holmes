export interface ImportStatement {
   moduleName: string
   specifier: string
}

export interface ParseModuleOptions {
   moduleName?: string
   specifier?: string
}
