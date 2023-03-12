import { GluegunToolbox } from 'gluegun'
import { glob } from 'glob'
import { inspectModule } from '../core/inspect-module'
import { ImportHolmesInspectWithOccur, InspectCommandOptions } from '../types'
import groupBy from 'lodash.groupby'
import { withOccurrences } from '../helpers/with-occurrences'
import { parseOptions } from '../helpers/parse-options'

const sortByOccurrences = (a: ImportHolmesInspectWithOccur, b: ImportHolmesInspectWithOccur) => {
  return b.occurrences - a.occurrences
}

const generateTable = (results: ImportHolmesInspectWithOccur[]) => {
  const columnsName = ['Specifier', 'Module', 'Occurrences', 'Referenced']

  const byModuleName = Object.values(groupBy(results, statement => statement.moduleName))

  return byModuleName.reduce(
    (acc, curr) => {
      const formattedSpecifiers = curr
        .sort(sortByOccurrences)
        .map(stt => [
          stt.specifier,
          stt.moduleName,
          String(stt.occurrences),
          String(stt.referenced)
        ])

      return [...acc, ...formattedSpecifiers]
    },
    [columnsName]
  )
}

export default {
  name: 'inspect',
  alias: 'i',
  run: async ({ print, filesystem, parameters }: GluegunToolbox) => {
    const options = parseOptions(parameters.options as InspectCommandOptions)
    const currentProjectPackage = filesystem.read('package.json', 'json')

    if (!currentProjectPackage) {
      print.error('no package.json file found.')
      process.exit(1)
    }

    const installedPackages = [
      ...Object.keys(currentProjectPackage.dependencies || []),
      ...Object.keys(currentProjectPackage.devDependencies || [])
    ]

    const globFiles = await glob(options.glob || '**/*.{ts,tsx}', {
      /**
       * @todo add an option to increment these
       */
      ignore: ['node_modules/**', '**/*.{spec,test}.{ts,tsx}', '**/*.d.ts']
    })

    print.info(`Found ${globFiles.length} files... Starting analysis`)
    const spinner = print.spin()

    /**
     * @todo create analysis error handler
     */
    const analysisErrors = []
    const analysisResult = await Promise.all(
      globFiles.flatMap(fileName => {
        try {
          return inspectModule(filesystem.read(fileName) || '', {
            modulesFilter: options.module || installedPackages,
            specifiersFilter: options.specifier,
            fileName,
            print
          })
        } catch (error) {
          analysisErrors.push({
            fileName,
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
      /**
       * @todo add an option to log these erros like --verbose
       */
      print.error(`got ${analysisErrors.length} errors`)
    }
  }
}
