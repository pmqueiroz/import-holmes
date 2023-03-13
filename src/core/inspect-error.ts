interface InspectErrorOptions {
  filename: string
  error: Error
}

export class InspectError extends Error {
  public filename: string
  public error: Error

  constructor(options: InspectErrorOptions) {
    super(`error while parsing file ${options.filename}`)
    this.filename = options.filename
    this.error = options.error
  }

  public static isInspectError(error: unknown): error is InspectError {
    return error instanceof InspectError
  }
}
