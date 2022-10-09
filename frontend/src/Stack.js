import { StackSquare } from './StackSquare.js'
import { InventoryPiece} from './InventoryPiece.js'

const boardStyle = {
  width: '100%',
  height: '100px',
  display: 'flex',
  border: '1px solid gray',
}

const squareStyle = { width: '100%', height: '100%' }

export const Stack = ({ game, player }) => {
  function renderSquare(i) {
    return (
      <div key={i} style={squareStyle}>
        <StackSquare game={game} player={player} >
          <InventoryPiece player={player} game={game} currentIndex={i} />
        </StackSquare>
      </div>
    )
  }
  const squares = []
  for (let i = 0; i < 3; i += 1) {
    squares.push(renderSquare(i))
  }
  return <div style={boardStyle}>{squares}</div>
}
