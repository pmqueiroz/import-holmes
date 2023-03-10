import { build } from 'gluegun'
import defaultCommand from './commands/inspect'

export const run = (argv: NodeJS.Process['argv']) => {
  const cli = build('import-holmes')
    .src(__dirname)
    .defaultCommand(defaultCommand)
    .help()
    .version()
    .exclude(['semver', 'http', 'template', 'patching'])
    .create()

  cli.run(argv)
}
