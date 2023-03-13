import { ParserConfig } from '@swc/core'

export interface ImportHolmesInspect {
  moduleName: string
  specifier: string
  referenced: number
}

export interface ImportHolmesInspectWithOccur extends ImportHolmesInspect {
  occurrences: number
}

export interface ParseModuleOptions {
  filename?: string
  modulesFilter?: string | string[]
  specifiersFilter?: string | string[]
  parseConfig?: ParserConfig
}

export type InspectCommandOptionsShortcuts = {
  s: string
  m: string
  g: string
}

export type InspectCommandFullOptions = {
  specifier: string
  module: string
  glob: string
}

export type InspectCommandOptions = InspectCommandOptionsShortcuts & InspectCommandFullOptions

export type ConfigFileOptions = {
  module: string | string[]
  specifier: string | string[]
  glob: string
  globIgnore: string[]
  parseConfig?: ParserConfig
}
