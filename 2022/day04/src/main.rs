use scan_fmt::scan_fmt;

fn part1() {
    let input = include_str!("input.txt")
        .lines()
        .map(|l| scan_fmt!(l, "{d}-{d},{d}-{d}", i32, i32, i32, i32).unwrap())
        .filter(|(a, b, c, d)| ((a <= c && b >= d) || (c <= a && d >= b)))
        .count();
    println!("{:?}", input)
}

fn main() {
    part1();
}
