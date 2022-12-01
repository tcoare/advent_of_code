def read_lines(path: str) -> list:
    with open(path) as f:
        return [line.strip() for line in f.readlines()]
