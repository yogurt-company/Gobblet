export class Game {
  red = { "S": 2, "M": 2, "L": 2 };
  green = { "S": 2, "M": 2, "L": 2 };

  board = [
    [
      {
        "tokens": ['red', 'M']
      },
      {
        "tokens": ['', '']
      },
      {
        "tokens": ['', '']
      }
    ],
    [
      {
        "tokens": ['green', 'L']
      },
      {
        "tokens": ['green', 'S']
      },
      {
        "tokens": ['', '']
      }
    ],
    [
      {
        "tokens": ['red', 'S']
      },
      {
        "tokens": ['', '']
      },
      {
        "tokens": ['', '']
      }
    ],
  ];

  canDropTarget(item, toX = undefined, toY = undefined) {
    if (item && item['from'] === 'stack') {
      return true;
    }

    if ((item['y'] != 0 && !item['y']) || (item['x'] != 0 && !item['x'])) {
      return false;
    }

    if (!this.board[item['y']][item['x']]) {
      return false;
    }

    let targetPlayer = this.board[toY][toX]['tokens'][0];
    let targetSize = this.board[toY][toX]['tokens'][1];

    let fromItemPlayer = this.board[item['y']][item['x']]['tokens'][0];
    let fromItemSize = this.board[item['y']][item['x']]['tokens'][1];

    if (!targetSize) {
      return true;
    }

    if (targetPlayer == fromItemPlayer) {
      return false;
    }

    let size = { 'S': 0, 'M': 1, 'L': 2 };

    if (size[targetSize] >= size[fromItemSize]) {
      return false;
    }

    return true;
  }

  canDragTargetFromBoard(x, y) {
    let targetPlayer = this.board[y][x]['tokens'][0];
    if (targetPlayer) {
      return true;
    }
    return false;
  }

  canDragTargetFromStack(player, size) {
    if(this[player][size] == 0) {
      return false;
    }
    return true;
  }

  whenDropTarget(item, toX, toY) {
    if (item['from'] === 'board') {
      let fromItem = this.board[item['y']][item['x']]['tokens'];
      this.board[toY][toX]['tokens'] = fromItem;
      this.board[item['y']][item['x']]['tokens'] = ["", ""];
    }

    if (item['from'] === 'stack') {
      this.board[toY][toX]['tokens'] = [item['player'], item['size']];
      let player = item['player'];
      this[player][item['size']] -= 1;
    }
  }
}
