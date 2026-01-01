//! Advent of Code 2024 Day 9
//!
//! Link: <https://adventofcode.com/2024/day/9>

use anyhow::Result;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::repeat_n;

pub fn main(input: &str) -> Result<(usize, usize)> {
    let data = parse_input(input)?;
    Ok((part1(&data), part2()))
}

fn parse_input(input: &str) -> Result<Vec<FileBlock>> {
    let mut blocks: Vec<FileBlock> = Vec::new();
    let mut block_idx = 0_usize;
    let mut file_id = 0_isize;

    for (i, c) in input.chars().enumerate() {
        let block_size = c.to_digit(10).unwrap() as usize;
        let end_idx = block_idx + block_size - 1;
        if block_size == 0 {
            continue;
        }
        if i % 2 == 0 {
            blocks.push(FileBlock {
                start: block_idx,
                end: end_idx,
                file_id,
            });
            file_id += 1;
        } else {
            blocks.push(FileBlock {
                start: block_idx,
                end: end_idx,
                file_id: -1,
            });
        }
        block_idx = end_idx + 1;
    }
    blocks.sort();
    Ok(blocks)
}

fn part1(input: &[FileBlock]) -> usize {
    let mut file_blocks = BinaryHeap::new();
    let mut free_blocks = BinaryHeap::new();

    for block in input {
        if block.file_id == -1 {
            free_blocks.push(Reverse(block.clone()));
        } else {
            file_blocks.push(block.clone());
        }
    }

    let mut curr_file = file_blocks.pop().unwrap();
    while let Some(Reverse(empty_block)) = free_blocks.pop() {
        let free_space = empty_block.size();
        let file_size = curr_file.size();

        if curr_file.start < empty_block.start {
            break;
        }

        match free_space.cmp(&file_size) {
            std::cmp::Ordering::Less => {
                let new_file = FileBlock {
                    start: empty_block.start,
                    end: empty_block.end,
                    file_id: curr_file.file_id,
                };
                let mut updated_file = curr_file.clone();
                updated_file.end -= free_space;
                curr_file = updated_file;
                file_blocks.push(new_file);
            }
            std::cmp::Ordering::Equal => {
                // File and empty space are the same size
                let new_file = FileBlock {
                    start: empty_block.start,
                    end: empty_block.end,
                    file_id: curr_file.file_id,
                };
                file_blocks.push(new_file);
                curr_file = file_blocks.pop().unwrap();
            }
            std::cmp::Ordering::Greater => {
                // Empty space is greater than file size
                let new_file = FileBlock {
                    start: empty_block.start,
                    end: empty_block.start + file_size - 1,
                    file_id: curr_file.file_id,
                };
                let remaining_empty_block = FileBlock {
                    start: new_file.end + 1,
                    end: empty_block.end,
                    file_id: -1,
                };
                free_blocks.push(Reverse(remaining_empty_block));
                file_blocks.push(new_file);
                curr_file = file_blocks.pop().unwrap();
            }
        }
    }
    file_blocks.push(curr_file);
    file_blocks.iter().map(|f| f.check_sum()).sum()
}

fn part2() -> usize {
    0
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct FileBlock {
    start: usize,
    end: usize,
    file_id: isize,
}

impl FileBlock {
    fn new(start: usize, end: usize, file_id: isize) -> Self {
        Self {
            start,
            end,
            file_id,
        }
    }
    fn size(&self) -> usize {
        self.end.abs_diff(self.start) + 1
    }

    fn check_sum(&self) -> usize {
        let (s, e) = (self.start as isize, self.end as isize);
        let sum_to_end = (e + 1) * e / 2;
        let sum_to_start = if s < 1 { 0 } else { (s) * (s - 1) / 2 };
        let check_sum = self.file_id * (sum_to_end - sum_to_start);
        check_sum as usize
    }
}

impl PartialOrd for FileBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FileBlock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_blocks() {
        let input = parse_input("12345").unwrap();
        assert_eq!(
            input,
            vec![
                FileBlock::new(0, 0, 0),
                FileBlock::new(1, 2, -1),
                FileBlock::new(3, 5, 1),
                FileBlock::new(6, 9, -1),
                FileBlock::new(10, 14, 2),
            ]
        )
    }

    #[test]
    fn test_parse_input_2() {
        let input = parse_input("2333133121414131402").unwrap();
        let expected_blocks = vec![
            FileBlock::new(0, 1, 0),
            FileBlock::new(2, 4, -1),
            FileBlock::new(5, 7, 1),
            FileBlock::new(8, 10, -1),
            FileBlock::new(11, 11, 2),
            FileBlock::new(12, 14, -1),
            FileBlock::new(15, 17, 3),
            FileBlock::new(18, 18, -1),
            FileBlock::new(19, 20, 4),
            FileBlock::new(21, 21, -1),
            FileBlock::new(22, 25, 5),
            FileBlock::new(26, 26, -1),
            FileBlock::new(27, 30, 6),
            FileBlock::new(31, 31, -1),
            FileBlock::new(32, 34, 7),
            FileBlock::new(35, 35, -1),
            FileBlock::new(36, 39, 8),
            FileBlock::new(40, 41, 9),
        ];
        assert_eq!(input, expected_blocks);
    }

    #[test]
    fn test_get_file_size() {
        let block = FileBlock::new(0, 1, 0);
        let block2 = FileBlock::new(32, 34, 7);
        assert_eq!(block.size(), 2);
        assert_eq!(block2.size(), 3);
    }

    #[test]
    fn test_part1() {
        let input = parse_input("2333133121414131402").unwrap();

        assert_eq!(part1(&input), 1928);
    }
}
