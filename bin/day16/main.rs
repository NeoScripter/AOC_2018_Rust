use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map_res,
    multi::separated_list0,
    sequence::{delimited, preceded},
    IResult,
};

const CTGS: [&str; 16] = ["addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr"];

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse::<usize>)(input)
}

fn parse_array(input: &str) -> IResult<&str, Vec<usize>> {
    preceded(
        alt((tag("Before: "), tag("After:  "))),
        delimited(
            char('['),
            separated_list0(tag(", "), parse_usize),
            char(']'),
        ),
    )(input)
}

fn parse_rgts(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list0(tag(" "), parse_usize)(input)
}

fn parse_cmd(input: &str) -> IResult<&str, Vec<usize>> {
    alt((parse_array, parse_rgts))(input)
}

fn vec_to_array(vec: Vec<usize>) -> Result<[usize; 4], &'static str> {
    vec.try_into().map_err(|_| "Length mismatch")
}

#[derive(Debug, Clone)]
struct Cmd {
    before: [usize; 4],
    cmd: [usize; 4],
    after: [usize; 4],
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let mut cmds = Vec::new();
        while let Some(l) = iter.next() {
            if let Ok((_, v)) = parse_cmd(l) {
                let arr = vec_to_array(v).unwrap();
                cmds.push(arr)
            }
        }
        Ok(Cmd { before: cmds[0], cmd: cmds[1], after: cmds[2] })
    }
}

#[derive(Debug, Clone)]
struct Device<'a> {
    rgts: [usize; 4],
    cmds: Vec<Cmd>,
    ctgs: HashMap<usize, &'a str>,
}

impl<'a> Device<'a> {
    fn new() -> Self {
        Self {
            rgts: [0; 4],
            cmds: Vec::new(),
            ctgs: HashMap::new(),
        }
    }
    fn execute_opcode(&mut self, cmd: [usize; 4], mut ctg: &'a str, part2: bool) {
        if part2 { ctg = self.ctgs[&cmd[0]] }
        let d = &mut self.rgts;
        d[cmd[3]] = match ctg {
            "addr" => d[cmd[1]] + d[cmd[2]],
            "addi" => d[cmd[1]] + cmd[2],
            "mulr" => d[cmd[1]] * d[cmd[2]],
            "muli" => d[cmd[1]] * cmd[2],
            "banr" => d[cmd[1]] & d[cmd[2]],
            "bani" => d[cmd[1]] & cmd[2],
            "borr" => d[cmd[1]] | d[cmd[2]],
            "bori" => d[cmd[1]] | cmd[2],
            "setr" => d[cmd[1]],
            "seti" => cmd[1],
            "gtir" => if cmd[1] > d[cmd[2]] {1} else {0},
            "gtri" => if d[cmd[1]] > cmd[2] {1} else {0},
            "gtrr" => if d[cmd[1]] > d[cmd[2]] {1} else {0},
            "eqir" => if cmd[1] == d[cmd[2]] {1} else {0},
            "eqri" => if d[cmd[1]] == cmd[2] {1} else {0},
            "eqrr" => if d[cmd[1]] == d[cmd[2]] {1} else {0},
            _ => !unreachable!(),
        }
    }
    fn count_opcodes(&mut self) -> usize {
        let cmds = self.cmds.clone(); 
        let mut map: HashMap<usize, HashSet<&str>> = HashMap::new();
        let mut p1 = 0;
        for cmd in cmds.iter() {
            let mut count = 0;
            let mut set = HashSet::new();
            for ct in CTGS {
                self.rgts = cmd.before;
                self.execute_opcode(cmd.cmd, ct, false);
                if self.rgts == cmd.after { set.insert(ct); count += 1; }
            };
            map.entry(cmd.cmd[0])
            .and_modify(|e| *e = e.intersection(&set).cloned().collect())
            .or_insert(set);
            if count >= 3 { p1 += 1 }
        };
        while self.ctgs.len() < 16 {
            for (&k, v) in map.iter() {
                if v.len() == 1 { self.ctgs.insert(k, v.into_iter().next().unwrap());}
            }
            for &single in self.ctgs.values() {
                map.values_mut().for_each(|set| {set.remove(single);})
            }
        }
        p1
    }
}

fn parse_input<'a>() -> (&'a str, Device<'a>) {
    let input = include_str!("input16.txt");
    let (cmds, test_program) = input.split_once("\r\n\r\n\r\n\r\n").unwrap();
    let mut device = Device::new();
    for c in cmds.split("\r\n\r\n") { device.cmds.push(c.parse().unwrap()) }
    (test_program, device)
}

fn solve() -> (usize, usize) {
    let (pr, mut device) = parse_input();
    let p1 = device.count_opcodes();
    device.rgts = [0; 4];
    for line in pr.lines() {
        let cmd: Vec<usize> = line.split_ascii_whitespace().filter_map(|c| c.parse().ok()).collect();
        device.execute_opcode(vec_to_array(cmd).unwrap(), "", true);
    }
    (p1, device.rgts[0])
}

fn main() {
    let (p1, p2) = solve();
    println!("part 1: {}, part 2: {}", p1, p2);
}