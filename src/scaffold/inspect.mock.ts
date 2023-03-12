import { ImportHolmesInspect } from 'src/types'

export const inspectMock: ImportHolmesInspect[] = [
  {
    moduleName: 'foo',
    specifier: 'bar',
    referenced: 1
  },
  {
    moduleName: 'baz',
    specifier: 'bar',
    referenced: 1
  }
]
