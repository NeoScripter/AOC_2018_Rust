use std::collections::VecDeque;

fn part1(players: usize, marbles: u32) -> u32 {
    let mut curr_player = 1;
    let mut players = vec![0; players];
    let mut circle: VecDeque<u32> = (0..2).collect();
    for curr_marble in 2..=marbles {
        curr_player += 1;
        let curr_player_idx = curr_player % players.len();
        if curr_marble % 23 == 0 {
            circle.rotate_right(7);
            let removed = circle.pop_front().unwrap();
            players[curr_player_idx] += curr_marble + removed;
            continue;
        }
        circle.rotate_left(2);
        circle.push_front(curr_marble);
    }
    players.into_iter().max().unwrap()
}
fn main() {
    let players = 452;
    let marbles = 7125000;
    println!("{}", part1(players, marbles));
}