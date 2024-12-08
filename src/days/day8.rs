use aoc_2024::{AocInput, AocSolver};
use std::collections::{HashMap, HashSet};

pub(crate) struct Day8;

impl AocSolver for Day8 {
    fn day(&self) -> u32 {
        8
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let grid = parse_input(&input);
        let antennas = get_antennas(&grid);
        let mut locations = HashSet::new();
        for (_, v) in &antennas {
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    let (x1, y1) = v[i];
                    let (x2, y2) = v[j];
                    let dy = y2 - y1;
                    let dx = x2 - x1;
                    let loc1 = (x1 - dx, y1 - dy);
                    let loc2 = (x2 + dx, y2 + dy);
                    locations.insert(loc1);
                    locations.insert(loc2);
                }
            }
        }

        count_valid_locations(&grid, &locations).to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let grid = parse_input(&input);
        let antennas = get_antennas(&grid);

        let mut locations = HashSet::new();
        for (_, v) in &antennas {
            for i in 0..v.len() {
                for j in i + 1..v.len() {
                    let (x1, y1) = v[i];
                    let (x2, y2) = v[j];

                    locations.insert((x1, y1));
                    locations.insert((x2, y2));

                    let dy = y2 - y1;
                    let dx = x2 - x1;

                    let (mut loc1_x, mut loc1_y) = (x1 - dx, y1 - dy);
                    loop {
                        locations.insert((loc1_x, loc1_y));
                        loc1_x -= dx;
                        loc1_y -= dy;
                        if loc1_y < 0 || loc1_y as usize >= grid.len() {
                            break;
                        }
                        if loc1_x < 0 || loc1_x as usize >= grid[loc1_y as usize].len() {
                            break;
                        }
                    }

                    let (mut loc2_x, mut loc2_y) = (x2 + dx, y2 + dy);
                    loop {
                        locations.insert((loc2_x, loc2_y));
                        loc2_x += dx;
                        loc2_y += dy;
                        if loc2_y < 0 || loc2_y as usize >= grid.len() {
                            break;
                        }
                        if loc2_x < 0 || loc2_x as usize >= grid[loc2_y as usize].len() {
                            break;
                        }
                    }
                }
            }
        }

        count_valid_locations(&grid, &locations).to_string()
    }
}

fn parse_input(input: &AocInput) -> Box<[Box<[char]>]> {
    input.lines.iter().map(|s| s.chars().collect()).collect()
}

fn get_antennas(grid: &[Box<[char]>]) -> HashMap<char, Vec<(i64, i64)>> {
    let mut antennas = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cell = grid[y][x];
            if cell != '.' {
                // Found an antenna.
                antennas
                    .entry(cell)
                    .or_insert_with(Vec::new)
                    .push((x as i64, y as i64));
            }
        }
    }

    antennas
}

fn count_valid_locations(grid: &[Box<[char]>], locations: &HashSet<(i64, i64)>) -> usize {
    locations
        .iter()
        .filter(|(x, y)| {
            let x = *x;
            let y = *y;
            y >= 0 && (y as usize) < grid.len() && x >= 0 && (x as usize) < grid[y as usize].len()
        })
        .count()
}
