use aoc_2024::{AocInput, AocSolver};
use std::collections::{BTreeMap, HashSet, VecDeque};

pub(crate) struct Day12;

impl AocSolver for Day12 {
    fn day(&self) -> u32 {
        12
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let grid = parse_input(input);
        let mut found = HashSet::new();
        let mut sum = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if !found.contains(&(x, y)) {
                    let (region, perimeter) = find_region(x, y, &grid);
                    sum += region.len() * perimeter;
                    found.extend(region);
                }
            }
        }

        sum.to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        #[derive(Hash, Eq, PartialEq, Debug)]
        struct RegionInfo {
            c: char,
            area: usize,
            sides: usize,
        }

        let grid = parse_input(input);
        let mut regions = vec![];
        let mut start_points = vec![];
        let mut region_info = BTreeMap::new();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if regions
                    .iter()
                    .all(|s: &HashSet<(usize, usize)>| !s.contains(&(x, y)))
                {
                    start_points.push((x, y));
                    let (region, _) = find_region(x, y, &grid);
                    regions.push(region);
                }
            }
        }

        let mut other_sides = vec![];
        start_points
            .iter()
            .copied()
            .enumerate()
            .for_each(|(i, (x, y))| {
                let region = &regions[i];
                let (sides, another_region) = count_sides(x, y, &grid, region);

                let c = grid[y][x];

                assert!(!region_info.contains_key(&i));
                region_info.insert(
                    i,
                    RegionInfo {
                        c,
                        area: region.len(),
                        sides,
                    },
                );

                if let Some((rx, ry)) = another_region {
                    other_sides.push(((rx, ry), sides));
                }
            });

        other_sides.into_iter().for_each(|((rx, ry), sides)| {
            let pos = regions.iter().position(|s| s.contains(&(rx, ry))).unwrap();
            region_info.get_mut(&pos).unwrap().sides += sides;
        });

        region_info
            .values()
            .map(|e| e.area * e.sides)
            .sum::<usize>()
            .to_string()
    }
}

fn parse_input(input: &AocInput) -> Box<[Box<[char]>]> {
    input
        .lines
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_region(x: usize, y: usize, grid: &[Box<[char]>]) -> (HashSet<(usize, usize)>, usize) {
    let mut result = HashSet::new();
    result.insert((x, y));

    let region = grid[y][x];
    let mut perimeter = 0;
    let mut shared_edges = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((x as isize, y as isize));
    while let Some((x, y)) = queue.pop_front() {
        perimeter += 4;
        let neighbours = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
        neighbours
            .iter()
            .copied()
            .filter(|(nx, ny)| {
                *ny >= 0
                    && (*ny as usize) < grid.len()
                    && *nx >= 0
                    && (*nx as usize) < grid[*ny as usize].len()
            })
            .for_each(|(nx, ny)| {
                let nx = nx as usize;
                let ny = ny as usize;
                let x = x as usize;
                let y = y as usize;
                if grid[ny][nx] == region {
                    if !result.contains(&(nx, ny)) {
                        queue.push_back((nx as isize, ny as isize));
                        result.insert((nx, ny));
                    }
                    if !shared_edges.contains(&((nx, ny), (x, y))) {
                        shared_edges.insert(((x, y), (nx, ny)));
                    }
                }
            });
    }

    perimeter -= shared_edges.len() * 2;
    (result, perimeter)
}

fn count_sides(
    x: usize,
    y: usize,
    grid: &[Box<[char]>],
    region: &HashSet<(usize, usize)>,
) -> (usize, Option<(usize, usize)>) {
    let region_char = grid[y][x];
    let mut count = 0;

    let mut robot = Robot::new(x, y, Dir::Right, region_char);
    let start_x = robot.x;
    let start_y = robot.y;

    let mut chars = HashSet::new();
    if start_y >= 0
        && (start_y as usize) < grid.len()
        && start_x >= 0
        && (start_x as usize) < grid[start_y as usize].len()
    {
        chars.insert(Some(grid[start_y as usize][start_x as usize]));
    } else {
        chars.insert(None);
    }

    loop {
        if robot.step(grid, region) {
            count += 1;
        }

        if robot.y >= 0
            && (robot.y as usize) < grid.len()
            && robot.x >= 0
            && (robot.x as usize) < grid[robot.y as usize].len()
        {
            chars.insert(Some(grid[robot.y as usize][robot.x as usize]));
        } else {
            chars.insert(None);
        }

        if robot.x == start_x && robot.y == start_y {
            break;
        }
    }

    if chars.len() == 1 && chars.iter().next().unwrap().is_some() {
        return (count, Some((start_x as usize, start_y as usize)));
    }

    (count, None)
}

#[derive(Debug)]
struct Robot {
    x: isize,
    y: isize,
    region: char,
    dir: Dir,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

impl Robot {
    fn new(x: usize, y: usize, dir: Dir, region: char) -> Self {
        Self {
            x: x as isize,
            y: y as isize - 1,
            dir,
            region,
        }
    }

    fn grid_pos(&self) -> (isize, isize) {
        match self.dir {
            Dir::Left => (self.x, self.y - 1),
            Dir::Right => (self.x, self.y + 1),
            Dir::Up => (self.x + 1, self.y),
            Dir::Down => (self.x - 1, self.y),
        }
    }

    /// Returns true if direction was changed.
    fn step(&mut self, grid: &[Box<[char]>], region: &HashSet<(usize, usize)>) -> bool {
        let is_valid = |x: isize, y: isize| -> bool {
            y >= 0 && (y as usize) < grid.len() && x >= 0 && (x as usize) < grid[y as usize].len()
        };

        let prev_dir = self.dir;
        let prev_x = self.x;
        let prev_y = self.y;
        for (x, y, d) in self.next_positions() {
            self.x = x;
            self.y = y;
            self.dir = d;

            let (gx, gy) = self.grid_pos();
            if is_valid(gx, gy)
                && grid[gy as usize][gx as usize] == self.region
                && region.contains(&(gx as usize, gy as usize))
            {
                if !is_valid(x, y) || grid[y as usize][x as usize] != self.region {
                    return self.dir != prev_dir;
                }
            }

            self.x = prev_x;
            self.y = prev_y;
            self.dir = prev_dir;
        }

        unreachable!()
    }

    fn next_positions(&self) -> [(isize, isize, Dir); 3] {
        match self.dir {
            Dir::Left => [
                (self.x - 1, self.y, Dir::Left),
                (self.x, self.y, Dir::Down),
                (self.x - 1, self.y - 1, Dir::Up),
            ],
            Dir::Right => [
                (self.x + 1, self.y, Dir::Right),
                (self.x, self.y, Dir::Up),
                (self.x + 1, self.y + 1, Dir::Down),
            ],
            Dir::Up => [
                (self.x, self.y - 1, Dir::Up),
                (self.x, self.y, Dir::Left),
                (self.x + 1, self.y - 1, Dir::Right),
            ],
            Dir::Down => [
                (self.x, self.y + 1, Dir::Down),
                (self.x, self.y, Dir::Right),
                (self.x - 1, self.y + 1, Dir::Left),
            ],
        }
    }
}
