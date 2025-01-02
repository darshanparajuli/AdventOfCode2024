use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub mod raw_ptr;

pub fn read_input(input_dir: &Path, name: &str) -> Vec<String> {
    read_input_map(input_dir, name, |e| e.to_owned())
}

pub fn read_input_map<T>(input_dir: &Path, name: &str, mapper: fn(&str) -> T) -> Vec<T> {
    match File::open(input_dir.join(name)) {
        Ok(f) => BufReader::new(f)
            .lines()
            .map(|line| mapper(&line.unwrap()))
            .collect(),
        Err(e) => {
            eprintln!("Error reading input file '{}': {}", input_dir.display(), e);
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

    let bi = nums;
    let n = mods.iter().product::<u64>();

    let xi = mods.iter().map(|e| n / e).enumerate().map(|(i, n)| {
        let m = mods[i];

        let mut count = 1;
        while (n * count) % m != 1 {
            count += 1;
        }

        count
    });

    let bi_ni_xi = xi
        .enumerate()
        .map(|(i, v)| bi[i].wrapping_mul((n / mods[i]) * v));

    let bi_ni_xi_sum = bi_ni_xi.fold(0u64, |acc, x| acc.wrapping_add(x));
    bi_ni_xi_sum % n
}

pub fn gcd(a: u64, b: u64) -> u64 {
    assert!(a != 0 || b != 0);
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

pub fn lcm(a: u64, b: u64) -> u64 {
    assert!(a != 0 && b != 0);
    (a * b) / gcd(a, b)
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
pub struct AocInput {
    pub lines: Vec<String>,
}

pub trait AocSolver {
    fn day(&self) -> u32;
    fn solve_part1(&mut self, input: &AocInput) -> String;
    fn solve_part2(&mut self, input: &AocInput) -> String;
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

    #[test]
    fn gcd_test() {
        assert!(gcd(8, 12) == 4);
    }

    #[test]
    fn lcm_test() {
        assert!(lcm(4, 6) == 12);
    }
}
