export interface ImportHolmesInspect {
  moduleName: string
  specifier: string
}

export interface ImportHolmesInspectWithOccur extends ImportHolmesInspect {
  occurrences: number
}

export type PrintModule = {
  info: (message: any) => void
  error: (message: any) => void
}

export interface ParseModuleOptions {
  modulesFilter?: string | string[]
  specifiersFilter?: string | string[]
  print?: PrintModule
}
