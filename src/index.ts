import { parseModule } from './core'

const mockEntry = `
import { useState, useRef, useContext as cleiton } from 'react'

import React from 'react'

import { Button, IconText } from '@dlpco/ginga-stone'
`

;(async () => {
   const result = await parseModule(mockEntry, {  modulesFilter: 'react' })

   console.log(result)
})()
