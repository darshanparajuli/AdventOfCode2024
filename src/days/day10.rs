use aoc_2024::{AocInput, AocSolver};
use std::collections::HashSet;

pub(crate) struct Day10;

impl AocSolver for Day10 {
    fn day(&self) -> u32 {
        10
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let input = parse_input(&input);

        fn dfs(
            input: &[Box<[i32]>],
            x: usize,
            y: usize,
            count: &mut usize,
            visited: &mut HashSet<(usize, usize)>,
        ) {
            visited.insert((x, y));
            let height = input[y][x];
            if height == 9 {
                *count += 1;
                return;
            }

            let neighbors = [
                (x as isize + 1, y as isize),
                (x as isize - 1, y as isize),
                (x as isize, y as isize + 1),
                (x as isize, y as isize - 1),
            ];

            neighbors
                .iter()
                .filter(|(nx, ny)| {
                    let nx = *nx;
                    let ny = *ny;
                    ny >= 0
                        && (ny as usize) < input.len()
                        && nx >= 0
                        && (nx as usize) < input[ny as usize].len()
                })
                .filter(|(nx, ny)| input[*ny as usize][*nx as usize] != -1)
                .filter(|(nx, ny)| input[*ny as usize][*nx as usize] == height + 1)
                .for_each(|(nx, ny)| {
                    if !visited.contains(&(*nx as usize, *ny as usize)) {
                        dfs(&input, *nx as usize, *ny as usize, count, visited);
                    }
                });
        }

        let mut sum = 0;

        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] == 0 {
                    let mut count = 0;
                    let mut visited = HashSet::new();
                    visited.insert((x, y));
                    dfs(&input, x, y, &mut count, &mut visited);
                    sum += count;
                }
            }
        }

        sum.to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let input = parse_input(&input);

        fn dfs(
            input: &[Box<[i32]>],
            x: usize,
            y: usize,
            count: &mut usize,
            visited: &mut HashSet<(usize, usize)>,
        ) {
            visited.insert((x, y));
            let height = input[y][x];
            if height == 0 {
                *count += 1;
                return;
            }

            let neighbors = [
                (x as isize + 1, y as isize),
                (x as isize - 1, y as isize),
                (x as isize, y as isize + 1),
                (x as isize, y as isize - 1),
            ];

            neighbors
                .iter()
                .filter(|(nx, ny)| {
                    let nx = *nx;
                    let ny = *ny;
                    ny >= 0
                        && (ny as usize) < input.len()
                        && nx >= 0
                        && (nx as usize) < input[ny as usize].len()
                })
                .filter(|(nx, ny)| input[*ny as usize][*nx as usize] != -1)
                .filter(|(nx, ny)| input[*ny as usize][*nx as usize] == height - 1)
                .for_each(|(nx, ny)| {
                    if !visited.contains(&(*nx as usize, *ny as usize)) {
                        let mut visited = visited.clone();
                        dfs(&input, *nx as usize, *ny as usize, count, &mut visited);
                    }
                });
        }

        let mut sum = 0;
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                if input[y][x] == 9 {
                    let mut visited = HashSet::new();
                    let mut count = 0;
                    visited.insert((x, y));
                    dfs(&input, x, y, &mut count, &mut visited);
                    sum += count;
                }
            }
        }

        sum.to_string()
    }
}

fn parse_input(input: &AocInput) -> Box<[Box<[i32]>]> {
    input
        .lines
        .iter()
        .map(|s| {
            s.chars()
                .map(|e| {
                    if e == '.' {
                        -1
                    } else {
                        e.to_digit(10).unwrap() as i32
                    }
                })
                .collect()
        })
        .collect()
}
