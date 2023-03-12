import { withOccurrences } from './with-occurrences'
import { inspectMock } from '../scaffold/inspect.mock'

describe('withOccurrences', () => {
  it('should be a function', () => {
    expect(withOccurrences).toBeInstanceOf(Function)
  })

  it('should dedup inspects and count occurs', () => {
    const repeatedInspects = [inspectMock.slice(0, 2), inspectMock.slice(0, 2)].flat()

    expect(withOccurrences(repeatedInspects)).toEqual(
      expect.arrayContaining([
        {
          ...inspectMock[0],
          occurrences: 2,
          referenced: inspectMock[0].referenced + inspectMock[1].referenced
        },
        {
          ...inspectMock[1],
          occurrences: 2,
          referenced: inspectMock[0].referenced + inspectMock[1].referenced
        }
      ])
    )
  })
})
