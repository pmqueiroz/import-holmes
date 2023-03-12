import { parseSync } from '@swc/core'

type ParsableTypes = string | number | Array<unknown>

export class MockAstGenerator {
  private code: string[]

  constructor() {
    this.code = []
  }

  private parseArgument(arg: ParsableTypes) {
    return typeof arg === 'string' ? `'${arg}'` : arg
  }

  public addImportDeclaration(specifier: string, module: string) {
    this.code.push(`import { ${specifier} } from '${module}'`)
    return this
  }

  public addAssignment(identifier: string, value: ParsableTypes) {
    this.code.push(`const ${identifier} = ${this.parseArgument(value)}`)
    return this
  }

  public addFunctionCall(fn: string, ...args: ParsableTypes[]) {
    this.code.push(`${fn}(${(args || []).map(this.parseArgument).join(',')})`)
    return this
  }

  public compile() {
    const finalCode = this.code.join('\n')

    return parseSync(finalCode)
  }
}
