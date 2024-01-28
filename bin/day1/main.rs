use std::collections::HashSet;
fn part1(input: &str) -> i32 {
    let mut acc = 0;
    for x in input.trim().lines().map(|s| s.parse::<i32>().unwrap()) {
        acc += x;
    }
    acc
}
fn part2(input: &str) -> i32 {
    let mut seen: HashSet<i32> = HashSet::new();
    let mut acc = 0;
    seen.insert(acc);
    for x in input.trim().lines().map(|s| s.parse::<i32>().unwrap()).cycle() {
        acc += x;
        if !seen.insert(acc) {
            break;
        }
    }
    acc
}
fn main() {
    let input = include_str!("input1.txt");
    println!("{}, {}", part1(input), part2(input));
}