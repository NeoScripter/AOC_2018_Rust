use std::collections::{HashMap, HashSet, VecDeque};

type Point = (i32, i32);

fn simulate_water_flow(clay: &HashSet<Point>, max_y: i32) -> usize {
    let mut water = HashSet::new();
    let mut moving_water = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((500, 0));

    while let Some(point) = queue.pop_front() {
        if point.1 > max_y || water.contains(&point) || moving_water.contains(&point) {
            continue;
        }

        if !clay.contains(&(point.0, point.1 + 1)) && point.1 + 1 <= max_y {
            moving_water.insert(point);
            queue.push_back((point.0, point.1 + 1));
        } else {
            let mut left_bound = None;
            let mut right_bound = None;
            let mut current = point.0;

            while clay.contains(&(current, point.1 + 1)) || water.contains(&(current, point.1 + 1)) {
                current -= 1;
                if clay.contains(&(current, point.1)) {
                    left_bound = Some(current + 1);
                    break;
                }
            }

            current = point.0;
            while clay.contains(&(current, point.1 + 1)) || water.contains(&(current, point.1 + 1)) {
                current += 1;
                if clay.contains(&(current, point.1)) {
                    right_bound = Some(current - 1);
                    break;
                }
            }

            match (left_bound, right_bound) {
                (Some(left), Some(right)) => {
                    for x in left..=right {
                        water.insert((x, point.1));
                    }
                    queue.push_back((point.0, point.1 - 1)); // check the row above
                }
                _ => {
                    moving_water.insert(point);
                    if left_bound.is_none() {
                        queue.push_back((point.0 - 1, point.1));
                    }
                    if right_bound.is_none() {
                        queue.push_back((point.0 + 1, point.1));
                    }
                }
            }
        }
    }

    water.union(&moving_water).count()
}

fn main() {
    let input = include_str!("input18.txt");
    let mut clay = HashSet::new();
    let mut max_y = 0;

    for line in input.lines() {
        let parts: Vec<_> = line.split(", ").collect();
        let range_part = if !parts[0].contains("..") { &parts[1] } else { &parts[0] };
        let fixed_part = if !parts[0].contains("..") { &parts[0] } else { &parts[1] };

        let (start, end): (i32, i32) = {
            let mut parts = range_part[2..].split("..").map(|num| num.parse::<i32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        };
        
        let fixed_value: i32 = fixed_part[2..].parse().unwrap();

        for i in start..=end {
            if !parts[0].contains("..") {
                clay.insert((fixed_value, i));
                max_y = max_y.max(i);
            } else {
                clay.insert((i, fixed_value));
            }
        }
    }

    let result = simulate_water_flow(&clay, max_y);
    println!("Tiles reached by water: {}", result);
}
