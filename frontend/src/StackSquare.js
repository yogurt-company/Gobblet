import { useDrop } from 'react-dnd'
import { ItemTypes } from './ItemTypes.js'
import { Overlay, OverlayType } from './Overlay.js'
import { Square } from './Square.js'
export const StackSquare = ({ player, children, game }) => {
  const [{ isOver, canDrop }, drop] = useDrop(
    () => ({
      accept: ItemTypes.KNIGHT,
      canDrop: () => game.canDropTarget({}, 0, 0),
      // drop: () => game.moveKnight(0, 0),
      collect: (monitor) => ({
        isOver: !!monitor.isOver(),
        canDrop: !!monitor.canDrop(),
      }),
    }),
    [game],
  )

  return (
    <div
      ref={drop}
      role="Space"
      style={{
        position: 'relative',
        width: '100%',
        height: '100%',
      }}
    >
      <Square colorProp={player} game={game}>{children}</Square>
      {isOver && !canDrop && <Overlay type={OverlayType.IllegalMoveHover} />}
      {!isOver && canDrop && <Overlay type={OverlayType.PossibleMove} />}
      {isOver && canDrop && <Overlay type={OverlayType.LegalMoveHover} />}
    </div>
  )
}
