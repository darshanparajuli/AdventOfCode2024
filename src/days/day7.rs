use aoc_2024::{AocInput, AocSolver};

pub(crate) struct Day7;

impl AocSolver for Day7 {
    fn day(&self) -> u32 {
        7
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let equations = parse_input(input);
        let mut sum = 0;
        for equation in &equations {
            let n = 1 << (equation.nums.len() - 1);
            let matches = (0..n).any(|k| {
                let mut test = equation.nums.first().copied().unwrap();
                for (index, value) in equation.nums.iter().skip(1).copied().enumerate() {
                    if ((k >> index) & 1) == 1 {
                        test += value;
                    } else {
                        test *= value;
                    }
                }

                test == equation.test_value
            });
            if matches {
                sum += equation.test_value;
            }
        }
        sum.to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let equations = parse_input(input);
        let mut sum = 0;

        fn concat(a: u64, b: u64) -> u64 {
            let mut digits = 0;
            let mut temp_b = b;
            while temp_b != 0 {
                temp_b /= 10;
                digits += 1;
            }
            (a * 10u64.pow(digits)) + b
        }

        for equation in &equations {
            let mut ternary_counter = TernaryCounter::new(equation.nums.len() - 1);
            loop {
                let mut test = equation.nums.first().copied().unwrap();
                for (index, value) in equation.nums.iter().skip(1).copied().enumerate() {
                    let op_type = ternary_counter.values()[index];
                    if op_type == 0 {
                        test += value;
                    } else if op_type == 1 {
                        test *= value;
                    } else if op_type == 2 {
                        test = concat(test, value);
                    } else {
                        unreachable!();
                    }
                }

                if test == equation.test_value {
                    sum += equation.test_value;
                    break;
                }

                if !ternary_counter.inc() {
                    break;
                }
            }
        }
        sum.to_string()
    }
}

struct TernaryCounter {
    vec: Box<[u8]>,
}

impl TernaryCounter {
    fn new(n: usize) -> Self {
        TernaryCounter {
            vec: vec![0; n].into_boxed_slice(),
        }
    }

    fn inc(&mut self) -> bool {
        let mut next = (self.vec.len() - 1) as isize;
        while next >= 0 {
            let current = &mut self.vec[next as usize];
            *current += 1;
            if *current == 3 {
                *current = 0;
                next -= 1;
            } else {
                break;
            }
        }
        next != -1
    }

    fn values(&self) -> &[u8] {
        &self.vec
    }
}

#[derive(Debug)]
struct Equation {
    test_value: u64,
    nums: Vec<u64>,
}

fn parse_input(input: &AocInput) -> Vec<Equation> {
    input
        .lines
        .iter()
        .map(|e| {
            let mut it = e.split(":");
            let test_value = it.next().unwrap().parse().unwrap();
            let nums = it
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            Equation { test_value, nums }
        })
        .collect()
}
