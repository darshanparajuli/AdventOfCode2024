use std::io::{stdout, Write};
use crate::days::day1::Day1;
use aoc_2024::AdventOfCodeDay;

mod days;

fn main() {
    let mut days: Vec<Box<dyn AdventOfCodeDay>> = vec![Box::new(Day1)];
    days.iter_mut().for_each(|day| {
        let input = day.input();
        println!("Part 1: {}", day.part1(input.clone()));
        stdout().lock().flush().unwrap();
        println!("Part 2: {}", day.part2(input));
        stdout().lock().flush().unwrap();
    });
}
