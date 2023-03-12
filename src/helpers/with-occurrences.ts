import { ImportHolmesInspect, ImportHolmesInspectWithOccur } from '../types'

export const withOccurrences = (results: ImportHolmesInspect[]) => {
  return results.reduce((acc, curr) => {
    const repeated = acc.find(
      item => item.moduleName === curr.moduleName && item.specifier === curr.specifier
    )
    if (repeated) {
      const repeatedIndex = acc.indexOf(repeated)
      acc[repeatedIndex] = {
        ...repeated,
        occurrences: repeated.occurrences + 1,
        referenced: repeated.referenced + curr.referenced
      }
      return acc
    }

    return [...acc, { ...curr, occurrences: 1 }]
  }, [] as ImportHolmesInspectWithOccur[])
}
