use aoc_2024::{AocInput, AocSolver};

pub(crate) struct Day2;

impl AocSolver for Day2 {
    fn day(&self) -> u32 {
        2
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let reports = parse_input(&input.lines);
        reports
            .iter()
            .filter(|r| is_level_safe(r))
            .count()
            .to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let mut reports = parse_input(&input.lines);
        let mut count = 0;
        for report in &mut reports {
            if is_level_safe(&report) {
                count += 1;
            } else {
                let len = report.len();
                for i in 0..len {
                    let level = report.remove(i);
                    if is_level_safe(&report) {
                        count += 1;
                        break;
                    } else {
                        report.insert(i, level);
                    }
                }
            }
        }

        count.to_string()
    }
}

fn parse_input(lines: &[String]) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_level_safe(report: &[i32]) -> bool {
    if !report.is_sorted_by(|a, b| a < b) && !report.is_sorted_by(|a, b| a > b) {
        return false;
    }

    for i in 0..report.len() - 1 {
        let a = report[i];
        let b = report[i + 1];
        let dist = (a - b).abs();
        if dist < 1 || dist > 3 {
            return false;
        }
    }

    true
}
