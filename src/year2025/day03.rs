use anyhow::Result;

const INPUT_NUM: usize = 0;

pub fn main() -> Result<(u64, u64)> {
    let input = parse_input(INPUT_NUM).unwrap();
    let (part1, part2) = solve(input);

    Ok((part1, part2))
}

pub fn solve(input: Vec<Vec<u32>>) -> (u64, u64) {
    let part1 = part1(&input);
    let part2 = part2(&input);

    (part1, part2)
}

type NestedU32 = Vec<Vec<u32>>;

pub fn parse_input(input: usize) -> Result<Vec<Vec<u32>>, String> {
    [
        include_str!("inputs/day03.inp"),
        include_str!("test_inputs/day03.inp1"),
    ][input]
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| format!("Parsing error: Invalid digit '{}' found.", c))
                })
                .collect::<Result<Vec<u32>, String>>()
        })
        .collect::<Result<NestedU32, String>>()
}

pub fn part1(input: &[Vec<u32>]) -> u64 {
    max_joltage::<2>(input)
}

pub fn part2(input: &[Vec<u32>]) -> u64 {
    max_joltage::<12>(input)
}

pub fn max_joltage<const DIGITS: usize>(input: &[Vec<u32>]) -> u64 {
    let mut answer = 0;
    for bank in input {
        let mut start = 0;
        let mut acc = 0;
        for remaining_digits in (0..DIGITS).rev() {
            let end = bank.len() - remaining_digits;

            let slice = &bank[start..end];
            let (mut max_idx, mut max_digit) = (0, 0);

            for (idx, &digit) in slice.iter().enumerate() {
                if digit > max_digit {
                    max_idx = idx;
                    max_digit = digit;
                }
            }

            acc = acc * 10 + u64::from(max_digit);
            start += max_idx + 1;
        }
        answer += acc;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        let input = parse_input(1).unwrap();
        let part1 = part1(&input);

        assert_eq!(part1, 357);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(1).unwrap();
        let part2 = part2(&input);

        assert_eq!(part2, 3121910778619);
    }
}
