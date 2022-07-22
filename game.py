import uuid
import random
from enum import Flag, IntEnum


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

class Block:
    def __init__(self):
        self.stack = []

class Board:
    def __init__(self):
        self.plate = [[Block() for i in range(3)] for j in range(3)]

def is_enough(player:Player, take_away:Size):
    if player.inventory[take_away] > 0:
        return True
    else:
        return False


def is_valid_take_from_board(board: Board, x: int, y: int):
    if 0 <= x < 3 and 0 <= y < 3:
        block:Block = board.plate[y][x]
        token_being_taken = block.stack.pop(-1)
        board.plate[y][x] = block
        if is_end(board):
            board.plate[y][x] = block
            return block, token_being_taken
        else:
            return False
    else:
        return False

def is_valid_move(block:Block, token: Token):
    outermost_token:Token = block.stack[-1]
    if outermost_token.color != token.color and outermost_token

    else:
        return False

def place_from_inventory():
    pass
def place_from_board():
    pass

def is_end(board: Board) -> bool:

    pass





class Game:
    def __init__(self, uid: str=str(uuid.uuid4()), round_flag: Color=random.choice([Color.RED,Color.GREEN])):
        self.uid = uid
        self.board = Board()
        self.round_flag = round_flag
        self.player1 = Player(round_flag)
        self.player2 = Player(~round_flag)

