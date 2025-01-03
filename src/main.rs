use crate::days::day1::Day1;
use crate::days::day10::Day10;
use crate::days::day11::Day11;
use crate::days::day12::Day12;
use crate::days::day13::Day13;
use crate::days::day14::Day14;
use crate::days::day15::Day15;
use crate::days::day16::Day16;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use crate::days::day5::Day5;
use crate::days::day6::Day6;
use crate::days::day7::Day7;
use crate::days::day8::Day8;
use crate::days::day9::Day9;
use aoc_2024::{read_input, AocInput, AocSolver};
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod days;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc-2024")]
struct Opt {
    #[structopt(long = "--input-dir")]
    input_dir: String,
    #[structopt(long = "--day", short = "-d")]
    day: Option<usize>,
}

fn main() {
    let opt = Opt::from_args();
    #[rustfmt::skip]
    let mut days: Vec<Box<dyn AocSolver>> = vec![
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
        Box::new(Day4),
        Box::new(Day5),
        Box::new(Day6),
        Box::new(Day7),
        Box::new(Day8),
        Box::new(Day9),
        Box::new(Day10),
        Box::new(Day11),
        Box::new(Day12),
        Box::new(Day13),
        Box::new(Day14),
        Box::new(Day15),
        Box::new(Day16),
    ];
    let input_dir = PathBuf::from(opt.input_dir);
    if let Some(day) = opt.day {
        assert!(day >= 1 && day <= 25);
        solve(&mut days[day - 1], |day| {
            read_input_for_day(&input_dir, day)
        });
    } else {
        days.iter_mut()
            .for_each(|day| solve(day, |day| read_input_for_day(&input_dir, day)));
    }
}

fn solve<F: Fn(u32) -> AocInput>(day: &mut Box<dyn AocSolver>, input_resolver: F) {
    let day_num = day.day();
    let input = input_resolver(day_num);
    println!("Day {}:", day_num);
    println!("  Part 1: {}", day.solve_part1(&input));
    stdout().lock().flush().unwrap();
    println!("  Part 2: {}", day.solve_part2(&input));
    stdout().lock().flush().unwrap();
}

fn read_input_for_day(input_dir: &Path, day: u32) -> AocInput {
    assert!(day >= 1 && day <= 25);
    let name = format!("day{}.txt", day);
    AocInput {
        lines: read_input(&input_dir, &name),
    }
}
