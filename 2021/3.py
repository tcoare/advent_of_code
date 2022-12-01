#! /home/hardcoare/.pyenv/shims/python3
from read_lines import read_lines

def power_consumption(bytes: list) -> tuple[str, str]:
    gamma = epsilon = ""
    gamma_count = epsilon_count = 0
    for pos in range(12):
        for byte in bytes:
            if byte[pos] == "1":
                gamma_count += 1
            else:
                epsilon_count += 1
        if gamma_count > epsilon_count:
            gamma += "1"
            epsilon += "0"
        else:
            gamma += "0"
            epsilon += "1"
        gamma_count = epsilon_count = 0
    return gamma, epsilon

def life_support_rating(bytes: list, oxygen: bool) -> str:
    for pos in range(12):
        x = y = 0
        if len(bytes) > 1:
            for byte in bytes:
                if byte[pos] == "1":
                    x += 1
                else:
                    y += 1
            if x >= y:
                bit_rating = "1" if oxygen else "0"
            else:
                bit_rating = "0" if oxygen else "1"
            bytes = [byte for byte in bytes if byte[pos] == bit_rating]
    return bytes[0]


if __name__ == "__main__":
    bytes = read_lines("input/3.txt")
    gamma, epsilon = power_consumption(bytes)
    print(int(gamma, 2) * int(epsilon, 2))
    oxygen_rating = life_support_rating(bytes, True)
    co2_rating = life_support_rating(bytes, False)
    print(int(oxygen_rating, 2) * int(co2_rating, 2))
