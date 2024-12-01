use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

pub mod raw_ptr;

pub fn read_input(name: &str) -> Vec<String> {
    read_input_map(name, |e| e.to_owned())
}

pub fn read_input_map<T>(name: &str, mapper: fn(&str) -> T) -> Vec<T> {
    let input_dir = env::args().nth(1).unwrap();
    match File::open(PathBuf::from(&input_dir).join(name)) {
        Ok(f) => BufReader::new(f)
            .lines()
            .map(|line| mapper(&line.unwrap()))
            .collect(),
        Err(e) => {
            eprintln!("Error reading input file '{}': {}", input_dir, e);
            std::process::exit(1);
        }
    }
}

/// Chinese Remainder Theorem
///
/// `nums` contains `a`s and `mods` contains `m`s in:
/// `x = a (mod m)`
pub fn crt(nums: &[u64], mods: &[u64]) -> u64 {
    assert_eq!(nums.len(), mods.len());

    let bi = nums.iter().copied().collect::<Vec<_>>();
    let n = mods.iter().copied().product::<u64>();
    let ni = mods.iter().map(|e| n / e).collect::<Vec<_>>();

    let xi = ni
        .iter()
        .enumerate()
        .map(|(i, n)| {
            let m = mods[i];

            let mut count = 1;
            while (n * count) % m != 1 {
                count += 1;
            }

            count
        })
        .collect::<Vec<_>>();

    let bi_ni_xi = (0..mods.len())
        .map(|i| bi[i].wrapping_mul(ni[i] * xi[i]))
        .collect::<Vec<_>>();

    let bi_ni_xi_sum = bi_ni_xi.iter().fold(0u64, |acc, x| acc.wrapping_add(*x));
    bi_ni_xi_sum % n
}

pub fn print_part1_answer<T>(result: T)
where
    T: Display,
{
    println!("part 1: {}", result);
}

pub fn print_part2_answer<T>(result: T)
where
    T: Display,
{
    println!("part 2: {}", result);
}

#[derive(Debug, Clone)]
pub struct AdventOfCodeDayInput {
    pub day: u32,
    pub lines: Vec<String>,
}

pub trait AdventOfCodeDay {
    fn input(&self) -> AdventOfCodeDayInput;
    fn part1(&mut self, input: &AdventOfCodeDayInput) -> String;
    fn part2(&mut self, input: &AdventOfCodeDayInput) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_crt() {
        let nums = [67, 6, 57, 58];
        let mods = [67, 7, 59, 61];
        assert_eq!(crt(&nums, &mods), 754018);
    }
}
