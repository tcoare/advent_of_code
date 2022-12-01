#! /home/hardcoare/.data/shims/python3
from collections import defaultdict, Counter
from read_lines import read_lines


class lanternfish:
    def __init__(self, timer, born_today: bool) -> None:
        self.timer: int = timer
        self.born_today: bool = born_today

    def __repr__(self) -> str:
        return f"fish: {self.timer}, {self.born_today}"


if __name__ == "__main__":
    lines = read_lines("input/6_work.txt")
    lines = [int(line) for line in lines[0].split(",")]
    ocean = [lanternfish(num, born_today=False) for num in lines]

    # part 1
    for day in range(80):
        for fish in ocean:
            # before we append the new fish we need to update the previous day's fish to be not born today
            if fish.timer > 8:
                fish.born_today = False
            # decrement timer if timer is larger than 1 and the fish wasnt born today
            # if its 1 then we create a new lanternfish and we set timer to 6
            if fish.timer > 0 and not fish.born_today:
                fish.timer -= 1
            else:
                fish.timer = 6
                ocean.append(lanternfish(9, born_today=True))
    print(len(ocean))

    # part 2
    ocean = Counter([int(line) for line in lines])
    for day in range(256):
        updated_ocean = defaultdict(int)
        for time, amount in ocean.items():
            # for items of 0 time we create that amount of fish at times of 6 and 8
            if time == 0:
                updated_ocean[6] += amount
                updated_ocean[8] += amount
            # otherwise we then add the value of the current time lanternfish one time lower
            # the x amount of y time lanternfish become the x amount of y-1 time lanternfish for the next day
            else:
                updated_ocean[time - 1] += amount
        ocean = updated_ocean
    print(sum(ocean.values()))
