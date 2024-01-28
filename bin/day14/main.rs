#[derive(Debug)]
struct Recipes {
    board: Vec<usize>,
    elfs: [usize; 2],
}

impl Recipes {
    fn new() -> Self {
        Self {
            board: vec![3, 7],
            elfs: [0, 1],
        }
    }
    fn add_rps(&mut self) {
        let new: usize = self.elfs.iter().map(|&elf| self.board[elf]).sum();
        if new > 9 { self.board.push(new / 10) }
        self.board.push(new % 10);
    }
    fn pick_rps(&mut self) {
        for i in self.elfs.iter_mut() {*i = (*i + self.board[*i] as usize + 1) % self.board.len()}
    }
    fn round(&mut self) {
        self.add_rps();
        self.pick_rps();
    }
    fn find_score(&self, rps: usize) -> u64 {
        let slice = self.board[rps..rps + 10].to_vec();
        slice.iter().fold(0, |acc, &d| acc * 10 + d as u64)
    }
    fn find_rps(&mut self, rps: &str) -> usize {
        let slice: Vec<usize> = rps.split("").filter_map(|c| c.parse::<usize>().ok()).collect();

        loop {
            self.round();
            let len = self.board.len();
            if len > slice.len() {
                for i in 0..=1 {
                    if self.board[len - slice.len() - i..len - i] == slice {
                        return len - slice.len() - i;
                    }
                }
            }
        }
    }
}

fn part1(rps: &str) -> u64 {
    let mut board = Recipes::new();
    let rps = rps.parse::<usize>().unwrap();
    while board.board.len() < rps + 10 {board.round()}
    board.find_score(rps)
}

fn part2(rps: &str) -> usize {
    let mut board = Recipes::new();
    board.find_rps(rps)
}

fn main() {
    println!("{}", part2("633601"));
}