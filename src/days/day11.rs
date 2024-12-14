use aoc_2024::{AocInput, AocSolver};
use std::collections::HashMap;

pub(crate) struct Day11;

impl AocSolver for Day11 {
    fn day(&self) -> u32 {
        11
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        count_stones(&parse_input(&input), 25).to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        count_stones(&parse_input(&input), 75).to_string()
    }
}

fn parse_input(input: &AocInput) -> Vec<u64> {
    input
        .lines
        .first()
        .unwrap()
        .split_ascii_whitespace()
        .map(|e| e.parse().unwrap())
        .collect()
}

fn digits_count(mut n: u64) -> u32 {
    let mut count = 0;
    while n != 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn count_stones(stones: &[u64], number_of_times: usize) -> usize {
    let mut map = HashMap::new();
    for s in stones {
        *map.entry(*s).or_insert(0) += 1;
    }

    let mut temp_next = vec![];
    for _ in 0..number_of_times {
        for (n, count) in map.drain() {
            let digits_num = digits_count(n);
            if n == 0 {
                temp_next.push((1, count));
            } else if digits_num % 2 == 0 {
                let mut left_half = n;
                let mut right_half = 0;
                for i in 0..digits_num / 2 {
                    right_half += (left_half % 10) * 10u64.pow(i);
                    left_half /= 10;
                }
                temp_next.push((left_half, count));
                temp_next.push((right_half, count));
            } else {
                let n = n * 2024;
                temp_next.push((n, count));
            }
        }
        for (n, count) in temp_next.drain(..) {
            map.entry(n).and_modify(|e| *e += count).or_insert(count);
        }
    }

    map.values().sum()
}
