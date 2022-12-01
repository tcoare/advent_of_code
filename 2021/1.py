#! /home/hardcoare/.pyenv/shims/python3
def read_lines(path: str) -> list[int]:
    with open(path) as f:
        return [int(num) for num in f.read().split()]


def part_1(data):
    increased: int = int()
    for pos, depth in enumerate(data):
        if depth > data[pos-1]:
            increased += 1
    return increased


def part_2(data):
    increased: int = int()
    for pos, _ in enumerate(data):
        window_a = sum(data[pos:pos+3])
        window_b = sum(data[pos+1:pos+4])
        if window_b > window_a:
            increased += 1
    return increased


if __name__ == "__main__":
    lines = read_lines("input/1.txt")
    print(part_1(lines))
    print(part_2(lines))
