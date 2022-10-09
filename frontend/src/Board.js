import { useEffect, useState } from 'react'
import { BoardSquare } from './BoardSquare.js'
import { BoardPiece } from './BoardPiece.js'

const boardStyle = {
  width: '100%',
  height: 300,
  display: 'flex',
  flexWrap: 'wrap',
  border: '1px solid gray',
  margin: '15px 0' 
}

const squareStyle = { width: '33.3%', height: '33.3%' }

export const Board = ({ game }) => {

  function renderSquare(i) {
    const x = i % 3
    const y = Math.floor(i / 3)
    return (
      <div key={i} style={squareStyle}>
        <BoardSquare x={x} y={y} game={game}>
          <BoardPiece game={game} x={x} y={y} />
        </BoardSquare>
      </div>
    )
  }
  const squares = []
  for (let i = 0; i < 9; i += 1) {
    squares.push(renderSquare(i))
  }
  return <div style={boardStyle}>{squares}</div>
}
