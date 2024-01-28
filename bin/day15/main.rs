use itertools::Itertools;
use std::{
    fmt,
    collections::{HashMap, HashSet, VecDeque},
};

#[derive(Debug, Clone, Hash)]
struct Unit {
    tp: char,
    hp: i32,
    hit: i32,
}

impl Unit {
    fn new(tp: char, hit: i32) -> Self {
        Self { tp, hp: 200, hit }
    }
}

#[derive(Debug, Clone)]
struct Battle {
    fld: Vec<Vec<char>>,
    units: HashMap<(usize, usize), Unit>,
    rnds: i32,
}

impl fmt::Display for Battle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.fld {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Battle {
    fn attack(&mut self, y: usize, x: usize, unit_tp: char, unit_dg: i32, enemy: char) -> Option<(usize, usize)> {
        let nghs = self.nghs(y, x, unit_tp);
        let mut ens = Vec::new();
        for n in nghs {
            if self.fld[n.0][n.1] == enemy { ens.push(n) }
        }
    
        if let Some(min_cds) = ens.into_iter().min_by_key(|&cds| self.units[&cds].hp) {
            let unit = self.units.get_mut(&min_cds).unwrap();
            unit.hp -= unit_dg;
    
            if unit.hp <= 0 {
                self.units.remove(&min_cds);
                self.fld[min_cds.0][min_cds.1] = '.';
                return Some(min_cds);
            }
        }
        None
    }
    fn battle_ended(&self) -> bool {
        self.units.values().map(|u| u.tp).all_equal()
    }
    fn remaining_hp(&self) -> i32 {
        self.units.values().map(|u| u.hp).sum()
    }
    fn round(&mut self) {
        let mut sorted: Vec<_> = self.units.keys().cloned().collect();
        sorted.sort_by_key(|&(y, x)| (y, x));

        let mut dead = Vec::new();
        for p in sorted {
            if dead.contains(&p) { continue }
            if self.battle_ended() { self.rnds -= 1; break; }
            if let Some(unit) = self.units.get(&p) {
                let unit_tp = unit.tp;
                let unit_dg = unit.hit;
                let enemy = match unit_tp { 'G' => 'E', _ => 'G' };
                let next = self.next_step(p.0, p.1, unit_tp, enemy);

                if next != p { self.move_unit(p, next, unit_tp) }
                if let Some(d) = self.attack(next.0, next.1, unit_tp, unit_dg, enemy) { dead.push(d) }
            }
        }
        self.rnds += 1
    }
    fn nghs(&self, y: usize, x: usize, unit_tp: char) -> Vec<(usize, usize)> {
        let mut nghs = Vec::new();
        let no_go = ['#', unit_tp];
        if y > 0 && !no_go.contains(&self.fld[y - 1][x]) { nghs.push((y - 1, x)) }
        if x > 0 && !no_go.contains(&self.fld[y][x - 1]) { nghs.push((y, x - 1)) }
        if x < self.fld[0].len() - 1 && !no_go.contains(&self.fld[y][x + 1]) { nghs.push((y, x + 1)) }
        if y < self.fld.len() - 1 && !no_go.contains(&self.fld[y + 1][x]) { nghs.push((y + 1, x)) }
        nghs
    }
    fn next_step(&self, y: usize, x: usize, unit_tp: char, enemy: char) -> (usize, usize) {
        let mut cache = HashSet::new();
        cache.insert((y, x));
        let nrs = self.nghs(y, x, unit_tp);
        let mut next = HashMap::new();
        let mut q = VecDeque::new();
        for (id, n) in nrs.into_iter().enumerate() {
            if self.fld[n.0][n.1] == enemy { return (y, x) }
            next.insert(id, n);
            q.push_back((n, id));
        }

        while let Some(((y2, x2), d)) = q.pop_front() {
            if !cache.insert((y2, x2)) { continue }

            let nghs = self.nghs(y2, x2, unit_tp);
            for n in nghs {
                if self.fld[n.0][n.1] == enemy { return next[&d] }
                q.push_back(((n.0, n.1), d))
            }
        }
        (y, x)
    }

    fn print_hp(&self) {
        self.units.values().for_each(|u| println!("type: {}, hp: {}", u.tp, u.hp));
    }
    fn move_unit(&mut self, old_p: (usize, usize), new_p: (usize, usize), unit_tp: char) {
        self.fld[old_p.0][old_p.1] = '.';
        self.fld[new_p.0][new_p.1] = unit_tp;

        if let Some(unit) = self.units.remove(&old_p) {
            self.units.insert(new_p, unit);
        }
    }
    fn increase_hit(&mut self, new_hit: i32) {
        self.units.values_mut().for_each(|u| if u.tp == 'E' { u.hit = new_hit })
    }
    fn elves_survived(&self) -> usize {
        self.units.values().filter(|&u| u.tp == 'E').count()
    }
}

fn create_battle(input: &str) -> Battle {
    let fld: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut units = HashMap::new();
    (0..fld.len()).for_each(|y| {
        (0..fld[0].len()).for_each(|x| {
            let cell = fld[y][x];
            if cell == 'G' || cell == 'E' { units.insert((y, x), Unit::new(cell, 3)); }
        })
    });
    let battle = Battle { fld , units, rnds: 0 };
    battle
}

fn part1(input: &str) -> i32 {
    let mut battle = create_battle(input);
    while !battle.battle_ended() {
        battle.round();
    }
    battle.rnds * battle.remaining_hp()
}

fn part2(input: &str) -> i32 {
    let backup = create_battle(input);
    let elves = backup.elves_survived();
    let mut battle = backup.clone();
    for d in 3.. {
        battle.increase_hit(d);
        while !battle.battle_ended() {
            battle.round();
        }
        if battle.elves_survived() == elves { break; }
        battle = backup.clone();
    }
    battle.rnds * battle.remaining_hp()
}


fn main() {
    let input = include_str!("input15.txt");
    println!("{}", part1(input));
   }