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
        let mut map = vec![vec!['.'; max_x + 500]; max_y + 33];
        map[0][500] = '+';
        Self { gd: map, st: (0, 500), max_y: 0 }
    }
    fn count_tiles(&self) -> usize {
        (0..self.gd.len()).map(|y| {
            (0..self.gd[0].len()).filter(|&x| {
                self.gd[y][x] == '|' || self.gd[y][x] == '~'
            }).count()
        }).sum()
    }
    fn water_sim(&mut self, x: usize, mut y: usize, stack: &mut Vec<(usize, usize)>) {
        while self.gd[y+1][x] == '.' {
            if y+1 >= self.max_y {return}
            y += 1;
            self.gd[y][x] = '~';
            stack.push((y, x));
        }

        if self.gd[y+1][x] == '~' {return}

        while let Some((ny, nx)) = stack.pop() {
            if ny > 10 {
                (ny-10..ny+20).for_each(|id_y| {
                    (nx-20..nx+40).for_each(|id_x| {
                        print!("{}", self.gd[id_y][id_x])
                    });
                    println!("");
                });
                println!("");
            }
            self.gd[ny][nx] = '~';
            let mut right = nx.clone();
            while self.gd[ny][right+1] == '.' {
                right += 1;
                self.gd[ny][right] = '~';
                if self.gd[ny+1][right] == '.' {
                    let mut s1 = Vec::new();
                    self.water_sim(right, ny, &mut s1);
                    if !s1.is_empty() {break}
                }
            }
            let mut left = nx.clone();
            while self.gd[ny][left-1] == '.' {
                left -= 1;
                self.gd[ny][left] = '~';
                if self.gd[ny+1][left] == '.' {
                    let mut s2 = Vec::new();
                    self.water_sim(left, ny, &mut s2);
                    if !s2.is_empty() {break}
                }
            }
        }
    }
}

fn parse_input() -> Scan {
    let input = include_str!("input17.txt");
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
                        max_x = max_x.max(end);
                        Crds::Xrng(st, end)
                    } else {
                        max_y = max_y.max(end);
                        Crds::Yrng(st, end)
                    }
                },
                None => {
                    let v = right.parse::<usize>().unwrap();
                    if left == "x" {
                        max_x = max_x.max(v);
                        Crds::Xv(v)
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
    let mut stack = Vec::new();
    scan.water_sim(scan.st.1, scan.st.0, &mut stack);
    //println!("{}", scan);
    scan.count_tiles()
}
fn main() {
    println!("{}", part1());
}
