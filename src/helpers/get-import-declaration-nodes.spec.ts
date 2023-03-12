import { MockAstGenerator } from '../scaffold/swc.mock'
import { getImportDeclarationNodes } from './get-import-declaration-nodes'

describe('getImportDeclarationNodes', () => {
  it('should be a function', () => {
    expect(getImportDeclarationNodes).toBeInstanceOf(Function)
  })

  it('should filter module items and return only import declarations', () => {
    const ast = new MockAstGenerator()
      .addAssignment('cleiton', 'salve')
      .addImportDeclaration('foo', 'bar')
      .compile()

    const result = getImportDeclarationNodes(ast.body)

    expect(result).toBeInstanceOf(Array)
    expect(result).toHaveLength(1)
  })
})
