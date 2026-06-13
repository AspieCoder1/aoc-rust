//! Advent of Code 2024 Day 17
//!
//! Link: <https://adventofcode.com/2024/day/17>

use anyhow::{Context, Result, anyhow};
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
}

impl FromStr for Computer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        // Helper to extract values after the colon ':'
        let mut parse_register = |line: Option<&str>, reg_name: &str| -> Result<u64> {
            let line = line.ok_or_else(|| anyhow!("Missing line for Register {}", reg_name))?;
            let (_, val_str) = line
                .split_once(':')
                .with_context(|| format!("Invalid format on line: '{}'", line))?;
            val_str
                .trim()
                .parse::<u64>()
                .with_context(|| format!("Failed to parse integer for Register {}", reg_name))
        };

        let a = parse_register(lines.next(), "A")?;
        let b = parse_register(lines.next(), "B")?;
        let c = parse_register(lines.next(), "C")?;

        // Skip potential blank separation line
        let mut next_line = lines.next();
        if let Some(line) = next_line {
            if line.trim().is_empty() {
                next_line = lines.next();
            }
        }

        // Get the program string block
        let program_line = next_line.ok_or_else(|| anyhow!("Missing Program line sequence"))?;
        let (_, program_str) = program_line
            .split_once(':')
            .with_context(|| format!("Invalid Program line format: '{}'", program_line))?;

        // Map every string segment directly to a raw u8 vector
        let program = program_str
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<u8>()
                    .with_context(|| format!("Failed to parse program byte: '{}'", s))
            })
            .collect::<Result<Vec<u8>>>()?;

        Ok(Self { a, b, c, program })
    }
}

impl Computer {
    /// Zero-allocation immutable simulator.
    /// By keeping it immutable (&self), we can run it multi-threaded or across
    /// intensive recursive loops without copying the base computer configuration state.
    fn run_sim(&self, mut a: u64, mut b: u64, mut c: u64) -> Vec<u8> {
        let mut ip: usize = 0;
        // Pre-allocate buffer capacity to eliminate hot-path reallocation penalties
        let mut output = Vec::with_capacity(16);

        // Safe bounds check: ensures we have a complete opcode/operand pair remaining
        while ip + 1 < self.program.len() {
            let opcode = self.program[ip];
            let operand = self.program[ip + 1];
            let mut next_ip = ip + 2;

            // Micro-optimization: Resolve combo operand rules inline using fast matching
            let combo = match operand {
                0..=3 => operand as u64,
                4 => a,
                5 => b,
                6 => c,
                _ => 0, // Reserved state
            };

            match opcode {
                0 => a = a.checked_shr(combo as u32).unwrap_or(0), // adv
                1 => b ^= operand as u64,                           // bxl
                2 => b = combo % 8,                                // bst
                3 => if a != 0 { next_ip = operand as usize; },    // jnz
                4 => b ^= c,                                       // bxc
                5 => output.push((combo % 8) as u8),               // out
                6 => b = a.checked_shr(combo as u32).unwrap_or(0), // bdv
                7 => c = a.checked_shr(combo as u32).unwrap_or(0), // cdv
                _ => unsafe { std::hint::unreachable_unchecked() } // Optimizes lookahead jumps
            }
            ip = next_ip;
        }
        output
    }
}

/// Part 2 Algorithmic Optimization: Backtracking Depth-First Search.
/// Because the system works like a 3-bit shift register, we walk backward through the
/// instructions array, guessing the 3 bits that generate the expected output byte.
fn solve_part2(computer: &Computer, current_a: u64, target_idx: usize) -> Option<u64> {
    // Base Case: If we have successfully matched the entire program length backwards, we are done
    if target_idx == computer.program.len() {
        return Some(current_a);
    }

    // Shift previously identified octal integer values up by 3 bits
    let shifted_a = current_a << 3;
    let expected_byte = computer.program[computer.program.len() - 1 - target_idx];

    // Evaluate all 8 possible octal configurations (0 through 7) for this tier
    for next_bits in 0..8 {
        let test_a = shifted_a | next_bits;

        let out = computer.run_sim(test_a, 0, 0);

        // If the first digit of output matches our expected sequence position, dive deeper
        if !out.is_empty() && out[0] == expected_byte {
            if let Some(final_a) = solve_part2(computer, test_a, target_idx + 1) {
                return Some(final_a); // Bubble up the correct initialization key
            }
        }
    }
    None
}

pub fn main(input_data: &str) -> Result<(String, u64)> {
    let computer = input_data.parse::<Computer>()?;

    // Part 1: Execute simulation directly from parsed parameters
    let part1_res = computer.run_sim(computer.a, computer.b, computer.c);
    let part1 = part1_res.iter().map(|num| num.to_string()).join(",");

    // Part 2: Solve via optimized backtracking depth-first search
    let part2 = solve_part2(&computer, 0, 0)
        .context("Could not find a valid matching initialization key for Part 2")?;

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_input_parsing() {
        let input = Computer::from_str(EXAMPLE).unwrap();
        let expected = Computer {
            a: 729,
            b: 0,
            c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
        };
        assert_eq!(input, expected);
    }

    #[test]
    fn test_part1() {
        let input = Computer::from_str(EXAMPLE).unwrap();
        let sim_result = input.run_sim(input.a, input.b, input.c);
        assert_eq!(sim_result, vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn test_part2() {
        let input = Computer::from_str(EXAMPLE2).unwrap();
        let res = solve_part2(&input, 0, 0).unwrap();

        assert_eq!(res, 117440);
    }
}