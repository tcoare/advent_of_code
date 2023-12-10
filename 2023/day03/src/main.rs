use regex::Regex;
use std::{cmp::min, collections::HashMap};

fn part1(input: &str) -> i32 {
    let matrix: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let number_re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;

    for i in 0..matrix.len() {
        let row = &matrix[i];

        'outer: for number in number_re.find_iter(row) {
            // Check the row above
            if i > 0 {
                let above_row = &matrix[i - 1].as_bytes();

                for j in number.start().saturating_sub(1)..min(above_row.len(), number.end() + 1) {
                    if above_row[j] != b'.' && !above_row[j].is_ascii_digit() {
                        sum += number.as_str().parse::<i32>().unwrap();
                        continue 'outer;
                    }
                }
            }

            // Check the row below
            if i < matrix.len() - 1 {
                let below_row = &matrix[i + 1].as_bytes();

                for j in number.start().saturating_sub(1)..min(below_row.len(), number.end() + 1) {
                    if below_row[j] != b'.' && !below_row[j].is_ascii_digit() {
                        sum += number.as_str().parse::<i32>().unwrap();
                        continue 'outer;
                    }
                }
            }

            // Check the index to the left
            if number.start() > 0 {
                let left = row.as_bytes()[number.start() - 1];

                if left != b'.' && !left.is_ascii_digit() {
                    sum += number.as_str().parse::<i32>().unwrap();
                    continue 'outer;
                }
            }

            // Check the index to the right
            if number.end() < row.len() {
                let right = row.as_bytes()[number.end()];

                if right != b'.' && !right.is_ascii_digit() {
                    sum += number.as_str().parse::<i32>().unwrap();
                    continue 'outer;
                }
            }
        }
    }

    sum
}

fn part2(input: &str) -> i32 {
    let matrix: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let number_re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    for i in 0..matrix.len() {
        let row = &matrix[i];

        for number in number_re.find_iter(row) {
            // Check the row above
            if i > 0 {
                let above_row = &matrix[i - 1].as_bytes();

                for j in number.start().saturating_sub(1)..min(above_row.len(), number.end() + 1) {
                    if above_row[j] == b'*' {
                        gears
                            .entry((i - 1, j))
                            .or_default()
                            .push(number.as_str().parse::<i32>().unwrap());
                    }
                }
            }

            // Check the row below
            if i < matrix.len() - 1 {
                let below_row = &matrix[i + 1].as_bytes();

                for j in number.start().saturating_sub(1)..min(below_row.len(), number.end() + 1) {
                    if below_row[j] == b'*' {
                        gears
                            .entry((i + 1, j))
                            .or_default()
                            .push(number.as_str().parse::<i32>().unwrap());
                    }
                }
            }

            // Check the index to the left
            if number.start() > 0 {
                let left = row.as_bytes()[number.start() - 1];

                if left == b'*' {
                    gears
                        .entry((i, number.start() - 1))
                        .or_default()
                        .push(number.as_str().parse::<i32>().unwrap());
                }
            }

            // Check the index to the right
            if number.end() < row.len() {
                let right = row.as_bytes()[number.end()];

                if right == b'*' {
                    gears
                        .entry((i, number.end()))
                        .or_default()
                        .push(number.as_str().parse::<i32>().unwrap());
                }
            }
        }
    }

    gears
        .values()
        .filter(|vec| vec.len() == 2)
        .for_each(|vec| sum += vec[0] * vec[1]);

    sum
}

fn main() {
    let part1 = part1(include_str!("input.txt"));
    println!("{}", part1);
    let part2 = part2(include_str!("input.txt"));
    println!("{}", part2);
}

#[cfg(test)]
mod test {

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_1() {
        assert_eq!(super::part1(INPUT), 4361);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::part2(INPUT), 467835);
    }
}
