fn part1() {
    let hands = include_str!("input.txt")
        .lines();


    let mut answer: i32 = 0;
    for throw in hands {
        let p1 = throw.chars().nth(0).unwrap();
        let p2 = throw.chars().nth(2).unwrap();

        // match each match and just grab the scores
        // the points are based on the p2 shape selected as that is our hand
        // A for Rock, B for Paper, and C for Scissors
        // X for Rock, Y for Paper, and Z for Scissors
        answer += match p1 {
            'A' => match p2 {
                'X' => 4,
                'Y' => 8,
                'Z' => 3,
                _ => panic!()
            },
            'B' => match p2 {
                'X' => 1,
                'Y' => 5,
                'Z' => 9,
                _ => panic!()
            },
            'C' => match p2 {
                'X' => 7,
                'Y' => 2,
                'Z' => 6,
                _ => panic!()
            }
            _ => panic!()
        };
    } 
    println!("{answer}");
}

fn main() {
    part1();
}
