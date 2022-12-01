#! /home/hardcoare/.pyenv/shims/python3
from collections import Counter
from read_lines import read_lines

if __name__ == "__main__":
    lines = read_lines("input/8.txt")
    lines = [line.split(" | ") for line in lines]
    # one line
    # acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
    numbers = list()

    for line in lines:
        output = line[1].split()
        for num in output:
            length = len(num)
            # becuase of the limited number of cases we can calculate number purely based on length
            if length == 2:
                numbers.append(1)
            if length == 4:
              numbers.append(4)
            if length == 3:
              numbers.append(7)
            if length == 7:
                numbers.append(8)
    # want to record the nums 1, 4, 7, 8
    print(len(numbers))

    def count_letters(num, target_word):
        count = int()
        for letter in num:
            if letter in target_word:
                count += 1
        return count

    final_num = int()
    numbers = None
    for line in lines:
        seg = line[0].split()
        seg.sort(key=len)
        output = line[1].split()
        # this was giving wrong result as defaultdict(str), not sure why
        numbers = {0: '', 1: '', 2: '', 3: '', 4: '', 5: '', 6: '', 7: '', 8: '', 9: ''}
        for num in seg:
            length = len(num)
            # need order statements by length
            if length == 2: numbers[1] = num
            if length == 3: numbers[7] = num
            if length == 4: numbers[4] = num
            # bit more work for the other numbers
            if length == 5:
                if count_letters(num, numbers[1]) == 2:
                    numbers[3] = num
                elif count_letters(num, numbers[4]) == 3:
                    numbers[5] = num
                else:
                    numbers[2] = num
            if length == 6:
                if count_letters(num, numbers[1]) == 1:
                    numbers[6] = num
                elif count_letters(num, numbers[4]) == 4:
                    numbers[9] = num
                else:
                    numbers[0] = num
            if length == 7: numbers[8] = num
        digits = ""
        for num in output:
            for val, key in enumerate(numbers.values()):
                if Counter(num) == Counter(key):
                    digits = digits + str(val)
        final_num += int(digits)
    print(final_num)

    # DISCLOSURE I GOT THIS ALOGRITHM FROM SOMEONE ON REDDIT WHO POSTED A DRAKE MEME
    # https://www.reddit.com/r/adventofcode/comments/rbuoil/why_did_they_give_us_all_10_numbers_anyway/
