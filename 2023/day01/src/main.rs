fn part1(input: &str) -> u32 {
    input
        .lines()
        // panics unwraping an empty line later on
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        // we dont have to convert the first digit we can just times 10
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        // panics unwraping an empty line later on
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.to_string()
                // we replace the spelt numbers to inlude the digit of the same number
                // our original solution will pick up the numbers
                .replace("one", "o1ne")
                .replace("two", "t2wo")
                .replace("three", "t3hree")
                .replace("four", "f4our")
                .replace("five", "f5ive")
                .replace("six", "s6ix")
                .replace("seven", "s7even")
                .replace("eight", "e8ight")
                .replace("nine", "n9ine")
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        // we dont have to convert the first digit we can just times 10
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}

fn main() {
    let part1 = part1(include_str!("input.txt"));
    println!("{}", part1);
    let part2 = part2(include_str!("input.txt"));
    println!("{}", part2);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_part_1() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(super::part1(input), 142);
    }

    #[test]
    fn test_part_2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";

        assert_eq!(super::part2(input), 281);
    }
}
