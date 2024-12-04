use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use aoc_2024::AdventOfCodeDay;
use std::io::{stdout, Write};

mod days;

fn main() {
    #[rustfmt::skip]
    let mut days: Vec<Box<dyn AdventOfCodeDay>> = vec![
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
    ];

    days.iter_mut().for_each(|day| {
        let input = day.input();
        println!("Day {}:", input.day);
        println!("  Part 1: {}", day.part1(&input));
        stdout().lock().flush().unwrap();
        println!("  Part 2: {}", day.part2(&input));
        stdout().lock().flush().unwrap();
    });
}
