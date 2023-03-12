import { parseSync } from '@swc/core'

export class MockAstGenerator {
  private code: string[]

  constructor() {
    this.code = []
  }

  public addImportDeclaration(specifier: string, module: string) {
    this.code.push(`import { ${specifier} } from '${module}'`)
    return this
  }

  public addAssignment(identifier: string, value: unknown) {
    if (typeof value === 'string') value = `'${value}'`

    this.code.push(`const ${identifier} = ${value}`)
    return this
  }

  public compile() {
    const finalCode = this.code.join('\n')

    return parseSync(finalCode)
  }
}
