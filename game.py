import uuid
import random
from enum import Flag, IntEnum
from optparse import Option
from typing import Optional


class Color(Flag):
    RED = True
    GREEN = False

class Size(IntEnum):
    BIG = 3
    MID = 2
    SMALL = 1


class Token:
    def __init__(self,color:Color , size:Size):
        self.color = color
        self.size = size

class Player:
    def __init__(self,color:Color):
        self.color = color
        self.inventory = {Size.BIG:2, Size.MID:2, Size.SMALL:2}

    def is_enough(self, take_away: Size):
        if self.inventory[take_away] > 0:
            self.inventory[take_away] -= 1
            return Token(self.color,take_away)
        else:
            return False

    def place_from_inventory(self, board, target_x, target_y, size):
        token = self.is_enough(size)
        if isinstance(token, Token):
            if is_stackable(board[target_y][target_x], token):
                board[target_y][target_x].append(token)
            else:
                print("Illegal movement")
                return False
        else:
            print("Illegal movement")
            return False

    def place_from_board(self, board, source_x, source_y, target_x, target_y):
        token = is_valid_take_from_board(board, source_x, source_y)
        if isinstance(token, Token):
            if is_stackable(board[target_y][target_x], token):
                board[target_y][target_x].append(token)
                return board
        else:
            print("Illegal movement")
            return False


class Block:
    def __init__(self):
        self.stack = []

    def get_outermost(self):
        if self.stack:
            return self.stack[-1]
        else:
            return False

    def pop_outermost(self):
        if self.stack:
            return self.stack.pop(-1)
        else:
            return False

    def is_stackable(self, token: Token):
        outermost_token: Token = self.get_outermost()
        if outermost_token:
            if outermost_token.color == ~token.color and outermost_token.size < token.size:
                return True
            else:
                return False
        else:
            return False

    def append(self, token: Token):
        if self.is_stackable(token):
            self.stack.append(token)
            return True
        else:
            return False




class Board:
    def __init__(self):
        self.plate = [[Block() for i in range(3)] for j in range(3)]

    def is_valid_take_from_board(self, x: int, y: int):
        if 0 <= x < 3 and 0 <= y < 3:
            block:Block = self.plate[y][x]
            token_being_taken = block.pop_outermost()
            self.plate[y][x] = block
            if self.is_end():
                return token_being_taken
            else:
                return False
        else:
            return False

    def is_end(self) -> Optional[Color]:
        patterns = [
            ([2, 0], [1, 1], [0, 2]),
            ([0, 0], [1, 1], [2, 2]),
            ([0, 0], [0, 1], [0, 2]),
            ([1, 0], [1, 1], [1, 2]),
            ([2, 0], [2, 1], [2, 2]),
            ([0, 0], [1, 0], [2, 0]),
            ([0, 1], [1, 1], [2, 1]),
            ([0, 2], [1, 2], [2, 2]),
        ]
        for pattern in patterns:
            if all([self.plate[y][x].pop_outermost() == Color.GREEN for x, y in pattern]):
                return Color.GREEN
            elif all([self.plate[y][x].pop_outermost() == Color.RED for x, y in pattern]):
                return Color.RED
            else:
                return None





class Game:
    def __init__(self, uid: str = str(uuid.uuid4()), round_flag: Color = random.choice([Color.RED, Color.GREEN])):
        self.uid = uid
        self.board = Board()
        self.round_flag = round_flag
        self.player1 = Player(round_flag)
        self.player2 = Player(~round_flag)



if __name__ == '__main__':
    print("Game Start")
    game = Game()
    value,b = input("Please enter a string:\n")