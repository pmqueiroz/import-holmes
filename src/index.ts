import { parseModule } from './parser'

const mockEntry = `import { useState } from 'react'`

console.log(parseModule(mockEntry))
