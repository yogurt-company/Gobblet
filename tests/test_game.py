import pytest
from game import *


@pytest.fixture
def empty_board():
    return Board()

@pytest.fixture
def almost_win_board():
    board = Board()
    board.plate[0][0].append(Token(Color.GREEN,Size.BIG))
    board.plate[1][1].append(Token(Color.GREEN, Size.BIG))
    board.plate[2][2].append(Token(Color.GREEN, Size.MID))
    board.plate[2][2].append(Token(Color.GREEN, Size.MID))
    return board


def test_is_valid_take_from_board_empty(empty_board):
    assert not empty_board.is_valid_take_from_board(0, 0), "Impossible to take token from empty board"

def test_is_valid_take_from_board_oversize(empty_board):
    assert not empty_board.is_valid_take_from_board(4, 0), "size should 3x3"

def test_is_valid_take_from_board_instant_win(almost_win_board):
    assert not almost_win_board.is_valid_take_from_board(2, 2), "Is invalid to cause instant win"

def test_is_end():
    assert False
