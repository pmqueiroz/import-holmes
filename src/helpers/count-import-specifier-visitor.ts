import { Identifier, JSXClosingElement, TsType } from '@swc/core'
import { Visitor } from '@swc/core/Visitor'

export class CountImportSpecifierVisitor extends Visitor {
  private specifier: string
  public count: number

  constructor(specifierName: string) {
    super()
    this.specifier = specifierName
    this.count = 0
  }

  visitIdentifierReference(id: Identifier): Identifier {
    if (id.value === this.specifier) {
      this.count++
    }

    return id
  }

  visitJSXClosingElement(id: JSXClosingElement | undefined): JSXClosingElement | undefined {
    /**
     * @disclaimer just for add this method here it deduplicates the identifier count on closing JSX
     * */

    return id
  }

  /**
   * @fix there still some types that aren't caught
   */
  visitTsType(id: TsType): TsType {
    if (
      (id.type === 'TsTypeReference' &&
        id.typeName.type === 'Identifier' &&
        id.typeName.value === this.specifier) ||
      (id.type === 'TsIndexedAccessType' &&
        id.objectType.type === 'TsTypeReference' &&
        id.objectType.typeName.type === 'Identifier' &&
        id.objectType.typeName.value === this.specifier)
    ) {
      this.count++
    }

    return id
  }
}
