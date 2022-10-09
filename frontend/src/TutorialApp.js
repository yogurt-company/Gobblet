import { useMemo } from 'react'
import { Board } from './Board.js'
import { Stack } from './Stack.js'
import { Game } from './Game.js'
const containerStyle = {
  width: 300,
  height: '100%',
}

export const TutorialApp = () => {
  const game = useMemo(() => new Game(), [])
  return (
    <div style={containerStyle}>
      <Stack game={game} player="red" />
      <Board game={game} />
      <Stack game={game} player="green" />
    </div>
  )
}
