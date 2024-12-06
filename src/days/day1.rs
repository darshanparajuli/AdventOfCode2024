use aoc_2024::{AocInput, AocSolver};
use std::collections::HashMap;

pub(crate) struct Day1;

impl AocSolver for Day1 {
    fn day(&self) -> u32 {
        1
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let mut left_numbers = vec![];
        let mut right_numbers = vec![];
        for line in &input.lines {
            let mut split_itr = line.split_ascii_whitespace();
            left_numbers.push(split_itr.next().unwrap().parse::<i64>().unwrap());
            right_numbers.push(split_itr.next().unwrap().parse::<i64>().unwrap());
        }

        left_numbers.sort();
        right_numbers.sort();

        left_numbers
            .iter()
            .copied()
            .zip(right_numbers.iter().copied())
            .map(|(a, b)| (a - b).abs())
            .sum::<i64>()
            .to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let mut left_numbers = vec![];
        let mut right_numbers = HashMap::new();
        for line in &input.lines {
            let mut split_itr = line.split_ascii_whitespace();
            left_numbers.push(split_itr.next().unwrap().parse::<i64>().unwrap());

            let right_n = split_itr.next().unwrap().parse::<i64>().unwrap();
            right_numbers.insert(right_n, right_numbers.get(&right_n).unwrap_or(&0) + 1);
        }

        left_numbers
            .iter()
            .map(|e| e * right_numbers.get(e).unwrap_or(&0))
            .sum::<i64>()
            .to_string()
    }
}
