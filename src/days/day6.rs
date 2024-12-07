use aoc_2024::{AocInput, AocSolver};
use std::collections::HashSet;

pub(crate) struct Day6;

impl AocSolver for Day6 {
    fn day(&self) -> u32 {
        6
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let mut input = parse_input(&input.lines);
        let mut positions = HashSet::new();
        positions.insert((input.guard.x, input.guard.y));
        loop {
            if input.move_guard() {
                if !input.is_guard_within_bounds() {
                    break;
                }

                positions.insert((input.guard.x, input.guard.y));
            }
        }

        positions.len().to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let mut input = parse_input(&input.lines);

        let initial_guard = input.guard;
        let mut count = 0;
        let mut positions = HashSet::new();
        for y in 0..input.grid.len() {
            for x in 0..input.grid[y].len() {
                if input.has_obstruction(x as i32, y as i32) {
                    continue;
                }
                if x as i32 == input.guard.x && y as i32 == input.guard.y {
                    continue;
                }

                input.place_obstruction(x, y);

                // Brute-force FTW! it's slow though :(
                while input.is_guard_within_bounds() {
                    positions.insert((input.guard.x, input.guard.y, input.guard.dir));

                    input.move_guard();
                    if positions.contains(&(input.guard.x, input.guard.y, input.guard.dir)) {
                        count += 1;
                        break;
                    }
                }

                positions.clear();
                input.remove_obstruction(x, y);
                input.guard = initial_guard;
            }
        }

        count.to_string()
    }
}

#[derive(Debug)]
struct Input {
    grid: Box<[Box<[u8]>]>,
    guard: Guard,
}

impl Input {
    fn is_guard_within_bounds(&self) -> bool {
        if self.guard.y < 0 || self.guard.y as usize >= self.grid.len() {
            return false;
        }
        if self.guard.x < 0 || self.guard.x as usize >= self.grid[self.guard.y as usize].len() {
            return false;
        }
        true
    }

    fn place_obstruction(&mut self, x: usize, y: usize) {
        self.grid[y][x] = 1;
    }

    fn remove_obstruction(&mut self, x: usize, y: usize) {
        self.grid[y][x] = 0;
    }

    fn has_obstruction(&self, x: i32, y: i32) -> bool {
        if y < 0 || y as usize >= self.grid.len() {
            return false;
        }
        if x < 0 || x as usize >= self.grid[y as usize].len() {
            return false;
        }

        self.grid[y as usize][x as usize] != 0
    }

    fn move_guard(&mut self) -> bool {
        let last_x = self.guard.x;
        let last_y = self.guard.y;
        match self.guard.dir {
            Direction::UP => {
                if self.has_obstruction(self.guard.x, self.guard.y - 1) {
                    self.guard.dir = Direction::RIGHT;
                } else {
                    self.guard.y -= 1;
                }
            }
            Direction::RIGHT => {
                if self.has_obstruction(self.guard.x + 1, self.guard.y) {
                    self.guard.dir = Direction::DOWN;
                } else {
                    self.guard.x += 1;
                }
            }
            Direction::DOWN => {
                if self.has_obstruction(self.guard.x, self.guard.y + 1) {
                    self.guard.dir = Direction::LEFT;
                } else {
                    self.guard.y += 1;
                }
            }
            Direction::LEFT => {
                if self.has_obstruction(self.guard.x - 1, self.guard.y) {
                    self.guard.dir = Direction::UP;
                } else {
                    self.guard.x -= 1;
                }
            }
        }

        self.guard.x != last_x || self.guard.y != last_y
    }
}

#[derive(Debug, Copy, Clone)]
struct Guard {
    x: i32,
    y: i32,
    dir: Direction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    UP = 0,
    RIGHT = 1,
    DOWN = 3,
    LEFT = 4,
}

fn parse_input(lines: &[String]) -> Input {
    let grid: Box<[Box<[u8]>]> = lines
        .iter()
        .map(|e| e.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();

    let mut guard_x = 0;
    let mut guard_y = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y].chars().nth(x).unwrap() == '^' {
                guard_x = x as i32;
                guard_y = y as i32;
                break;
            }
        }
    }

    Input {
        grid,
        guard: Guard {
            x: guard_x,
            y: guard_y,
            dir: Direction::UP,
        },
    }
}
