import { implementReferences } from './implement-references'
import { MockAstGenerator } from '../scaffold/swc.mock'

describe('implementReferences', () => {
  it('should be a function', () => {
    expect(implementReferences).toBeInstanceOf(Function)
  })

  it('should visit ast and count references', () => {
    const ast = new MockAstGenerator()
      .addImportDeclaration('foo', 'bar')
      .addFunctionCall('foo', 1, [])
      .addFunctionCall('foo', 'salve')
      .addFunctionCall('foo')
      .compile()

    const mockInspect = { specifier: 'foo', moduleName: 'bar' }

    expect(implementReferences(mockInspect, ast)).toEqual(
      expect.objectContaining({ ...mockInspect, referenced: 3 })
    )
  })

  it.todo('should not consider blocked identifier with the same value as the import specifier')
})
