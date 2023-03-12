import { InspectCommandOptions, ConfigFileOptions } from '../types'
import { parseOptions } from './parse-options'

const mockCliOptions: Partial<InspectCommandOptions> = {
  g: 'glob',
  m: 'module',
  specifier: 'specifier'
}
const mockConfigFileOptions: Partial<ConfigFileOptions> = { glob: 'foo', globIgnore: ['baz'] }

describe('parseOptions', () => {
  it('should be a function', () => {
    expect(parseOptions).toBeInstanceOf(Function)
  })

  it('should parse shorthand options in full options', () => {
    expect(parseOptions(mockCliOptions)).toEqual(
      expect.objectContaining({ glob: 'glob', module: 'module', specifier: 'specifier' })
    )
  })

  it('should merge with config from file giving preference to cli options', () => {
    expect(parseOptions(mockCliOptions, mockConfigFileOptions)).toEqual(
      expect.objectContaining({
        glob: 'glob',
        module: 'module',
        specifier: 'specifier',
        globIgnore: expect.arrayContaining(['baz'])
      })
    )
  })
})
