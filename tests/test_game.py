import pytest
from game import *


@pytest.fixture
def empty_board():
    return Board()

@pytest.fixture
def empty_block():
    block = Block()
    return block


@pytest.fixture
def full_block():
    block = Block()
    block.stack.append(Token(Color.GREEN, Size.BIG))
    return block

@pytest.fixture
def mid_block():
    block = Block()
    block.stack.append(Token(Color.GREEN, Size.MID))
    return block


@pytest.fixture
def almost_win_board():
    board = Board()
    board.plate[0][0].append(Token(Color.GREEN, Size.BIG))
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


def test_display(almost_win_board):
    almost_win_board.display_board()


def test_is_end():
    assert False


def test_get_outermost():
    assert False


def test_pop_outermost():
    assert False


def test_is_stackable(full_block,mid_block):
    assert not full_block.is_stackable(Token(Color.GREEN, Size.BIG))
    assert mid_block.is_stackable(Token(Color.RED, Size.BIG)), 'should be able to put different color and bigger token'
    assert not mid_block.is_stackable(Token(Color.GREEN, Size.BIG)), "same color shouldn't allow"


def test_append(empty_block):
    empty_block.append(Token(Color.GREEN, Size.BIG))
    assert len(empty_block.stack) == 1
    empty_block.append(Token(Color.GREEN, Size.BIG))
    assert len(empty_block.stack) == 1, "should not change, since it's not stackable"
