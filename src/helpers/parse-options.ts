import {
  InspectCommandOptions,
  InspectCommandFullOptions,
  InspectCommandOptionsShortcuts,
  ConfigFileOptions
} from '../types'

const optionsShortcuts: Record<
  keyof InspectCommandOptionsShortcuts,
  keyof InspectCommandFullOptions
> = {
  s: 'specifier',
  m: 'module',
  g: 'glob'
}

export const parseOptions = (
  options: Partial<InspectCommandOptions>,
  ConfigFileOptions: Partial<ConfigFileOptions> = {}
) => {
  const parsedOptions: Partial<InspectCommandFullOptions> = {}

  for (const key in options) {
    if (Object.keys(optionsShortcuts).includes(key)) {
      const parsedKey = optionsShortcuts[key as keyof InspectCommandOptionsShortcuts]

      parsedOptions[parsedKey] = options[key as keyof InspectCommandOptions]
    } else {
      parsedOptions[key as keyof InspectCommandFullOptions] =
        options[key as keyof InspectCommandOptions]
    }
  }

  return Object.assign(ConfigFileOptions, parsedOptions)
}
