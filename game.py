import logging
import uuid
import random
from enum import Flag, IntEnum
from typing import Optional
from termcolor import colored


logger = logging.getLogger()
logger.setLevel(logging.DEBUG)


class Color(Flag):
    RED = True
    GREEN = False

class Size(IntEnum):
    BIG = 3
    MID = 2
    SMALL = 1


class Token:
    def __init__(self, color: Color, size: Size):
        self.color = color
        self.size = size
    def __str__(self):
        return f'color: {self.color.name}, size: {self.size.name}'


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
        if not self.stack:
            return True
        else:
            outermost_token: Token = self.get_outermost()
            if outermost_token.color == ~token.color and outermost_token.size < token.size:
                return True
            else:
                return False

    def append(self, token: Token):
        if self.is_stackable(token):
            self.stack += [token]
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


    def display_board(self):
        def _token_indicator(token: Optional[Token]):
            if token:
                if token.color == Color.GREEN:
                    return colored(str(token.size.name),'green')
                else:
                    return colored(str(token.size.name),'red')
            else:
                return "   "

        for j in range(3):
            left = self.plate[j][0].get_outermost()
            mid = self.plate[j][1].get_outermost()
            right = self.plate[j][2].get_outermost()
            print(_token_indicator(left), _token_indicator(mid), _token_indicator(right))

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
        logging.debug(f"""token is {token}""")
        if isinstance(token, Token):
            if board.plate[target_y][target_x].append(token):
                return True
            else:
                print("Illegal movement")
                return False
        else:
            print("Illegal movement")
            return False


    def place_from_board(self, board:Board, source_x, source_y, target_x, target_y):
        token = board.is_valid_take_from_board(source_x, source_y)
        if isinstance(token, Token):
            if board.plate[target_y][target_x].is_stackable(token):
                board.plate[target_y][target_x].append(token)
                return True
        else:
            print("Illegal movement")
            return False


class Game:
    def __init__(self, uid: str = str(uuid.uuid4()), round_flag: Color = random.choice([Color.RED, Color.GREEN])):
        self.uid = uid
        self.board = Board()
        self.round_flag = round_flag
        self.players = {round_flag: Player(round_flag), ~round_flag: Player(~round_flag)}


    def processing(self):
        while True:
            self.cmd(self.players[self.round_flag])
            self.round_flag = ~self.round_flag
            if self.board.is_end():
                print('end')
                break
    def cmd(self,player:Player):
        logging.debug(f"debug msg")
        self.board.display_board()
        need_to_conti = True
        while need_to_conti:
            in_str = input(f"Player{str(player.color)} a: display, b: take from invetory, c:board 2 board:\n")
            if in_str == 'a':
                self.board.display_board()
            elif in_str == 'b':
                size = input("size? :\n")
                target_x, target_y = map(lambda x: int(x), input("target? :\n").split(","))
                if size == 'b':
                    size = Size.BIG
                elif size == 'm':
                    size = Size.MID
                elif size == 's':
                    size = Size.SMALL
                else:
                    continue
                if player.place_from_inventory(self.board, int(target_x), int(target_y), size):
                    break
            elif in_str == 'c':
                try:
                    from_x, from_y = map(lambda x:int(x),input("from? :\n").split(","))
                    target_x, target_y = map(lambda x:int(x),input("target? :\n").split(","))
                    updated_board = player.place_from_board(self.board,from_x, from_y,target_x, target_y)
                    if updated_board:
                        self.board = updated_board
                        break
                except:
                    continue



if __name__ == '__main__':
    print("Game Start")
    g1 = Game()
    g1.processing()