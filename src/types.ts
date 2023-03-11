export interface ImportHolmesInspect {
  moduleName: string
  specifier: string
}

export interface ImportHolmesInspectReferenced extends ImportHolmesInspect {
  referenced: number
}

export interface ImportHolmesInspectWithOccur extends ImportHolmesInspectReferenced {
  occurrences: number
}

export type PrintModule = {
  info: (message: unknown) => void
  error: (message: unknown) => void
}

export interface ParseModuleOptions {
  modulesFilter?: string | string[]
  specifiersFilter?: string | string[]
  print?: PrintModule
}
