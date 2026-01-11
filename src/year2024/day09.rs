//! Advent of Code 2024 Day 9
//!
//! Part 1: Two-pointer compaction
//! Part 2: Interval Tree leftmost-fit defragmentation

use crate::utils::interval_tree::{Interval, IntervalTree};
use anyhow::{Context, Result};

pub fn main(input: &str) -> Result<(usize, usize)> {
    let data = parse_input(input.trim())?;
    Ok((part1(input.trim()), part2(&data)))
}

pub fn parse_input(input: &str) -> Result<Vec<FileBlock>> {
    let mut blocks = Vec::new();
    let mut curr_pos = 0;
    let mut file_id = 0;

    for (i, c) in input.chars().enumerate() {
        let size = c.to_digit(10).context("Invalid digit")? as usize;
        if size > 0 {
            let end = curr_pos + size - 1;
            blocks.push(FileBlock {
                start: curr_pos,
                end,
                file_id: if i % 2 == 0 { file_id } else { -1 },
            });
        }
        if i % 2 == 0 {
            file_id += 1;
        }
        curr_pos += size;
    }
    Ok(blocks)
}

/// Part 1: Standard two-pointer approach on the expanded disk map
pub fn part1(input: &str) -> usize {
    let mut disk: Vec<isize> = Vec::new();
    let mut file_id = 0;

    for (i, c) in input.chars().enumerate() {
        let size = c.to_digit(10).unwrap() as usize;
        let val = if i % 2 == 0 {
            let id = file_id;
            file_id += 1;
            id
        } else {
            -1
        };
        for _ in 0..size {
            disk.push(val);
        }
    }

    let mut left = 0;
    let mut right = disk.len() - 1;

    while left < right {
        if disk[left] != -1 {
            left += 1;
        } else if disk[right] == -1 {
            right -= 1;
        } else {
            disk[left] = disk[right];
            disk[right] = -1;
            left += 1;
            right -= 1;
        }
    }

    // Fix: Using |&id| instead of |(&id)| or explicit deref
    disk.iter()
        .enumerate()
        .filter(|(_, id)| **id != -1)
        .map(|(pos, id)| pos * (*id as usize))
        .sum()
}

/// Part 2: Interval Tree to find leftmost available space for whole files
pub fn part2(input: &[FileBlock]) -> usize {
    let mut tree = IntervalTree::<usize>::new();
    let mut files = Vec::new();

    for block in input {
        if block.file_id == -1 {
            tree.insert(block.start, block.end);
        } else {
            files.push(*block);
        }
    }

    let mut final_checksum = 0;

    // Process files from right to left (highest ID first)
    for mut file in files.into_iter().rev() {
        // Safety check: if file.start is 0, there are no gaps before it
        if file.start > 0 {
            // Find all potential gaps to the left of the current file
            let mut candidates = tree.find_all_overlapping(Interval::new(0, file.start - 1));

            // Sort to ensure we pick the LEFTMOST fitting gap
            candidates.sort_by_key(|c| c.low);

            if let Some(target) = candidates
                .into_iter()
                .find(|c| (c.high - c.low + 1) >= file.size())
            {
                // Remove the old gap
                tree.delete(target.low, target.high);

                let f_size = file.size();
                file.start = target.low;
                file.end = target.low + f_size - 1;

                // Re-insert remaining free space if the gap was larger than the file
                if (target.high - target.low + 1) > f_size {
                    tree.insert(target.low + f_size, target.high);
                }
            }
        }

        final_checksum += file.check_sum();
    }

    final_checksum
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct FileBlock {
    pub start: usize,
    pub end: usize,
    pub file_id: isize,
}

impl FileBlock {
    pub fn new(start: usize, end: usize, file_id: isize) -> Self {
        Self {
            start,
            end,
            file_id,
        }
    }

    pub fn size(&self) -> usize {
        self.end - self.start + 1
    }

    /// O(1) Checksum using arithmetic series: ID * (n/2 * (start + end))
    pub fn check_sum(&self) -> usize {
        if self.file_id == -1 {
            return 0;
        }
        let n = (self.end - self.start) + 1;
        let sum_of_positions = n * (self.start + self.end) / 2;
        sum_of_positions * (self.file_id as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1928);
    }

    #[test]
    fn test_part2() {
        let data = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&data), 2858);
    }

    #[test]
    fn test_checksum_o1() {
        let file = FileBlock::new(4, 6, 2); // Positions 4, 5, 6 with ID 2
        // (4*2) + (5*2) + (6*2) = 8 + 10 + 12 = 30
        assert_eq!(file.check_sum(), 30);
    }

    #[test]
    fn test_leftmost_fit() {
        // File 2 (size 2) at end, gaps of size 1 and 3 at the start
        let input = vec![
            FileBlock::new(0, 0, -1), // gap size 1
            FileBlock::new(1, 1, 0),
            FileBlock::new(2, 4, -1), // gap size 3
            FileBlock::new(5, 6, 2),  // file size 2
        ];
        // It should skip the first gap (too small) and take the start of the second gap
        let result = part2(&input);
        // File 0: (1*0) = 0
        // File 2: moved to pos 2,3: (2*2) + (3*2) = 10
        assert_eq!(result, 10);
    }
}
