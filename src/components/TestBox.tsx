import React from 'react'

function TestBox({ min_width, min_height, grow }: {min_width?: number, min_height?: number, grow?: boolean}) {
  return (
    <div style={{minWidth: min_width, minHeight: min_height, outline: '1px solid black', flexGrow: grow? 1 : undefined}}>TestBox</div>
  )
}

export default TestBox;
