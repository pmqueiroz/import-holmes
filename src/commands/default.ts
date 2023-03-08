import { GluegunToolbox } from 'gluegun'
import { glob } from 'glob'
import { parseModule } from '../core'

export default {
   name: 'package',
   alias: 'p',
   run: async ({ print, filesystem }: GluegunToolbox) => {
      const currentProjectPackage = filesystem.read('package.json', 'json')

      if (!currentProjectPackage) throw print.error('no package.json file found.')

      const installedPackages = [...Object.keys(currentProjectPackage.dependencies), ...Object.keys(currentProjectPackage.devDependencies)]

      const globFiles = await glob('**/*.{ts,tsx}', {
         ignore: ['node_modules/**', '**/*.{spec,test}.{ts,tsx}', '**/*.d.ts']
      })

      print.info(`Found ${globFiles.length} files... Starting analysis`)

      const analysisErrors = []
      const analysisResult = globFiles.flatMap(file => {
         try {
            return parseModule(filesystem.read(file) || '', { modulesFilter: installedPackages })
         } catch (error) {
            analysisErrors.push({ 
               file,
               error
            })

            return []
         }
      })

      print.info(analysisResult)
      print.error(`got ${analysisErrors.length} errors`)
   }
}
