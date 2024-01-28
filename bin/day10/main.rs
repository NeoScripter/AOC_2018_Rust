// This is not my solution

use std::{collections::HashSet, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix("<")
            .unwrap()
            .strip_suffix(">")
            .unwrap();
        let (x, y) = s.split_once(", ").unwrap();

        let coord = Coord {
            x: x.trim().parse().unwrap(),
            y: y.trim().parse().unwrap(),
        };

        Ok(coord)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Star {
    id: usize,
    position: Coord,
    velocity: Coord,
}

fn parse(input: &str) -> HashSet<Star> {
        input
            .lines()
            .enumerate()
            .map(|(idx, line)| {
                let (position, velocity) = line.split_once(" velocity=").unwrap();
                let position = position
                    .strip_prefix("position=")
                    .unwrap();
                Star {
                    id: idx,
                    position: position.parse().unwrap(),
                    velocity: velocity.parse().unwrap(),
                }
            })
            .collect()
}

fn part1(set: HashSet<Star>) -> (String, u32) {
    let mut stars = set;
    let mut new_stars = stars.clone();
    let (_minmax_x, minmax_y) = stars
        .iter()
        .map(|star| star.position)
        .fold(
            ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
            |(minmax_x, minmax_y), Coord { x, y }| {
                (
                    (minmax_x.0.min(x), minmax_x.1.max(x)),
                    (minmax_y.0.min(y), minmax_y.1.max(y)),
                )
            },
        );
    let mut size = minmax_y.1.abs_diff(minmax_y.0);
    let mut new_size;
    let mut counter = 0;
    loop {
        new_stars.clear();
        // println!("After {} seconds", i);
        // show(&stars.iter().map(|star| star.position).collect());
        // println!();
        for star in stars.iter() {
            new_stars.insert(Star {
                id: star.id,
                position: Coord {
                    x: star.position.x + star.velocity.x,
                    y: star.position.y + star.velocity.y,
                },
                velocity: star.velocity,
            });
        }
        let (_minmax_x, minmax_y) = new_stars
            .iter()
            .map(|star| star.position)
            .fold(
                ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
                |(minmax_x, minmax_y), Coord { x, y }| {
                    (
                        (minmax_x.0.min(x), minmax_x.1.max(x)),
                        (minmax_y.0.min(y), minmax_y.1.max(y)),
                    )
                },
            );
        new_size = minmax_y.1.abs_diff(minmax_y.0);

        if new_size > size {
            break;
        }
        counter += 1;
        // double buffer
        std::mem::swap(&mut stars, &mut new_stars);
        std::mem::swap(&mut size, &mut new_size);
    }

    let stars: HashSet<Coord> = stars
        .iter()
        .map(|star| star.position)
        .collect();
    let mut result = String::new();

    let (minmax_x, minmax_y) = stars.iter().fold(
        ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
        |(minmax_x, minmax_y), Coord { x, y }| {
            (
                (minmax_x.0.min(*x), minmax_x.1.max(*x)),
                (minmax_y.0.min(*y), minmax_y.1.max(*y)),
            )
        },
    );
    for y in minmax_y.0..=minmax_y.1 {
        for x in minmax_x.0..=minmax_x.1 {
            let coord = Coord { x, y };
            let dot = if stars.contains(&coord) { '#' } else { '.' };
            result.push(dot);
        }
        result.push_str("\r\n");
    }

    (result, counter)
}

fn main() {
    let input = include_str!("input10.txt");
    let set = parse(input);
    let result = part1(set);
    println!("{}", result.1);
}