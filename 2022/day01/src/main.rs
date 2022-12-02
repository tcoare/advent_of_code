fn part1() {
    let numbers = include_str!("input.txt")
        .lines();

    let mut calories: Vec<i32> = vec![];
    let mut count: i32 = 0;
    
    for cal in numbers {
        if cal.is_empty() {
            // empty line is break in elves
            calories.push(count);
            count = 0
        } else {
            count += cal.parse::<i32>().unwrap();
        }
    }
    calories.sort();
    calories.reverse();
    println!("{:?}", calories[0]);
}

fn part2() {
    let numbers = include_str!("input.txt")
        .lines();

    let mut calories: Vec<i32> = vec![];
    let mut count: i32 = 0;
    
    for cal in numbers {
        if cal.is_empty() {
            // empty line is break in elves
            calories.push(count);
            count = 0
        } else {
            count += cal.parse::<i32>().unwrap();
        }
    }
    calories.sort();
    calories.reverse();
    count = 0;
    for i in 0..3 {
        count += calories[i];
    }
    println!("{:?}", count)
}

fn main() {
    part1();
    part2();
}
