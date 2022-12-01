#! /home/hardcoare/.pyenv/shims/python3
def read_lines(path: str) -> list:
    with open(path) as f:
        # [['forward', '5'], ['up', '5']]
        return [line.split() for line in f]


class Submarine:
    def __init__(self, depth: int = 0, height: int = 0, aim: int = 0) -> None:
        self.depth: int = depth
        self.height: int = height
        self.aim: int = aim

    def run_commands(self, commands: list):
        for command in commands:
            direction = command[0]
            distance = int(command[1])
            if direction == "down":
                self.aim += distance
            elif direction == "up":
                self.aim -= distance
            elif direction == "forward":
                self.height += distance
                self.depth += distance * self.aim

class NewSubmarine():
    depth: int = 0
    height: int = 0
    aim: int = 0


def run_commands(sub: NewSubmarine, commands: list):
    for command in commands:
        direction = command[0]
        distance = int(command[1])
        match direction:
            case "down":
                sub.aim += distance
            case "up":
                sub.aim -= distance
            case "forward":
                sub.height += distance
                sub.depth += distance * sub.aim


if __name__ == "__main__":
    commands = read_lines("input/2.txt")
    sub = Submarine()
    sub.run_commands(commands)
    # for part 2 aim is how we calculate depth so part 1 calulation becomes height * aim
    print(sub.height * sub.aim)
    print(sub.height * sub.depth)

    # using smaller class and separate match case function
    new_sub = NewSubmarine()
    run_commands(new_sub, commands)
    # print(new_sub.height * new_sub.aim, new_sub.height * new_sub.depth)
