use aoc_2024::{AocInput, AocSolver};
use std::collections::{BTreeMap, HashSet};

pub(crate) struct Day9;

impl AocSolver for Day9 {
    fn day(&self) -> u32 {
        9
    }

    fn solve_part1(&mut self, input: &AocInput) -> String {
        let disk_map = parse_input(&input);
        let mut blocks = convert_to_blocks(&disk_map);

        let mut left = blocks.iter().position(|e| *e == -1).unwrap();
        let mut right = blocks.len() - 1;
        while left < right {
            if blocks[right] != -1 {
                blocks[left] = blocks[right];
                blocks[right] = -1;
                left += blocks[left + 1..].iter().position(|e| *e == -1).unwrap() + 1;
            }
            right -= 1;
        }

        check_sum(&blocks).to_string()
    }

    fn solve_part2(&mut self, input: &AocInput) -> String {
        let disk_map = parse_input(&input);
        let mut blocks = convert_to_blocks(&disk_map);

        let mut free_space = BTreeMap::new();
        {
            let mut i = 0;
            while i < blocks.len() {
                if blocks[i] == -1 {
                    let index = i;
                    let mut count = 1usize;
                    while i < blocks.len() - 1 && blocks[i + 1] == -1 {
                        i += 1;
                        count += 1;
                    }
                    free_space.insert(index, count);
                }
                i += 1;
            }
        }

        let mut block_counts = BTreeMap::new();
        {
            let mut i = 0;
            while i < blocks.len() {
                let id = blocks[i];
                if id != -1 {
                    let index = i;
                    let mut count = 1usize;
                    while i < blocks.len() - 1 && blocks[i + 1] == id {
                        i += 1;
                        count += 1;
                    }
                    block_counts.insert(id, (index, count));
                }
                i += 1;
            }
        }

        let mut already_moved = HashSet::new();
        block_counts.iter().rev().for_each(|(id, (index, count))| {
            if !already_moved.contains(id) {
                let available_space = free_space.iter().find(|(_, v)| **v >= *count);
                if let Some((&space_index, &space_count)) = available_space {
                    if space_count >= *count && space_index < *index {
                        for i in 0..*count {
                            blocks[space_index + i] = *id;
                            blocks[*index + i] = -1;
                        }
                        free_space.remove(&space_index);
                        free_space.insert(
                            space_index + *count,
                            std::cmp::max(space_count as isize - *count as isize, 0) as usize,
                        );
                        already_moved.insert(*id);
                    }
                }
            }
        });

        check_sum(&blocks).to_string()
    }
}

fn parse_input(input: &AocInput) -> Vec<u32> {
    input
        .lines
        .first()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn convert_to_blocks(disk_map: &[u32]) -> Vec<i64> {
    let mut blocks = vec![];
    let mut id = 0i64;
    for (i, v) in disk_map.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*v {
                blocks.push(id);
            }
            id += 1;
        } else {
            for _ in 0..*v {
                blocks.push(-1);
            }
        }
    }
    blocks
}

fn check_sum(blocks: &[i64]) -> usize {
    blocks
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| if v != -1 { v as usize * i } else { 0 })
        .sum::<usize>()
}
