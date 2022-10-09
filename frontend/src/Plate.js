import { DragPreviewImage, useDrag } from 'react-dnd'
import { ItemTypes } from './ItemTypes.js'
import { knightImage } from './knightImage.js'
const knightStyle = {
  fontSize: 20,
  fontWeight: 'bold',
  cursor: 'move',
}

const containerStyle = {
  width: '100%',
  height: '100%',
  textAlign: 'center'
}

export const Plate = ({ game, x, y }) => {
  const player = game['board'][y][x]['tokens'][0];
  const size = game['board'][y][x]['tokens'][1];

  const [{ isDragging }, drag, preview] = useDrag(
    () => ({
      type: ItemTypes.KNIGHT,
      item: { 
        from: "board",
        x: x,
        y: y
      },
      canDrag: () => game.canDragTargetFromBoard(x, y),
      collect: (monitor) => ({
        isDragging: !!monitor.isDragging(),
        item: monitor.getItem(),
      }),
    }),
    [],
  )

  return (
    <>
      <DragPreviewImage connect={preview} src={knightImage} />
      <div
        ref={drag}
        style={{
          ...knightStyle,
          opacity: isDragging ? 0.5 : 1,
          width: '100%',
          height: '100%',
          backgroundColor: game['board'][y][x]['tokens'][0]
        }}
      >
        <div style={{ ...containerStyle }}>{size}</div>
      </div>
    </>
  )
}
