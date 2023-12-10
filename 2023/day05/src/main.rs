use std::{collections::HashMap, vec};

struct Almanac {
    source: u64,
    dest: u64,
    range: u64,
}

struct Mapping {
    name: String,
    maps: Vec<u64>,
}

impl Almanac {
    // fn genereate_mappings(&self) -> HashMap<usize, usize> {}
}

fn part1(input: &str) -> usize {
    let lines = input.lines();

    // let mappings: Vec<_> = input
    //     .lines()
    //     .into_iter()
    //     .map(|line| {
    //         let mappings = line.split_whitespace().collect::<Vec<_>>();
    //         mappings
    //     })
    //     .collect();
    let (seeds, mappings) = input.split_once("\n\n").expect("Expected two sections");
    let seeds: Vec<usize> = seeds
        .trim()
        .strip_prefix("seeds: ")
        .and_then(|s| {
            s.split_ascii_whitespace()
                .map(|s| s.parse::<usize>())
                .collect::<Result<_, _>>()
                .ok()
        })
        .expect("Expected seed numbers");

    let mappings = mappings
        .trim()
        .split("\n\n")
        .map(|s| s.parse::<Mapping>())
        .collect::<Result<_, _>>()
        .expect("Expected mappings");

    for mapping in mappings {
        println!("{}", mapping)
    }

    let mut source: usize = 90;
    let mut dest: usize = 58;
    let range: usize = 2;

    for _ in 0..range {
        dest += 1;
        source -= 1;
    }

    source
}

// fn part2(input: &str) -> u32 {
// }

fn main() {
    println!("{}", part1(include_str!("input.txt")));
    // println!("{}", part2(include_str!("input.txt")));
}

#[cfg(test)]
mod test {

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        assert_eq!(super::part1(INPUT), 35);
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(super::part2(INPUT), 30);
    // }
}
