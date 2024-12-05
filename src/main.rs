use crate::days::day1::Day1;
use crate::days::day2::Day2;
use crate::days::day3::Day3;
use crate::days::day4::Day4;
use aoc_2024::{AocContext, AocDay};
use std::io::{stdout, Write};
use std::path::PathBuf;
use structopt::StructOpt;
use crate::days::day5::Day5;

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
    let mut days: Vec<Box<dyn AocDay>> = vec![
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
        Box::new(Day4),
        Box::new(Day5),
    ];
    let context = AocContext {
        input_dir: PathBuf::from(opt.input_dir),
    };
    if let Some(day) = opt.day {
        assert!(day >= 1 && day <= 25);
        solve(&context, &mut days[day - 1]);
    } else {
        days.iter_mut().for_each(|day| solve(&context, day));
    }
}

fn solve(context: &AocContext, day: &mut Box<dyn AocDay>) {
    let input = day.input(context);
    println!("Day {}:", input.day);
    println!("  Part 1: {}", day.part1(&input));
    stdout().lock().flush().unwrap();
    println!("  Part 2: {}", day.part2(&input));
    stdout().lock().flush().unwrap();
}
