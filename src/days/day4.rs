use aoc_2024::{read_input, AdventOfCodeDay, AdventOfCodeDayInput};
use std::collections::HashSet;

pub(crate) struct Day4;

impl AdventOfCodeDay for Day4 {
    fn input(&self) -> AdventOfCodeDayInput {
        AdventOfCodeDayInput {
            day: 4,
            lines: read_input("day4.txt"),
        }
    }

    fn part1(&mut self, input: &AdventOfCodeDayInput) -> String {
        let mut values = HashSet::new();
        for y in 0..input.lines.len() {
            for x in 0..input.lines[y].len() {
                find_xmas(&input.lines, x, y).into_iter().for_each(|e| {
                    values.insert(e);
                });
            }
        }
        values.len().to_string()
    }

    fn part2(&mut self, input: &AdventOfCodeDayInput) -> String {
        let mut values = HashSet::new();
        for y in 0..input.lines.len() {
            for x in 0..input.lines[y].len() {
                find_xmas_2(&input.lines, x, y).into_iter().for_each(|e| {
                    values.insert(e);
                });
            }
        }
        values.len().to_string()
    }
}

fn find_xmas(grid: &[String], x: usize, y: usize) -> Vec<[(usize, usize); 4]> {
    let mut result = vec![];

    // right
    if x + 3 < grid[y].len() {
        if &grid[y][x..x + 4] == "XMAS" || &grid[y][x..x + 4] == "SAMX" {
            result.push([(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]);
        }
    }

    // down
    if y + 3 < grid.len() {
        let chars = [
            grid[y + 0].chars().nth(x).unwrap(),
            grid[y + 1].chars().nth(x).unwrap(),
            grid[y + 2].chars().nth(x).unwrap(),
            grid[y + 3].chars().nth(x).unwrap(),
        ];
        if chars == ['X', 'M', 'A', 'S'] || chars == ['S', 'A', 'M', 'X'] {
            result.push([(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]);
        }
    }

    // diagonal \
    if y + 3 < grid.len() && x + 3 < grid[y].len() {
        let chars = [
            grid[y + 0].chars().nth(x + 0).unwrap(),
            grid[y + 1].chars().nth(x + 1).unwrap(),
            grid[y + 2].chars().nth(x + 2).unwrap(),
            grid[y + 3].chars().nth(x + 3).unwrap(),
        ];
        if chars == ['X', 'M', 'A', 'S'] || chars == ['S', 'A', 'M', 'X'] {
            result.push([(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)]);
        }
    }

    // diagonal /
    if y + 3 < grid.len() && (x as i32) - 3 >= 0 {
        let chars = [
            grid[y + 0].chars().nth(x - 0).unwrap(),
            grid[y + 1].chars().nth(x - 1).unwrap(),
            grid[y + 2].chars().nth(x - 2).unwrap(),
            grid[y + 3].chars().nth(x - 3).unwrap(),
        ];
        if chars == ['X', 'M', 'A', 'S'] || chars == ['S', 'A', 'M', 'X'] {
            result.push([(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)]);
        }
    }

    result
}

fn find_xmas_2(grid: &[String], x: usize, y: usize) -> Option<[(usize, usize); 6]> {
    let mut diag1 = None;
    // diagonal \
    if y + 2 < grid.len() && x + 2 < grid[y].len() {
        let chars = [
            grid[y + 0].chars().nth(x + 0).unwrap(),
            grid[y + 1].chars().nth(x + 1).unwrap(),
            grid[y + 2].chars().nth(x + 2).unwrap(),
        ];
        if chars == ['M', 'A', 'S'] || chars == ['S', 'A', 'M'] {
            diag1 = Some([(x, y), (x + 1, y + 1), (x + 2, y + 2)]);
        }
    }

    // diagonal /
    let mut diag2 = None;
    if y + 2 < grid.len() && x + 2 < grid[y].len() {
        let chars = [
            grid[y + 0].chars().nth(x + 2).unwrap(),
            grid[y + 1].chars().nth(x + 1).unwrap(),
            grid[y + 2].chars().nth(x + 0).unwrap(),
        ];
        if chars == ['M', 'A', 'S'] || chars == ['S', 'A', 'M'] {
            diag2 = Some([(x + 2, y), (x + 1, y + 1), (x, y + 2)]);
        }
    }

    if let Some(d1) = diag1 {
        if let Some(d2) = diag2 {
            return Some([d1[0], d1[1], d1[2], d2[0], d2[1], d2[2]]);
        }
    }

    None
}
