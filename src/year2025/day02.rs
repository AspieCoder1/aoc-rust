use anyhow::Result;
use std::str::FromStr;

const INPUT_NUM: usize = 0;

pub fn main() -> Result<(u64, u64)> {
    let input = parse_input(INPUT_NUM)?;

    let (part1, part2) = solve(input);

    Ok((part1, part2))
}

pub fn solve(input: Vec<IdRange>) -> (u64, u64) {
    let part1 = part1(&input);
    let part2 = part2(&input);

    (part1, part2)
}

pub fn parse_input(input: usize) -> Result<Vec<IdRange>> {
    [
        include_str!("inputs/day02.inp"),
        include_str!("test_inputs/day02.inp1"),
    ][input]
        .split(",")
        .map(str::parse)
        .collect()
}

#[derive(Debug, Clone)]
pub struct IdRange {
    start: u64,
    end: u64,
}

impl FromStr for IdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-');
        let start = split.next().unwrap().parse()?;
        let end = split.next().unwrap().parse()?;

        Ok(Self { start, end })
    }
}

pub fn part1(inp: &[IdRange]) -> u64 {
    let mut part1: u64 = 0;

    for range in inp {
        let mut range_start = range.start;
        let mut range_end = range.end;

        // Get the number of digits
        let mut start_num_digits = range_start.ilog10() + 1;
        let mut end_num_digits = range_end.ilog10() + 1;

        // Handle cases when odd number of digits
        if !start_num_digits.is_multiple_of(2) {
            range_start = 10_u64.pow(start_num_digits);
            start_num_digits += 1;
        }
        if !end_num_digits.is_multiple_of(2) {
            end_num_digits -= 1;
            range_end = 10_u64.pow(end_num_digits) - 1;
        }

        // Get the prefixes we are looping between
        let mut start_prefix = range_start / (10_u64.pow(start_num_digits / 2));
        let mut end_prefix = range_end / (10_u64.pow(end_num_digits / 2));
        let start_suffix = range_start % (10_u64.pow(start_num_digits / 2));
        let end_suffix = range_end % (10_u64.pow(end_num_digits / 2));

        // Reducing range to set of suffixes which shall work
        if start_prefix < start_suffix {
            start_prefix = start_suffix
        }
        if end_prefix > end_suffix {
            end_prefix = end_suffix
        }

        if start_prefix > end_prefix {
            continue;
        }

        // As ids will have will the same magnitude can use a triangular number formula
        let sum_of_prefixes =
            (((end_prefix + 1) * end_prefix) - (start_prefix * (start_prefix - 1))) / 2;
        part1 += sum_of_prefixes + (sum_of_prefixes * 10_u64.pow(start_num_digits / 2))
    }
    part1
}

pub fn part2(inp: &[IdRange]) -> u64 {
    let mut part2: u64 = 0;

    for range in inp {
        // Loop through each id in the range
        for id in range.start..=range.end {
            let num_digits = id.ilog10() + 1;
            let mut is_valid = true;

            for pattern_len in 1..=num_digits / 2 {
                if num_digits.is_multiple_of(pattern_len) {
                    let pattern = id / (10_u64.pow(num_digits - pattern_len));

                    let mut id_to_test: u64 = 0;
                    for pow in (0..num_digits).step_by(pattern_len as usize) {
                        id_to_test += pattern * (10_u64.pow(pow));
                    }

                    if id_to_test == id {
                        is_valid = false;
                        break;
                    }
                }
            }
            if !is_valid {
                part2 += id;
            }
        }
    }

    part2
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        let input = parse_input(1).unwrap();
        assert_eq!(part1(&input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(1).unwrap();
        assert_eq!(part2(&input), 4174379265);
    }
}
