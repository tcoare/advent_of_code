#! /home/hardcoare/.pyenv/shims/python3
def get_boards(path: str):
    with open(path) as f:
        boards = [
            [[int(i) for i in line.split()] for line in x.splitlines()]
            for x in f.read().strip().split("\n\n")[1:]
        ]
    return boards


def get_numbers(path: str) -> list[int]:
    with open(path) as f:
        return [int(x) for x in f.readline().strip().split(",")]


def winning_score(numbers: list[int], boards: list[list[list[int]]]):
    for num in numbers:
        for board in boards:
            for i, row in enumerate(board):
                for j, val in enumerate(row):
                    if val == num:
                        board[i][j] = None
            # looking for winners in all rows
            for row in board:
                if all(x is None for x in row):
                    score = sum(i for r in board for i in r if i is not None)
                    return score * num
            # looking for winners in all cols
            for col in zip(*board):
                if all(x is None for x in col):
                    score = sum(i for r in board for i in r if i is not None)
                    return score * num


def last_winning_score(numbers: list[int], boards: list[list[list[int]]]):
    wins = list()
    winner = None
    for num in numbers:
        if sum(x not in wins for x in boards) == 1:
            winner = next(i for i, x in enumerate(boards) if x not in wins)
        for pos, board in enumerate(boards):
            for i, row in enumerate(board):
                for j, val in enumerate(row):
                    if val == num:
                        board[i][j] = None
            # looking for winners in all rows
            for row in board:
                if all(x is None for x in row):
                    if pos == winner:
                        score = sum(i for r in board for i in r if i is not None)
                        return score * num
                    wins.append(board)
            # looking for winners in all cols
            for col in zip(*board):
                if all(x is None for x in col):
                    if pos == winner:
                        score = sum(i for r in board for i in r if i is not None)
                        return score * num
                    wins.append(board)


if __name__ == "__main__":
    path = "input/4.txt"
    numbers = get_numbers(path)
    boards = get_boards(path)
    print(winning_score(numbers, boards))
    print(last_winning_score(numbers, boards))
