use std::fmt;

#[derive(Debug, Clone)]
struct Grid (Vec<Vec<i32>>);

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for &num in row {
                write!(f, "{:>2}", num)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Grid {
    fn new() -> Self {
        Self (vec![vec![0; 300]; 300])
    }
    fn find_pw(&mut self, ser_num: i32, x: usize, y: usize) {
        let rack_id = x as i32 + 11;
        let mut pow_lev = ((y as i32 + 1) * rack_id + ser_num) * rack_id;
        pow_lev = pow_lev % 1000 / 100 - 5;
        self.0[y][x] = pow_lev
    }
    fn populate_grid(&mut self, ser_num: i32) {
        (0..self.0.len()).for_each(|y| {
            (0..self.0[0].len()).for_each(|x| {
                self.find_pw(ser_num, x, y);
            })
        });
    }
    fn find_square_part1(&self) -> (usize, usize, i32) {
        let (x, y, sum) = (0..self.0.len() - 3).flat_map(|y| {
            (0..self.0[0].len() - 3).map(move |x| {
                let sum = (y..y + 3).flat_map(|small_y| {
                    (x..x + 3).map(move |small_x| {
                        self.0[small_y][small_x]
                    })
                }).sum::<i32>();
                (x, y, sum)
            })
        }).max_by_key(|&(_, _, sum)| sum).unwrap();
        (x + 1, y + 1, sum)
    }
    fn find_square_part2(&self) -> (usize, usize, usize, i32) {
        let rows = self.0.len();
        let cols = self.0[0].len();
        let mut prefix_sum = vec![vec![0; cols + 1]; rows + 1];

        for y in 1..=rows {
            for x in 1..=cols {
                prefix_sum[y][x] = self.0[y - 1][x - 1]
                    + prefix_sum[y - 1][x]
                    + prefix_sum[y][x - 1]
                    - prefix_sum[y - 1][x - 1];
            }
        }

        let mut max_sum = i32::MIN;
        let mut result = (0, 0);
        let mut square = 0;

        for area in 1..=rows.min(cols) {
            for y in area..=rows {
                for x in area..=cols {
                    let sum = prefix_sum[y][x]
                        - prefix_sum[y - area][x]
                        - prefix_sum[y][x - area]
                        + prefix_sum[y - area][x - area];

                    if sum > max_sum {
                        max_sum = sum;
                        result = (x - area, y - area);
                        square = area;
                    }
                }
            }
        }

        (result.0 + 1, result.1 + 1, square, max_sum)
    }
}

fn part1(ser_num: i32) -> (usize, usize, usize, i32) {
    let mut grid = Grid::new();
    grid.populate_grid(ser_num);
    let (x, y, sq, sum) = grid.find_square_part2();
    (x, y, sq, sum)
}
fn main() {
    let input = 6878;
    let (x, y, sq, sum) = part1(input);
    println!("{}, {}, {}, {}", x, y, sq, sum);
}