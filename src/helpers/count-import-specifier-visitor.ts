import { Identifier, TsType } from '@swc/core'
import { Visitor } from '@swc/core/Visitor'

export class CountImportSpecifierVisitor extends Visitor {
  private specifier: string
  public count: number

  constructor(specifierName: string) {
    super()
    this.specifier = specifierName
    /**
     * @disclaimer starts negative to bypass import id itself
     */
    this.count = -1
  }

  visitIdentifier(id: Identifier): Identifier {
    if (id.value === this.specifier) {
      this.count++
    }

    return id
  }

  visitTsType(n: TsType): TsType {
    return n
  }
}
