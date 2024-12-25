use aoc_2024::{AocInput, AocSolver, Vec2};

pub(crate) struct Day14;

const COLS: usize = 101;
const ROWS: usize = 103;

impl AocSolver for Day14 {
    fn day(&self) -> u32 {
        14
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let mut robots = parse_input(input);
        for _ in 0..100 {
            robots.iter_mut().for_each(|r| {
                let n = next_position(r);
                r.pos = n;
            });
        }

        let mut top_left = 0;
        let mut top_right = 0;
        let mut bottom_left = 0;
        let mut bottom_right = 0;
        for r in robots {
            let x = r.pos.x;
            let y = r.pos.y;
            if x < COLS / 2 && y < ROWS / 2 {
                top_left += 1;
            } else if x > COLS / 2 && x < COLS && y < ROWS / 2 {
                top_right += 1;
            } else if x < COLS / 2 && y > ROWS / 2 && y < ROWS {
                bottom_left += 1;
            } else if x > COLS / 2 && x < COLS && y > ROWS / 2 && y < ROWS {
                bottom_right += 1;
            }
        }

        (top_left * top_right * bottom_left * bottom_right).to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let mut robots = parse_input(input);
        let mut grid = [[-1; COLS]; ROWS];

        robots.iter().for_each(|r| {
            grid[r.pos.y][r.pos.x] = 1;
        });

        let mut t = 1;
        loop {
            robots.iter_mut().for_each(|r| {
                let n = next_position(r);
                r.pos = n;
            });

            grid.fill([-1; COLS]);
            robots.iter().for_each(|r| {
                grid[r.pos.y][r.pos.x] = 1;
            });

            if maybe_has_christmas_tree(&grid) {
                for y in 0..grid.len() {
                    for x in 0..grid[y].len() {
                        if grid[y][x] != -1 {
                            print!("X");
                        } else {
                            print!(".");
                        }
                    }
                    println!();
                }
                break;
            }

            t += 1;
        }
        t.to_string()
    }
}

fn maybe_has_christmas_tree(grid: &[[i32; COLS]]) -> bool {
    fn match_pattern(grid: &[[i32; COLS]], x: usize, mut y: usize) -> usize {
        let mut k = 0;
        while y < ROWS {
            let mut matches = true;
            for col in ((x as i64) - k)..=((x as i64) + k) {
                if col >= 0 && col < COLS as i64 {
                    let col = col as usize;
                    if grid[y][col] == -1 {
                        matches = false;
                        break;
                    }
                } else {
                    matches = false;
                    break;
                }
            }

            if !matches {
                break;
            }

            k += 1;
            y += 1;
        }
        k as usize
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if v != -1 {
                if match_pattern(grid, x, y) > 4 {
                    return true;
                }
            }
        }
    }
    false
}

fn next_position(r: &Robot) -> Vec2<usize> {
    let mut x = r.pos.x as i64 + r.vel.x;
    if x >= COLS as i64 {
        x = x % COLS as i64;
    } else if x < 0 {
        x = COLS as i64 - (x.abs() % COLS as i64);
    }
    let mut y = r.pos.y as i64 + r.vel.y;
    if y >= ROWS as i64 {
        y = y % ROWS as i64;
    } else if y < 0 {
        y = ROWS as i64 - (y.abs() % ROWS as i64);
    }
    Vec2 {
        x: x as usize,
        y: y as usize,
    }
}

#[derive(Debug)]
struct Robot {
    pos: Vec2<usize>,
    vel: Vec2<i64>,
}

fn parse_input(input: &AocInput) -> Box<[Robot]> {
    input
        .lines
        .iter()
        .map(|e| {
            let mut it = e.split(" ");
            let first_part = &it.next().unwrap()[2..];
            let second_part = &it.next().unwrap()[2..];

            let mut it = first_part.split(",");
            let pos = Vec2::new(
                it.next().unwrap().parse::<usize>().unwrap(),
                it.next().unwrap().parse::<usize>().unwrap(),
            );

            let mut it = second_part.split(",");
            let vel = Vec2::new(
                it.next().unwrap().parse::<i64>().unwrap(),
                it.next().unwrap().parse::<i64>().unwrap(),
            );

            Robot { pos, vel }
        })
        .collect()
}
