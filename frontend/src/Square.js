const squareStyle = {
  width: '100%',
  height: '100%',
}
export const Square = ({ colorProp, children }) => {
  const backgroundColor = (colorProp) ? colorProp : 'white';
  const color = 'black';
  return (
    <div
      style={{
        ...squareStyle,
        color,
        backgroundColor,
      }}
    >
      {children}
    </div>
  )
}
