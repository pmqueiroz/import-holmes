import React, { useState, useRef as useReference } from 'react'
import { Button } from 'design-system'

export const AnotherComponent = () => {
  const [state, setState] = useState()

  const ref = useReference()

  return (
    <Button ref={ref} onClick={setState(state + 1)}>
      <Button />

      {state}
    </Button>
  )
}
