import { inspectModule } from './core/inspect-module'

const mockEntry = `\
import a from 'b'
import { c } from 'd'
import { e as f } from 'g'
`

;(async () => {
   const result = await inspectModule(mockEntry)

   console.log(result)
})()
