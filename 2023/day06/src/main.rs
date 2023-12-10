fn part1(input: &str) -> i64 {
    let (times, records) = input.split_once("\n").unwrap();

    let times: Vec<&str> = times
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect();

    let records: Vec<&str> = records
        .split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .collect();

    let mut num_wins = vec![];
    for (time, record) in times.iter().zip(records.iter()) {
        let mut time_left: i64 = time.parse::<i64>().unwrap();
        let mut wins: i64 = 0;
        for speed in 1..time.parse().unwrap() {
            time_left -= 1;
            let distance: i64 = speed * time_left;
            if distance > record.parse::<i64>().unwrap() {
                wins += 1
            }
        }
        num_wins.push(wins)
    }

    num_wins.iter().fold(1, |acc, &x| acc * x as i64)
}

fn part2(input: &str) -> i64 {
    let (times, records) = input.split_once("\n").unwrap();

    let time: String = times
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>();

    let record: String = records
        .split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|&c| !c.is_whitespace())
        .collect::<String>();

    // println!("{:?}, {:?}", times, records);

    let mut num_wins = 0;
    // for (time, record) in times.zip(records) {
    let mut time_left: i64 = time.parse::<i64>().unwrap();
    for speed in 1..time.parse().unwrap() {
        time_left -= 1;
        let distance: i64 = speed * time_left;
        if distance > record.parse::<i64>().unwrap() {
            num_wins += 1
        }
    }
    // }

    num_wins
}

fn main() {
    println!("{}", part1(include_str!("input.txt")));
    println!("{}", part2(include_str!("input.txt")));
}

#[cfg(test)]
mod test {

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_1() {
        assert_eq!(super::part1(INPUT), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::part2(INPUT), 71503);
    }
}
