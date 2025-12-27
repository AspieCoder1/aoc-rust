use anyhow::Result;
use std::collections::HashSet;

pub fn main(data: &str) -> Result<(u32, u64)> {
    let input = parse_input(data)?;
    let part1 = part1(&input);
    let part2 = part2(&input);

    Ok((part1, part2))
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<char>>> {
    Ok(input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect())
}

/// For this implementation we can perform a simple DFS of the manifold.
pub fn part1(input: &[Vec<char>]) -> u32 {
    let start = input[0].iter().position(|&c| c == 'S').unwrap();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: Vec<(usize, usize)> = vec![(0, start)];
    let mut acc = 0;

    while let Some((y, x)) = stack.pop() {
        if visited.contains(&(y, x)) {
            continue;
        }

        visited.insert((y, x));
        match input[y][x] {
            '.' | 'S' => {
                if y < input.len() - 1 {
                    stack.push((y + 1, x));
                }
            }
            '^' => {
                stack.extend(vec![(y, x + 1), (y, x - 1)]);
                acc += 1;
            }
            _ => unreachable!("Invalid char: {:?}", input[y][x]),
        }
    }
    acc
}

/// Part 2 can be performed using dynamic programming.
pub fn part2(input: &[Vec<char>]) -> u64 {
    let mut dp: Vec<Vec<u64>> = vec![vec![0; input[0].len()]; input.len()];
    let start = input[0].iter().position(|&c| c == 'S').unwrap();
    dp[0][start] = 1;
    for (y, row) in input.iter().enumerate().skip(1) {
        for x in 0..row.len() {
            if input[y - 1][x] != '^' {
                dp[y][x] = dp[y - 1][x];
            }
            if x > 0 && input[y][x - 1] == '^' {
                dp[y][x] += dp[y - 1][x - 1];
            }
            if x < row.len() - 1 && input[y][x + 1] == '^' {
                dp[y][x] += dp[y - 1][x + 1];
            }
        }
    }
    dp.last().unwrap().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 21);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 40);
    }
}
