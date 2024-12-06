use aoc_2024::{AocInput, AocSolver};

pub(crate) struct Day5;

impl AocSolver for Day5 {
    fn day(&self) -> u32 {
        5
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let input = parse_input(&input.lines);
        let mut sum = 0;
        for page_nums in &input.page_numbers_list {
            let is_correct_order = input.rules.iter().all(|&(a, b)| {
                let a_index = page_nums.iter().position(|e| e == &a);
                let b_index = page_nums.iter().position(|e| e == &b);
                if a_index.is_some() && b_index.is_some() {
                    let a_index = a_index.unwrap();
                    let b_index = b_index.unwrap();
                    if a_index > b_index {
                        return false;
                    }
                }
                return true;
            });
            if is_correct_order {
                sum += page_nums[page_nums.len() / 2];
            }
        }

        sum.to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let mut input = parse_input(&input.lines);
        let mut sum = 0;
        for page_nums in &mut input.page_numbers_list {
            let mut fixed = false;
            loop {
                let mut needed_fixing = false;
                input.rules.iter().for_each(|&(a, b)| {
                    let a_index = page_nums.iter().position(|e| e == &a);
                    let b_index = page_nums.iter().position(|e| e == &b);
                    if a_index.is_some() && b_index.is_some() {
                        let a_index = a_index.unwrap();
                        let b_index = b_index.unwrap();
                        if a_index > b_index {
                            page_nums.swap(a_index, b_index);
                            fixed = true;
                            needed_fixing = true;
                        }
                    }
                });
                if !needed_fixing {
                    break;
                }
            }
            if fixed {
                sum += page_nums[page_nums.len() / 2];
            }
        }

        sum.to_string()
    }
}

#[derive(Debug)]
struct Input {
    rules: Vec<(i32, i32)>,
    page_numbers_list: Vec<Vec<i32>>,
}

fn parse_input(lines: &[String]) -> Input {
    let divider = lines.iter().position(|l| l.is_empty()).unwrap();
    let rules = lines
        .iter()
        .take(divider)
        .map(|line| {
            let mut it = line.split('|');
            let a: i32 = it.next().unwrap().parse().unwrap();
            let b: i32 = it.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect();

    let page_numbers = lines
        .iter()
        .skip(divider + 1)
        .map(|l| l.split(",").map(|e| e.parse::<i32>().unwrap()).collect())
        .collect();

    Input {
        rules,
        page_numbers_list: page_numbers,
    }
}
