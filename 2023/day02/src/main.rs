fn part1(input: &str) -> i32 {
    let input = input.lines();

    let mut total: i32 = 0;

    for games in input {
        let result = games.split(":").collect::<Vec<&str>>();
        let id = result
            .first()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let game = result.last().unwrap().split(";").collect::<Vec<&str>>();

        let mut possible: bool = true;

        for set in game {
            let mut green: i32 = 0;
            let mut red: i32 = 0;
            let mut blue: i32 = 0;

            let set_pairs: Vec<&str> = set.split(',').map(|s| s.trim()).collect();

            for pair in set_pairs {
                let parts: Vec<&str> = pair.split_whitespace().collect();
                if parts.len() == 2 {
                    let count = parts[0].parse::<i32>().unwrap_or_default();
                    let colour = parts[1];

                    match colour {
                        "green" => green += count,
                        "blue" => blue += count,
                        "red" => red += count,
                        _ => println!("missing some colour"),
                    }
                }
            }
            if red > 12 || green > 13 || blue > 14 {
                possible = false;
            }
        }

        if possible {
            total += id
        }
    }

    total
}

fn part2(input: &str) -> i32 {
    let input = input.lines();

    let mut total: i32 = 0;

    for games in input {
        let result = games.split(":").collect::<Vec<&str>>();
        let id = result
            .first()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();

        let game = result.last().unwrap().split(";").collect::<Vec<&str>>();

        println!("on game {:?}", id);
        println!("{:?}", game);
        let mut green: i32 = 0;
        let mut red: i32 = 0;
        let mut blue: i32 = 0;

        for set in game {
            let set_pairs: Vec<&str> = set.split(',').map(|s| s.trim()).collect();
            println!("{:?}", set_pairs);

            for pair in set_pairs {
                let parts: Vec<&str> = pair.split_whitespace().collect();
                if parts.len() == 2 {
                    let count = parts[0].parse::<i32>().unwrap_or_default();
                    let colour = parts[1];

                    match colour {
                        "green" => {
                            if count > green {
                                green = count
                            }
                        }
                        "red" => {
                            if count > red {
                                red = count
                            }
                        }
                        "blue" => {
                            if count > blue {
                                blue = count
                            }
                        }
                        _ => println!("missing some colour"),
                    }
                }
            }
        }
        total += green * red * blue;
    }
    total
}

fn main() {
    let part1 = part1(include_str!("input.txt"));
    println!("{}", part1);
    let part2 = part2(include_str!("input.txt"));
    println!("{}", part2);
}

#[cfg(test)]
mod test {

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1() {
        assert_eq!(super::part1(INPUT), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::part2(INPUT), 2286);
    }
}
