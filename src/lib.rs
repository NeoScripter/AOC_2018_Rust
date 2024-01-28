#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]
use itertools::Itertools;
use std::{
    fmt,
    collections::{VecDeque, HashSet},
};

#[derive(Debug, Clone, Copy)]
enum Crds {
    Xv(usize),
    Yv(usize),
    Xrng(usize, usize),
    Yrng(usize, usize),
}

#[derive(Debug, Clone)]
struct Scan {
    gd: Vec<Vec<char>>,
    st: (usize, usize),
    max_y: usize,
}

impl fmt::Display for Scan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.gd {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Scan {
    fn new(max_y: usize, max_x: usize) -> Self {
        let mut map = vec![vec!['.'; max_x + 2]; max_y + 3];
        map[0][6] = '+';
        Self { gd: map, st: (0, 6), max_y: 0 }
    }
    fn count_tiles(&self) -> usize {
        (0..self.gd.len()).map(|y| {
            (0..self.gd[0].len()).filter(|&x| {
                self.gd[y][x] == '|' || self.gd[y][x] == '~'
            }).count()
        }).sum()
    }
    fn water_sim(&mut self, x: usize, mut y: usize) {
        let mut stack = Vec::new();
        while self.gd[y+1][x] != '#' {
            if y >= self.max_y {return}
            self.gd[y+1][x] = '|';
            stack.push((y+1, x));
            y += 1;
        }

        while let Some((ny, nx)) = stack.pop() {
            //println!("{}", self);
            self.gd[ny][nx] = '~';
            let mut overflowing = false;
            let mut right = nx.clone();
            while self.gd[ny][right+1] != '#' {
                self.gd[ny][right+1] = '~';
                right += 1;
                if self.gd[ny+1][right] == '.' {
                    overflowing = true;
                    self.water_sim(right, ny);
                    if self.gd[ny+1][right] != '~' {break}
                    else {overflowing = false}
                }
            }
            let mut left = nx.clone();
            while self.gd[ny][left-1] != '#' {
                self.gd[ny][left-1] = '~';
                left -= 1;
                if self.gd[ny+1][left] == '.' {
                    overflowing = true;
                    self.water_sim(left, ny);
                    if self.gd[ny+1][left] != '~' {break}
                    else {overflowing = false}
                }
            }
            if overflowing {break}
        }
    }
    fn fill_x(&mut self, old_y: usize, old_x: usize, cache: &mut HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut overflowing = Vec::new();
        let mut pq = Vec::new();
        pq.push((old_y, old_x));
        while let Some((y, x)) = pq.pop() {
            self.gd[y][x] = '~';
            if !cache.insert((y, x)) { continue }
            if self.gd[y + 1][x] == '.' {overflowing.push((y, x)); continue; }
            if self.gd[y][x + 1] == '.' {pq.push((y, x + 1))}
            if self.gd[y][x - 1] == '.' {pq.push((y, x - 1))}
        }
        overflowing
    }
    fn fill_y(&mut self) -> usize {
        let mut cache: HashSet<(usize, usize)> = HashSet::new();
        let mut mem: HashSet<(usize, usize)> = HashSet::new();
        let mut pq = VecDeque::new();
        pq.push_back((self.st.0 + 1, self.st.1));
        while let Some((y, x)) = pq.pop_front() {
            self.gd[y][x] = '~';
            println!("{}", self);
            mem.insert((y, x));
            if y + 1 > self.max_y { continue }
            if self.gd[y + 1][x] == '#' {
                let mut new_y = y;
                loop {
                    println!("{}", new_y);
                    let overflowing = self.fill_x(new_y, x, &mut cache);
                    if overflowing.is_empty() { new_y -= 1; continue; }
                    else {
                        for crd in overflowing { pq.push_back(crd) };
                        break;
                    }
                }
            } else 
            { pq.push_back((y + 1, x)); }
        }
        let ovrl: HashSet<(usize, usize)> = cache.into_iter().chain(mem.into_iter()).collect();
        ovrl.len()
    }
}

fn parse_input() -> Scan {
    let input = include_str!("input_lib.txt");
    let (mut max_x, mut max_y) = (0, 0);
    let mut crds = Vec::new();

    for line in input.lines() {
        let mut line_crds = Vec::new();
        for part in line.split(", ") {
            let (left, right) = part.split_once('=').unwrap();
            let crd = match right.split_once("..") {
                Some((l, r)) => {
                    let (st, end) = (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap());
                    if left == "x" {
                        max_x = max_x.max(end - 494);
                        Crds::Xrng(st - 494, end - 494)
                    } else {
                        max_y = max_y.max(end);
                        Crds::Yrng(st, end)
                    }
                },
                None => {
                    let v = right.parse::<usize>().unwrap();
                    if left == "x" {
                        max_x = max_x.max(v - 494);
                        Crds::Xv(v - 494)
                    } else {
                        max_y = max_y.max(v);
                        Crds::Yv(v)
                    }
                },
            };
            line_crds.push(crd);
        }
        crds.push(line_crds);
    }    

    let mut scan = Scan::new(max_y, max_x);
    for crd in crds {
        match (crd[0], crd[1]) {
            (Crds::Xv(x), Crds::Yrng(st, end)) | (Crds::Yrng(st, end), Crds::Xv(x)) => 
                (st..=end).for_each(|y| scan.gd[y][x] = '#'),
    
            (Crds::Yv(y), Crds::Xrng(st, end)) | (Crds::Xrng(st, end), Crds::Yv(y)) => 
                (st..=end).for_each(|x| scan.gd[y][x] = '#'),
    
            _ => {}
        }
    }
    scan.max_y = max_y;
    scan
}

fn part1() -> usize {
    let mut scan = parse_input();
    scan.water_sim(scan.st.1, scan.st.0);
    println!("{}", scan);
    scan.count_tiles()
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_1() {
        assert_eq!(57, part1());
    }
}