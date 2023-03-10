import { GluegunToolbox } from 'gluegun'
import { glob } from 'glob'
import { inspectModule } from '../core/inspect-module'
import { ImportHolmesInspect, ImportHolmesInspectWithOccur } from '../types'
import groupBy from 'lodash.groupby'

const withOccurrences = (results: ImportHolmesInspect[]) => {
  return results.reduce((acc, curr) => {
    const repeated = acc.find(
      item => item.moduleName === curr.moduleName && item.specifier === curr.specifier
    )
    if (repeated) {
      const repeatedIndex = acc.indexOf(repeated)
      acc[repeatedIndex] = { ...repeated, occurrences: repeated.occurrences + 1 }
      return acc
    }

    return [...acc, { ...curr, occurrences: 1 }]
  }, [] as ImportHolmesInspectWithOccur[])
}

const sortByOccurrences = (a: ImportHolmesInspectWithOccur, b: ImportHolmesInspectWithOccur) => {
  return b.occurrences - a.occurrences
}

const generateTable = (results: ImportHolmesInspectWithOccur[]) => {
  const columnsName = ['Specifier', 'Module', 'Occurrences']

  const byModuleName = Object.values(groupBy(results, statement => statement.moduleName))

  return byModuleName.reduce(
    (acc, curr) => {
      const formattedSpecifiers = curr
        .sort(sortByOccurrences)
        .map(stt => [stt.specifier, stt.moduleName, String(stt.occurrences)])

      return [...acc, ...formattedSpecifiers]
    },
    [columnsName]
  )
}

export default {
  name: 'package',
  alias: 'p',
  run: async ({ print, filesystem }: GluegunToolbox) => {
    const currentProjectPackage = filesystem.read('package.json', 'json')

    if (!currentProjectPackage) throw print.error('no package.json file found.')

    const installedPackages = [
      ...Object.keys(currentProjectPackage.dependencies),
      ...Object.keys(currentProjectPackage.devDependencies)
    ]

    const globFiles = await glob('**/*.{ts,tsx}', {
      ignore: ['node_modules/**', '**/*.{spec,test}.{ts,tsx}', '**/*.d.ts']
    })

    print.info(`Found ${globFiles.length} files... Starting analysis`)
    const spinner = print.spin()

    const analysisErrors = []
    const analysisResult = await Promise.all(
      globFiles.flatMap(file => {
        try {
          return inspectModule(filesystem.read(file) || '', { modulesFilter: installedPackages })
        } catch (error) {
          analysisErrors.push({
            file,
            error
          })

          return []
        }
      })
    )
    const cleanAnalysisResult = analysisResult.flat()
    const resultsWithOccurrences = withOccurrences(cleanAnalysisResult)

    spinner.stop()

    print.table(generateTable(resultsWithOccurrences), { format: 'lean' })

    if (analysisErrors.length) {
      print.error(`got ${analysisErrors.length} errors`)
    }
  }
}
