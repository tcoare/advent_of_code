use std::collections::HashMap;

fn part1() {
    let input = include_str!("input.txt")
        .lines();

    let mut prio_value: HashMap<char, i32> = HashMap::new();
    let letters: String = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();
    let mut count: i32 = 1;

    for c in letters.chars() {
        prio_value.insert(c, count);
        count += 1
    }

    let mut prio: i32 = 0;
    for line in input {
        let (first, last) = line.split_at(line.len()/2);

        let mut seen: Vec<char> = vec![];
        for c in first.chars() {
            if last.contains(c) && !seen.contains(&c) {
                prio += prio_value.get(&c).unwrap(); 
                seen.push(c)
            }
        }

    }
    println!("{:?}", prio)

}

fn main() {
    part1();
}
