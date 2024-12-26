use aoc_2024::{AocInput, AocSolver, Vec2};
use std::collections::HashSet;

pub(crate) struct Day15;

impl AocSolver for Day15 {
    fn day(&self) -> u32 {
        15
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let (mut grid, moves) = parse_input(input);
        for m in moves {
            grid.move_robot(m);
        }

        let mut sum = 0;
        for (y, row) in grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 'O' {
                    sum += y * 100 + x;
                }
            }
        }

        sum.to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let (grid, moves) = parse_input(input);
        let mut grid = scaled_grid(grid);
        for m in moves {
            grid.move_robot(m);
        }

        let mut sum = 0;
        for (y, row) in grid.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == '[' {
                    sum += y * 100 + x;
                }
            }
        }

        sum.to_string()
    }
}

#[derive(Debug)]
struct Grid {
    cells: Box<[Box<[char]>]>,
    robot: Vec2<usize>,
}

#[derive(Debug)]
struct ScaledGrid {
    cells: Box<[Box<[char]>]>,
    robot: Vec2<usize>,
}

impl Grid {
    fn move_robot(&mut self, dir: MoveDirection) {
        match dir {
            MoveDirection::Right => {
                let mut next_x = self.robot.x + 1;
                while self.cells[self.robot.y][next_x] != '#' {
                    if self.cells[self.robot.y][next_x] == '.' {
                        while next_x > self.robot.x {
                            self.cells[self.robot.y][next_x] = self.cells[self.robot.y][next_x - 1];
                            next_x -= 1;
                        }
                        self.robot.x += 1;
                        break;
                    }
                    next_x += 1;
                }
            }
            MoveDirection::Left => {
                let mut next_x = self.robot.x - 1;
                while self.cells[self.robot.y][next_x] != '#' {
                    if self.cells[self.robot.y][next_x] == '.' {
                        while next_x < self.robot.x {
                            self.cells[self.robot.y][next_x] = self.cells[self.robot.y][next_x + 1];
                            next_x += 1;
                        }
                        self.robot.x -= 1;
                        break;
                    }
                    next_x -= 1;
                }
            }
            MoveDirection::Up => {
                let mut next_y = self.robot.y - 1;
                while self.cells[next_y][self.robot.x] != '#' {
                    if self.cells[next_y][self.robot.x] == '.' {
                        while next_y < self.robot.y {
                            self.cells[next_y][self.robot.x] = self.cells[next_y + 1][self.robot.x];
                            next_y += 1;
                        }
                        self.robot.y -= 1;
                        break;
                    }
                    next_y -= 1;
                }
            }
            MoveDirection::Down => {
                let mut next_y = self.robot.y + 1;
                while self.cells[next_y][self.robot.x] != '#' {
                    if self.cells[next_y][self.robot.x] == '.' {
                        while next_y > self.robot.y {
                            self.cells[next_y][self.robot.x] = self.cells[next_y - 1][self.robot.x];
                            next_y -= 1;
                        }
                        self.robot.y += 1;
                        break;
                    }
                    next_y += 1;
                }
            }
        }
    }
}

