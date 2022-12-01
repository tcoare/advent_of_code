#! /home/hardcoare/.pyenv/shims/python3
from read_lines import read_lines
import statistics

if __name__ == "__main__":
    lines = read_lines("input/7_work.txt")
    lines = [int(line) for line in lines[0].split(",")]
    fuel_cost = int()
    # using median as if there is a large anomaly and a number of identical positions
    # we would end up using more fuel
    target_pos = statistics.median_low(lines)
    for pos in lines:
        fuel_cost += abs(pos-target_pos)
    print(fuel_cost)


    def crab_cost(num):
        return num*(num+1)/2

    fuel_cost = 1e9
    for test_pos in range(max(lines)):
        test_cost = 0
        # work out fuel cost of each pos up to highest, save lowest fuel cost
        for pos in lines:
            test_cost += crab_cost((abs(pos-test_pos)))
        if test_cost < fuel_cost:
            fuel_cost = test_cost
    print(round(fuel_cost))
