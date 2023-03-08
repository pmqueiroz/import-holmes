import { GluegunToolbox } from 'gluegun'
import { glob } from 'glob'
import { parseModule } from '../core'

export default {
   name: 'package',
   alias: 'p',
   run: async ({ print, filesystem }: GluegunToolbox) => {
      const globFiles = await glob('**/*.{ts,tsx}', {
         ignore: ['node_modules/**', '**/*.{spec,test}.{ts,tsx}', '**/*.d.ts']
      })

      const errors = []

      print.info(`Found ${globFiles.length} files... Starting analysis`)

      const analysisResult = globFiles.flatMap(file => {
         try {
            return parseModule(filesystem.read(file) || '')
         } catch (error) {
            errors.push({ 
               file,
               error
            })

            return []
         }
      })

      print.info(analysisResult)
      print.error(`got ${errors.length} errors`)
   }
}