impl ScaledGrid {
    fn move_robot(&mut self, dir: MoveDirection) {
        match dir {
            MoveDirection::Right => {
                let mut next_x = self.robot.x + 1;
                while self.cells[self.robot.y][next_x] != '#' {
                    if self.cells[self.robot.y][next_x] == '.' {
                        while next_x > self.robot.x {
                            self.cells[self.robot.y][next_x] = self.cells[self.robot.y][next_x - 1];
                            next_x -= 1;
                        }
                        self.robot.x += 1;
                        break;
                    }
                    next_x += 1;
                }
            }
            MoveDirection::Left => {
                let mut next_x = self.robot.x - 1;
                while self.cells[self.robot.y][next_x] != '#' {
                    if self.cells[self.robot.y][next_x] == '.' {
                        while next_x < self.robot.x {
                            self.cells[self.robot.y][next_x] = self.cells[self.robot.y][next_x + 1];
                            next_x += 1;
                        }
                        self.robot.x -= 1;
                        break;
                    }
                    next_x -= 1;
                }
            }
            MoveDirection::Up => {
                let next_y = self.robot.y - 1;
                if self.cells[next_y][self.robot.x] != '#' {
                    if self.cells[next_y][self.robot.x] == '.' {
                        self.robot.y = next_y;
                    } else {
                        assert!(
                            self.cells[next_y][self.robot.x] == '['
                                || self.cells[next_y][self.robot.x] == ']'
                        );
                        let mut boxes = vec![];
                        if self.cells[next_y][self.robot.x] == '[' {
                            let mut next_boxes = HashSet::new();
                            next_boxes.insert((self.robot.x, next_y, '['));
                            next_boxes.insert((self.robot.x + 1, next_y, ']'));
                            boxes.push(next_boxes);
                        } else {
                            let mut next_boxes = HashSet::new();
                            next_boxes.insert((self.robot.x - 1, next_y, '['));
                            next_boxes.insert((self.robot.x, next_y, ']'));
                            boxes.push(next_boxes);
                        }
                        loop {
                            let mut next_boxes = HashSet::new();
                            let last = boxes.last().unwrap();
                            last.iter().for_each(|&(x, y, _)| {
                                let ny = y - 1;
                                if self.cells[ny][x] == ']' {
                                    next_boxes.insert((x - 1, ny, self.cells[ny][x - 1]));
                                    next_boxes.insert((x, ny, self.cells[ny][x]));
                                } else if self.cells[ny][x] == '[' {
                                    next_boxes.insert((x, ny, self.cells[ny][x]));
                                    next_boxes.insert((x + 1, ny, self.cells[ny][x + 1]));
                                }
                            });

                            if boxes
                                .last()
                                .unwrap()
                                .iter()
                                .any(|&(x, y, _)| self.cells[y - 1][x] == '#')
                            {
                                break;
                            }

                            if boxes
                                .last()
                                .unwrap()
                                .iter()
                                .all(|&(x, y, _)| self.cells[y - 1][x] == '.')
                            {
                                // move
                                for v in &boxes {
                                    for &(x, y, _) in v {
                                        self.cells[y][x] = '.';
                                    }
                                }

                                for v in boxes {
                                    for (x, y, c) in v {
                                        self.cells[y - 1][x] = c;
                                    }
                                }

                                self.robot.y -= 1;
                                break;
                            }

                            boxes.push(next_boxes);
                        }
                    }
                }
            }
            MoveDirection::Down => {
                let next_y = self.robot.y + 1;
                if self.cells[next_y][self.robot.x] != '#' {
                    if self.cells[next_y][self.robot.x] == '.' {
                        self.robot.y = next_y;
                    } else {
                        assert!(
                            self.cells[next_y][self.robot.x] == '['
                                || self.cells[next_y][self.robot.x] == ']'
                        );

                        let mut boxes = vec![];
                        if self.cells[next_y][self.robot.x] == '[' {
                            let mut next_boxes = HashSet::new();
                            next_boxes.insert((self.robot.x, next_y, '['));
                            next_boxes.insert((self.robot.x + 1, next_y, ']'));
                            boxes.push(next_boxes);
                        } else {
                            let mut next_boxes = HashSet::new();
                            next_boxes.insert((self.robot.x - 1, next_y, '['));
                            next_boxes.insert((self.robot.x, next_y, ']'));
                            boxes.push(next_boxes);
                        }
                        loop {
                            let mut next_boxes = HashSet::new();
                            let last = boxes.last().unwrap();
                            last.iter().for_each(|&(x, y, _)| {
                                let ny = y + 1;
                                if self.cells[ny][x] == ']' {
                                    next_boxes.insert((x - 1, ny, self.cells[ny][x - 1]));
                                    next_boxes.insert((x, ny, self.cells[ny][x]));
                                } else if self.cells[ny][x] == '[' {
                                    next_boxes.insert((x, ny, self.cells[ny][x]));
                                    next_boxes.insert((x + 1, ny, self.cells[ny][x + 1]));
                                }
                            });

                            if boxes
                                .last()
                                .unwrap()
                                .iter()
                                .any(|&(x, y, _)| self.cells[y + 1][x] == '#')
                            {
                                break;
                            }

                            if boxes
                                .last()
                                .unwrap()
                                .iter()
                                .all(|&(x, y, _)| self.cells[y + 1][x] == '.')
                            {
                                // move
                                for v in &boxes {
                                    for &(x, y, _) in v {
                                        self.cells[y][x] = '.';
                                    }
                                }

                                for v in boxes {
                                    for (x, y, c) in v {
                                        self.cells[y + 1][x] = c;
                                    }
                                }

                                self.robot.y += 1;
                                break;
                            }

                            boxes.push(next_boxes);
                        }
                    }
                }
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                if self.robot.x == x && self.robot.y == y {
                    print!("@");
                } else {
                    print!("{}", self.cells[y][x]);
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &AocInput) -> (Grid, Vec<MoveDirection>) {
    let mut cells: Box<[Box<[char]>]> = input
        .lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let mut robot = Vec2::new(0, 0);
    for y in 0..cells.len() {
        for x in 0..cells[y].len() {
            if cells[y][x] == '@' {
                robot.x = x;
                robot.y = y;
                break;
            }
        }
    }
    cells[robot.y][robot.x] = '.';

    let moves = input
        .lines
        .iter()
        .skip(cells.len() + 1)
        .map(|e| {
            e.chars()
                .map(|e| match e {
                    '<' => MoveDirection::Left,
                    '>' => MoveDirection::Right,
                    '^' => MoveDirection::Up,
                    'v' => MoveDirection::Down,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    (Grid { cells, robot }, moves)
}

fn scaled_grid(mut grid: Grid) -> ScaledGrid {
    let mut cells = vec![vec![' '; grid.cells[0].len() * 2].into_boxed_slice(); grid.cells.len()]
        .into_boxed_slice();
    grid.cells[grid.robot.y][grid.robot.x] = '@';
    for y in 0..grid.cells.len() {
        for x in 0..grid.cells[y].len() {
            match grid.cells[y][x] {
                '#' => {
                    cells[y][x * 2] = '#';
                    cells[y][x * 2 + 1] = '#';
                }
                '@' => {
                    cells[y][x * 2] = '@';
                    cells[y][x * 2 + 1] = '.';
                }
                '.' => {
                    cells[y][x * 2] = '.';
                    cells[y][x * 2 + 1] = '.';
                }
                'O' => {
                    cells[y][x * 2] = '[';
                    cells[y][x * 2 + 1] = ']';
                }
                _ => unreachable!(),
            }
        }
    }

    let mut robot = Vec2::new(0, 0);
    for y in 0..cells.len() {
        for x in 0..cells[y].len() {
            if cells[y][x] == '@' {
                robot.x = x;
                robot.y = y;
                break;
            }
        }
    }
    cells[robot.y][robot.x] = '.';

    ScaledGrid { cells, robot }
}
