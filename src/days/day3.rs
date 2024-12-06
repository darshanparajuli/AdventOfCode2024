use aoc_2024::{AocInput, AocSolver};

pub(crate) struct Day3;

impl AocSolver for Day3 {
    fn day(&self) -> u32 {
        3
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        input
            .lines
            .iter()
            .map(|line| parse_instructions(line.as_str()))
            .flatten()
            .map(|instruction| match instruction {
                Instruction::Mul { x, y } => x * y,
                _ => 0,
            })
            .sum::<i32>()
            .to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let mut sum = 0;
        let mut can_multiply = true;
        input
            .lines
            .iter()
            .map(|line| parse_instructions(line.as_str()))
            .flatten()
            .for_each(|instruction| match instruction {
                Instruction::Mul { x, y } => {
                    if can_multiply {
                        sum += x * y;
                    }
                }
                Instruction::Dont => {
                    can_multiply = false;
                }
                Instruction::Do => {
                    can_multiply = true;
                }
            });

        sum.to_string()
    }
}

enum Instruction {
    Mul { x: i32, y: i32 },
    Do,
    Dont,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut result = vec![];
    let parse_num = |input: &str, start: usize| -> Option<(i32, usize)> {
        let mut i = start;
        let k = i;
        while i < input.len() && input.chars().nth(i).unwrap().is_ascii_digit() {
            i += 1;
        }
        if k == i {
            return None;
        }
        input[k..i].parse::<i32>().ok().map(|n| (n, i))
    };

    let mut i = 0;
    while i < input.len() {
        if i < input.len() - 4 && &input[i..i + 4] == "do()" {
            result.push(Instruction::Do);
            i += 4;
            continue;
        }

        if i < input.len() - 7 && &input[i..i + 7] == "don't()" {
            result.push(Instruction::Dont);
            i += 4;
            continue;
        }

        if i < input.len() - 4 && &input[i..i + 4] == "mul(" {
            i += 4;

            let x = parse_num(input, i);
            if x.is_none() {
                i += 1;
                continue;
            }
            let (x, next) = x.unwrap();
            i = next;

            if i < input.len() && input.chars().nth(i).unwrap() != ',' {
                i += 1;
                continue;
            }
            i += 1;

            let y = parse_num(input, i);
            if y.is_none() {
                i += 1;
                continue;
            }
            let (y, next) = y.unwrap();
            i = next;
            if i < input.len() && input.chars().nth(i).unwrap() != ')' {
                i += 1;
                continue;
            }
            i += 1;

            result.push(Instruction::Mul { x, y });
        } else {
            i += 1;
        }
    }
    result
}
