use aoc_2024::{AocInput, AocSolver, Vec2};

pub(crate) struct Day13;

impl AocSolver for Day13 {
    fn day(&self) -> u32 {
        13
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let machines = parse_input(&input);
        machines
            .iter()
            .map(|e| play(e.button_a, e.button_b, e.prize).unwrap_or(0))
            .sum::<i64>()
            .to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let mut machines = parse_input(&input);
        machines.iter_mut().for_each(|e| {
            e.prize.x += 10000000000000;
            e.prize.y += 10000000000000;
        });
        machines
            .iter()
            .map(|e| play(e.button_a, e.button_b, e.prize).unwrap_or(0))
            .sum::<i64>()
            .to_string()
    }
}

fn play(button_a: Vec2<i64>, button_b: Vec2<i64>, prize: Vec2<i64>) -> Option<i64> {
    let b = (button_a.x * prize.y - button_a.y * prize.x)
        / (button_a.x * button_b.y - button_a.y * button_b.x);
    let a = (prize.x - button_b.x * b) / button_a.x;

    if a * button_a.x + b * button_b.x == prize.x && a * button_a.y + b * button_b.y == prize.y {
        let result = a * 3 + b;
        return Some(result);
    }

    None
}

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: Vec2<i64>,
    button_b: Vec2<i64>,
    prize: Vec2<i64>,
}

fn parse_input(input: &AocInput) -> Box<[ClawMachine]> {
    fn parse_button(mut line: &str) -> Vec2<i64> {
        // x
        let index = line.find('+').unwrap();
        let comma = line.find(',').unwrap();
        let x = line[index + 1..comma].parse::<i64>().unwrap();

        // y
        line = &line[comma + 1..];
        let index = line.find('+').unwrap();
        let y = line[index + 1..].parse::<i64>().unwrap();

        Vec2::new(x, y)
    }

    fn parse_prize(mut line: &str) -> Vec2<i64> {
        // x
        let index = line.find('=').unwrap();
        let comma = line.find(',').unwrap();
        let x = line[index + 1..comma].parse::<i64>().unwrap();

        // y
        line = &line[comma + 1..];
        let index = line.find('=').unwrap();
        let y = line[index + 1..].parse::<i64>().unwrap();

        Vec2::new(x, y)
    }

    input
        .lines
        .chunk_by(|a, _| !a.is_empty())
        .map(|e| e.iter().filter(|e| !e.is_empty()))
        .map(|e| {
            let mut iterator = e.into_iter();
            ClawMachine {
                button_a: parse_button(iterator.next().unwrap()),
                button_b: parse_button(iterator.next().unwrap()),
                prize: parse_prize(iterator.next().unwrap()),
            }
        })
        .collect()
}
