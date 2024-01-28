use std::collections::HashMap;
use regex::Regex;

fn part1(input: &str) -> usize {
    let mut claims = HashMap::new();
    input.lines().for_each(|line| {
        let (_id, rest) = line.trim().split_once("@ ").unwrap();
        let (left, right) = rest.split_once(": ").unwrap();
        let (dis_left_edge, dis_top_edge) = left.split_once(",").unwrap();
        let (width, height) = right.split_once("x").unwrap();

        let dis_left_edge: u32 = dis_left_edge.parse().unwrap();
        let dis_top_edge: u32 = dis_top_edge.parse().unwrap();
        let width: u32 = width.parse().unwrap();
        let height: u32 = height.parse().unwrap();

        for x in dis_left_edge..(dis_left_edge + width) {
            for y in dis_top_edge..(dis_top_edge + height) {
                let inch = (x, y);
                claims.entry(inch).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    });
    claims.values().filter(|&&val| val > 1).count()
}

fn part2(input: &str) -> u32 {
    let mut claims = HashMap::new();
    let mut all_ids: Vec<u32> = Vec::new();
    let mut overlapping_ids: Vec<u32> = Vec::new();

    for line in input.lines() {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        if let Some(caps) = re.captures(line.trim()) {
            let id: u32 = caps[1].parse().unwrap();
            let dis_left_edge: u32 = caps[2].parse().unwrap();
            let dis_top_edge: u32 = caps[3].parse().unwrap();
            let width: u32 = caps[4].parse().unwrap();
            let height: u32 = caps[5].parse().unwrap();
            all_ids.push(id);
            
            for x in dis_left_edge..(dis_left_edge + width) {
                for y in dis_top_edge..(dis_top_edge + height) {
                    let inch = (x, y);
                    claims.entry(inch)
                        .and_modify(|e: &mut Vec<u32>| {
                            e.push(id);
                            if e.len() > 1 {
                                overlapping_ids.extend(e.iter().copied());
                            }
                        })
                        .or_insert_with(|| vec![id]);
                }
            }
        }
    }

    overlapping_ids.sort();
    overlapping_ids.dedup();

    let non_overlapping_ids: Vec<u32> = all_ids.into_iter()
        .filter(|id| !overlapping_ids.contains(id))
        .collect();

    non_overlapping_ids[0]
}
fn main() {
    let input = include_str!("input3.txt");
    println!("{}, {}", part1(input), part2(input));
}