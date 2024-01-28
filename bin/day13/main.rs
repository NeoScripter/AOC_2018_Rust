use std::{
    cmp::Ordering,
    fmt,
};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Cart {
    cds: (usize, usize),
    dir: u32,
    turns: Vec<u32>,
}

impl Ord for Cart {
    // Reverse order because we'll take carts from the end of the vector after sorting
    fn cmp(&self, other: &Self) -> Ordering {
        other.cds.0.cmp(&self.cds.0).then_with(|| other.cds.1.cmp(&self.cds.1))
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cart {
    fn new(cds: (usize, usize), dir: u32) -> Self {
        Self { cds, dir, turns: vec![90, 0, 270] }
    }
    fn turn(&mut self) {
        self.dir = (self.dir + self.turns.first().copied().unwrap()) % 360;
        self.turns.rotate_left(1);
    }
}
#[derive(Debug, Clone)]
struct Tracks {
    trks: Vec<Vec<char>>,
    carts: Vec<Cart>,
}

impl fmt::Display for Tracks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for trk in &self.carts {
            write!(f, "{}, {}", trk.cds.0, trk.cds.1)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Tracks {
    fn new(trks: Vec<Vec<char>>, carts: Vec<Cart>) -> Self {
        Self { trks, carts }
    }
    fn adj_dir(&self, cart: &mut Cart, y: usize, x: usize) {
        cart.cds = match cart.dir {
            0 => (y, x + 1),
            90 => (y - 1, x),
            180 => (y, x - 1),
            _ => (y + 1, x),
        };

        let (new_y, new_x) = cart.cds;
        match self.trks[new_y][new_x] {
            '\\' => cart.dir = match cart.dir {
                90 | 270 => cart.dir + 90,
                _ => cart.dir + 270,
            },
            '/' => cart.dir = match cart.dir {
                90 | 270 => cart.dir + 270,
                _ => cart.dir + 90,
            },
            '+' => cart.turn(),
            _ => {},
        }
        cart.dir %= 360
    }
    
    fn check_collisions(&self, cart: &Cart, moved_carts: &Vec<Cart>) -> bool {
        moved_carts.iter().any(|c| c.cds == cart.cds) || self.carts.iter().any(|c| c.cds == cart.cds)
    }
    fn find_first_cart(&mut self) -> Option<(usize, usize)> {
        self.carts.sort_unstable();
        let mut moved_carts: Vec<Cart> = Vec::new();
    
        while let Some(mut cart) = self.carts.pop() {
            let (y, x) = cart.cds;

            self.adj_dir(&mut cart, y, x);
    
            if self.check_collisions(&cart, &moved_carts) { return Some(cart.cds) }
            else { moved_carts.push(cart) }
        }
        self.carts = moved_carts;
        None
    }

    fn find_last_cart(&mut self) -> Option<(usize, usize)> {
        self.carts.sort_unstable();
        let mut moved_carts: Vec<Cart> = Vec::new();

        while let Some(mut cart) = self.carts.pop() {
            let (y, x) = cart.cds;

            self.adj_dir(&mut cart, y, x);
    
            if self.check_collisions(&cart, &moved_carts) { 
                moved_carts.retain(|c| c.cds != cart.cds);
                self.carts.retain(|c| c.cds != cart.cds);
            } 
            else { moved_carts.push(cart) }
        }
        self.carts = moved_carts;
        if self.carts.len() == 1 { return Some(self.carts[0].cds) }
        None
    }
}

fn is_cart(c: &char) -> bool {
    match c {
        '>'|'v'|'^'|'<' => true,
        _ => false,
    }
}

fn parse_cart(c: &char) -> u32 {
    match c {
        '>' => 0,
        'v' => 270,
        '^' => 90,
        _ => 180,
    }
}
fn part1() -> (usize, usize) {
    let input = include_str!("input13.txt");
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut carts = Vec::new();

    (0..map.len()).for_each(|y| {
        (0..map[0].len()).for_each(|x| {
            if is_cart(&map[y][x]) {carts.push(Cart::new((y, x), parse_cart(&map[y][x])))}
        })
    });
    let mut tracks = Tracks::new(map, carts);

    let mut crash = None;
    while !crash.is_some() {
        crash = tracks.find_last_cart();
    }
    let (y, x) = crash.unwrap();
    (x, y)
}
fn main() {
    println!("{:?}", part1());
}