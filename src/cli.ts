import { build } from 'gluegun'

export const run = (argv: NodeJS.Process['argv']) => {
   const cli = build('import-analyzer')
      .src(__dirname)
      .help()
      .version()
      .exclude(['semver', 'http', 'template', 'patching'])
      .create()

   cli.run(argv)
}
