import { InspectError } from './inspect-error'

const randomError = new Error('some error')

describe('InspectError', () => {
  const inspectError = new InspectError({ filename: 'foo.bar', error: randomError })

  it('should be instance of error', () => {
    expect(inspectError).toBeInstanceOf(Error)
  })

  it('should has filename and error property', () => {
    expect(inspectError).toHaveProperty('filename', expect.any(String))
    expect(inspectError).toHaveProperty('error', expect.any(Error))
  })
})
