import { useDrop } from 'react-dnd'
import { ItemTypes } from './ItemTypes.js'
import { Overlay, OverlayType } from './Overlay.js'
import { Square } from './Square.js'
export const BoardSquare = ({ x, y, children, game }) => {
  const [{ isOver, canDrop }, drop] = useDrop(
    () => ({
      accept: ItemTypes.KNIGHT,
      canDrop: (item) => game.canDropTarget(item, x, y),
      drop: (item) => game.whenDropTarget(item, x, y),
      collect: (monitor) => ({
        isOver: !!monitor.isOver(),
        canDrop: !!monitor.canDrop(),
      }),
    }),
    [game],
  )
  const color = 'white';

  return (
    <div
      ref={drop}
      role="Space"
      data-testid={`(${x},${y})`}
      style={{
        position: 'relative',
        width: '100%',
        height: '100%',
      }}
    >
      <Square colorProp={color}>{children}</Square>
      {isOver && !canDrop && <Overlay type={OverlayType.IllegalMoveHover} />}
      {!isOver && canDrop && <Overlay type={OverlayType.PossibleMove} />}
      {isOver && canDrop && <Overlay type={OverlayType.LegalMoveHover} />}
    </div>
  )
}
