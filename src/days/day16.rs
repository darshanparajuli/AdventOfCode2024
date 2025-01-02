use aoc_2024::{AocInput, AocSolver, Vec2};
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

pub(crate) struct Day16;

impl AocSolver for Day16 {
    fn day(&self) -> u32 {
        16
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let maze = parse_input(input);
        let mut r = Reindeer::new();
        r.find_path(&maze).to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let maze = parse_input(input);
        let mut r = Reindeer::new();
        r.num_tiles(&maze).to_string()
    }
}

#[derive(Debug)]
struct Maze {
    cells: Box<[Box<[char]>]>,
    start: Vec2<usize>,
    end: Vec2<usize>,
}

impl Maze {
    #[allow(dead_code)]
    fn print(&self) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if Vec2::new(x, y) == self.start {
                    print!("S");
                } else if Vec2::new(x, y) == self.end {
                    print!("E");
                } else {
                    print!("{}", c);
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct Reindeer {
    dir: Dir,
}

impl Reindeer {
    fn new() -> Self {
        Reindeer { dir: Dir::East }
    }

    fn find_path(&mut self, maze: &Maze) -> usize {
        let (dist, _) = self.run_dijkstra(maze);
        dist.get(&maze.end).unwrap().0
    }

    fn num_tiles(&mut self, maze: &Maze) -> usize {
        let (dist, prev) = self.run_dijkstra(maze);

        fn dfs(
            pos: Vec2<usize>,
            dir: Dir,
            prev: &HashMap<(Vec2<usize>, Dir), HashSet<(Vec2<usize>, Dir, usize)>>,
            visited: &mut HashSet<(Vec2<usize>, Dir)>,
            shortest_path: &mut HashSet<Vec2<usize>>,
            maze: &Maze,
            cost: usize,
            best_cost: usize,
        ) {
            if cost > best_cost {
                return;
            }

            if pos == maze.start {
                if cost == best_cost {
                    shortest_path.extend(visited.iter().map(|e| e.0));
                }
                return;
            }

            visited.insert((pos, dir));

            if let Some(nodes) = prev.get(&(pos, dir)) {
                nodes.iter().for_each(|&(v, d, c)| {
                    if !visited.contains(&(v, d)) {
                        let cost = cost + 1 + if d == dir { 0 } else { 1000 };
                        if best_cost as isize - c as isize == cost as isize {
                            dfs(v, d, &prev, visited, shortest_path, maze, cost, best_cost);
                        }
                    }
                });
            }

            visited.remove(&(pos, dir));
        }

        let mut visited = HashSet::new();
        let mut shortest_paths = HashSet::new();
        dfs(
            maze.end,
            dist[&maze.end].1,
            &prev,
            &mut visited,
            &mut shortest_paths,
            maze,
            0,
            dist[&maze.end].0,
        );

        shortest_paths.len() + 1
    }

    fn run_dijkstra(
        &mut self,
        maze: &Maze,
    ) -> (
        HashMap<Vec2<usize>, (usize, Dir)>,
        HashMap<(Vec2<usize>, Dir), HashSet<(Vec2<usize>, Dir, usize)>>,
    ) {
        let mut dist = HashMap::new();
        dist.insert(maze.start, (0usize, Dir::East));
        let mut prev = HashMap::new();

        #[derive(Eq, PartialEq, Hash)]
        struct Vertex {
            pos: Vec2<usize>,
            dir: Dir,
            cost: usize,
        }

        impl Ord for Vertex {
            fn cmp(&self, other: &Self) -> Ordering {
                other.cost.cmp(&self.cost)
            }
        }
        impl PartialOrd for Vertex {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(other.cost.cmp(&self.cost))
            }
        }

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse(Vertex {
            pos: maze.start,
            dir: self.dir,
            cost: 0,
        }));

        while let Some(Reverse(Vertex { pos, dir, cost })) = min_heap.pop() {
            [
                (Vec2::new(pos.x + 1, pos.y), Dir::East),
                (Vec2::new(pos.x, pos.y + 1), Dir::South),
                (Vec2::new(pos.x - 1, pos.y), Dir::West),
                (Vec2::new(pos.x, pos.y - 1), Dir::North),
            ]
            .into_iter()
            .filter(|(v, _)| maze.cells[v.y][v.x] != '#')
            .for_each(|(p, d)| {
                let c = 1 + if d == dir { 0 } else { 1000 };
                let next_cost = cost + c;
                let curr_dist = if let Some(dist_value) = dist.get(&p) {
                    dist_value.0
                } else {
                    usize::MAX
                };
                if next_cost <= curr_dist {
                    dist.insert(p, (next_cost, d));
                    prev.entry((p, d))
                        .or_insert_with(|| HashSet::new())
                        .insert((pos, dir, cost));
                    min_heap.push(Reverse(Vertex {
                        pos: p,
                        dir: d,
                        cost: next_cost,
                    }));
                }
            });
        }

        (dist, prev)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Dir {
    East,
    West,
    North,
    South,
}

fn parse_input(input: &AocInput) -> Maze {
    let cells: Box<[Box<[char]>]> = input.lines.iter().map(|s| s.chars().collect()).collect();
    let mut start = Vec2::new(0, 0);
    let mut end = Vec2::new(0, 0);
    for y in 0..cells.len() {
        for x in 0..cells[y].len() {
            if cells[y][x] == 'S' {
                start.x = x;
                start.y = y;
            } else if cells[y][x] == 'E' {
                end.x = x;
                end.y = y;
            }
        }
    }
    Maze { cells, start, end }
}
