export interface ImportHolmesInspect {
   moduleName: string
   specifier: string
}

export interface ImportHolmesInspectWithOccur extends ImportHolmesInspect {
   occurrences: number
}

export interface ParseModuleOptions {
   modulesFilter?: string | string[]
   specifiersFilter?: string | string[]
}
