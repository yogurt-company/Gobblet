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
  height: '50%',
  textAlign: 'center'
}

export const Inventory = ({ player, game, currentIndex }) => {
  const sizeList = ['S', 'M', 'L']

  const [{ isDragging }, drag, preview] = useDrag(
    () => ({
      type: ItemTypes.KNIGHT,
      item: { 
        from: "stack",
        player: player,
        size: sizeList[currentIndex]
      },
      canDrag: () => game.canDragTargetFromStack(player, sizeList[currentIndex]),
      collect: (monitor) => ({
        isDragging: !!monitor.isDragging(),
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
          height: '100%'
        }}
      >
        <div style={{ ...containerStyle }}>{sizeList[currentIndex]}</div>
        <div style={{ ...containerStyle }}>{game[player][sizeList[currentIndex]]}</div>
      </div>
    </>
  )
}
