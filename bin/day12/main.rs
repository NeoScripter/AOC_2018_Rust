use std::collections::HashMap;
use itertools::Itertools;
use std::fmt;

#[derive(Debug, Clone)]
struct Pots {
    row: Vec<char>,
    state: i64,
    map: HashMap<Vec<char>, char>,
}

impl fmt::Display for Pots {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for c in &self.row {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl Pots {
    fn new(row: Vec<char>) -> Self {
        let offset = row.iter().position(|&x| x == '#').unwrap() as i64;
        Pots {
            row: row,
            state: offset,
            map: HashMap::new(),
        }
    }
    fn next_gen(&mut self) {
        for _ in 0..3 {
            self.row.insert(0, '.');
            self.row.push('.');
        }
        self.state += 3;
        let row_len = self.row.len();
        let mut new = vec!['.'; row_len];
    
        self.row.windows(5)
            .enumerate()
            .for_each(|(idx, win)| {
                if let Some(&c) = self.map.get(win) {
                    new[idx + 2] = c;
                }
            });
    
        self.row = new;
    }
    fn find_sum(&self) -> i64 {
        self.row.iter().positions(|&x| x == '#').map(|x| x as i64 - self.state).sum::<i64>()
    }
}
fn parse_input() -> Option<Pots> {
    let input = include_str!("input12.txt");
    let (state, map) = input.split_once("\r\n\r\n")?;
    let state = state.strip_prefix("initial state: ")?;
    let row: Vec<char> = state.chars().collect();
    let mut pots = Pots::new(row);
    map.lines().for_each(|line| {
        let (key, value) = line.split_once(" => ").unwrap();
        let key: Vec<char> = key.chars().collect();
        pots.map.insert(key, value.chars().next().unwrap());
    });
    Some(pots)
}
fn part1() -> i64 {
    let mut pots = parse_input().unwrap();
    for _ in 0..20 {
        pots.next_gen();
    }
    pots.find_sum()
}
fn part2() -> i64 {
    let mut pots = parse_input().unwrap();
    let generations: i64 = 50_000_000_000;
    let mut last = 0;
    let mut pr_dif = 0;
    let mut repeated = 0;
    
    for gen in 1..=generations {
        pots.next_gen();
        let score = pots.find_sum();
    
        let dif = score - last;
        if dif == pr_dif {
            repeated += 1;
            if repeated > 10 {
                return (generations - gen) * dif + score;
            }
        } else {
            repeated = 1;
        }
    
        last = score;
        pr_dif = dif;
    }
    last
}
fn main() {
    println!("{}", part2());
}