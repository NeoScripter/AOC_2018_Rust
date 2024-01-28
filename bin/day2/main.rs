use itertools::Itertools;
fn part1(input: &str) -> u32 {
    let mut x = 0;
    let mut y = 0;

    for line in include_str!("input2.txt").lines() {
        let mut counts = [0u8; 26];
        for c in line.trim().chars() {
            counts[(c as u8 - b'a') as usize] += 1;
        }

        x += counts.contains(&2) as u32;
        y += counts.contains(&3) as u32;
    };
    x * y
}
fn part2(input: &str) -> String {
    let boxes: Vec<Vec<char>> = include_str!("input2.txt").lines().map(|line| line.trim().chars().collect()).collect();
    let pair = boxes.iter()
    .combinations(2)
    .find(|pair| {
        pair[0].iter()
            .zip(pair[1])
            .filter(|(a, b)| a != b)
            .count() == 1
    });
    let same_chars: String = pair.map(|pair| {
        pair[0].iter()
            .zip(pair[1])
            .filter_map(|(a, b)| if a == b { Some(*a) } else { None })
            .collect()
    }).unwrap_or_else(String::new);
    same_chars
}
fn main() {
    let input = include_str!("input2.txt");
    println!("{}, {}", part1(input), part2(input));
}
