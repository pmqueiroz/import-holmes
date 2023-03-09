import { parseModule } from './core'

const mockEntry = `\
import a from 'b'
import { c } from 'd'
import { e as f } from 'g'
`

;(async () => {
   const result = await parseModule(mockEntry)

   console.log(result)
})()
