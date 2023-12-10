fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let (winning_numbers, numbers) =
                line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning_numbers: Vec<&str> = winning_numbers.split_whitespace().collect();
            let matches = numbers
                .split_whitespace()
                .filter_map(|n| winning_numbers.contains(&n).then_some(()))
                .count() as u32;
            (matches > 0).then(|| 2u32.pow(matches - 1))
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let matches: Vec<_> = input
        .lines()
        .map(|line| {
            let (winning_numbers, numbers) =
                line.split_once(':').unwrap().1.split_once('|').unwrap();
            let winning_numbers: Vec<&str> = winning_numbers.split_whitespace().collect();
            numbers
                .split_whitespace()
                .filter_map(|n| winning_numbers.contains(&n).then_some(()))
                .count() as u32
        })
        .collect();

    let mut card_counts = vec![1u32; matches.len()];

    for (index, _) in matches.clone().into_iter().enumerate() {
        let card_count = card_counts[index];
        let next_card_indices = index + 1..=index + matches[index] as usize;
        for next_card_index in next_card_indices {
            card_counts[next_card_index] += card_count;
        }
    }

    card_counts.iter().sum()
}

fn main() {
    println!("{}", part1(include_str!("input.txt")));
    println!("{}", part2(include_str!("input.txt")));
}

#[cfg(test)]
mod test {

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1() {
        assert_eq!(super::part1(INPUT), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::part2(INPUT), 30);
    }
}
