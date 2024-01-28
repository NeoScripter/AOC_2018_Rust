use std::collections::HashMap;
use itertools::Itertools;
use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
struct Guard {
    number: u32,
    time_asleep: Vec<u32>,
    time_awake: Vec<u32>,
    sleep_ranges: Vec<Range<u32>>,
}
impl Guard {
    fn clear(&mut self) {
        self.number = 0;
        self.time_asleep.clear();
        self.time_awake.clear();
    }
}
struct PrettyVec(Vec<Guard>);

impl fmt::Display for PrettyVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for guard in &self.0 {
            writeln!(f, "Number: {}", guard.number)?;
            writeln!(f, "Time Asleep: {:?}", guard.time_asleep)?;
            writeln!(f, "Time Awake: {:?}", guard.time_awake)?;
            writeln!(f, "")?;
        }
        Ok(())
    }
}
fn sort_keys_part2(map: HashMap<u32, u32>) -> Option<(u32, u32)> {
    map.into_iter()
       .map(|(k, v)| (std::cmp::Reverse(v), k))
       .sorted_unstable()
       .next()
       .map(|(std::cmp::Reverse(v), k)| (v, k))
}

fn sort_keys_part1(map: HashMap<u32, u32>) -> Option<u32> {
    map.into_iter()
       .map(|(k, v)| (std::cmp::Reverse(v), k))
       .sorted_unstable()
       .next()
       .map(|(_k, v)| v)
}
fn find_the_guard(vec: Vec<Guard>) -> (u32, Vec<Range<u32>>) {
    let mut longest_sleep_time = 0;
    let mut number = 0;
    let mut ranges = vec![];

    for guard in vec.iter() {
        let sum: u32 = guard.time_asleep.iter().sum();
        if sum > longest_sleep_time {
            longest_sleep_time = sum;
            number = guard.number;
            ranges.clear();
            ranges.extend(guard.sleep_ranges.clone());
        }
    }
    (number, ranges)
}

fn sort_lines(vec: &mut Vec<String>) {
    vec.sort_by(|a, b| {
        let num_a: i32 = a[0..8].parse().unwrap();
        let num_b: i32 = b[0..8].parse().unwrap();
        num_a.cmp(&num_b)
    });
}

fn find_unique_guards(input: &str) -> Vec<Guard> {
    let mut lines: Vec<String> = input.lines().map(|line| {
        line.trim().strip_prefix("[1518-").unwrap().replace("-", "").replace(":", "").replace(" ", "")
    }).collect();
    sort_lines(&mut lines);
    let mut lines: Vec<String> = lines.iter_mut().map(|line| {
        let mut mod_line = String::from(&line[4..]);
        if mod_line.starts_with("23") {
            mod_line.replace_range(0..4, "0000")
        }
        let mod_line = String::from(&mod_line[2..]);
        mod_line
    }).collect();
    lines.push("End of list".to_string());
    let mut guards: Vec<Guard> = Vec::new();
    let mut guard = Guard {
        number: 0,
        time_asleep: vec![],
        time_awake: vec![],
        sleep_ranges: vec![],
    };
    for line in lines.iter() {
        if line.ends_with("beginsshift") {
            if !guard.time_asleep.is_empty() {
                guard.time_asleep.push(59);
                guards.push(guard.clone());
            }
            guard.clear();
            let (_num, rest) = line.split_once("]").unwrap();
            guard.time_awake.push(0);
            let number = rest.strip_prefix("Guard#").unwrap().strip_suffix("beginsshift").unwrap().parse::<u32>().unwrap();
            guard.number += number;
        }
        if line.ends_with("fallsasleep") {
            let (num, _) = line.split_once("]").unwrap();
            guard.time_asleep.push(num.parse::<u32>().unwrap());
        }
        if line.ends_with("wakesup") {
            let (num, _) = line.split_once("]").unwrap();
            guard.time_awake.push(num.parse::<u32>().unwrap());
        }
        if line == "End of list" {
            guard.time_asleep.push(59);
            guards.push(guard.clone());
            break;
        }
    }
    for guard in &mut guards {
        let mut vec_clone = guard.time_awake.clone();
        vec_clone.remove(0);
        let mut interleave_iter = guard.time_asleep.iter().cloned().interleave(vec_clone.iter().cloned());
        let mut vec_range: Vec<Range<u32>> = Vec::new();

        while let (Some(start), Some(end)) = (interleave_iter.next(), interleave_iter.next()) {
            vec_range.push(start..end);
        }
        guard.sleep_ranges.extend(vec_range);
    }
    for guard in &mut guards {
        let vec: Vec<u32> = guard
        .time_awake
        .clone()
        .into_iter()
        .interleave(guard.time_asleep.clone().into_iter())
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();
        guard.time_awake.clear();
        guard.time_asleep.clear();
        
        let len = vec.len();
        for idx in 0..len {
            if idx % 2 == 0 {
                guard.time_awake.push(vec[idx]);
            } else {
                guard.time_asleep.push(vec[idx]);
            }
        }
    }
    let mut guard_map: HashMap<u32, Guard> = HashMap::new();

    for guard in guards {
        guard_map
            .entry(guard.number)
            .and_modify(|existing: &mut Guard| {
                existing.time_asleep.extend(guard.time_asleep.clone());
                existing.time_awake.extend(guard.time_awake.clone());
                existing.sleep_ranges.extend(guard.sleep_ranges.clone());
            })
            .or_insert(guard);
    }
    let unique_guards: Vec<Guard> = guard_map.into_iter().map(|(_, v)| v).collect();
    unique_guards
}
fn part1(input: &str) -> u32 {
    let unique_guards = find_unique_guards(input);
    let (number, ranges) = find_the_guard(unique_guards);
    let mut range_map = HashMap::new();
    for range in ranges.into_iter() {
        for el in range {
            *range_map.entry(el).or_insert(0) += 1;
        }
    }
    number * sort_keys_part1(range_map).unwrap()
}
fn part2(input: &str) -> u32 {
    let unique_guards = find_unique_guards(input);
    let mut num_of_times = 0;
    let mut number = 0;
    let mut minute_asleep = 0;
    for guard in unique_guards {
        let mut range_map = HashMap::new();
        for range in guard.sleep_ranges.into_iter() {
            for el in range {
                *range_map.entry(el).or_insert(0) += 1;
            }
        }
        let (freq, minute) = sort_keys_part2(range_map).unwrap();
        if num_of_times < freq {
            num_of_times = freq;
            minute_asleep = minute;
            number = guard.number;
        }
    }
    number * minute_asleep
}

fn main() {
    let input = include_str!("input4.txt");
    println!("{}, {}", part1(input), part2(input));
}