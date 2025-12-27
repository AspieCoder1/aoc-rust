use anyhow::Result;

pub fn main(data: &str) -> Result<(u64, u64)> {
    let input = parse_input(data)?;

    Ok((part1(&input), part2(&input)))
}

pub fn parse_input(input: &str) -> Result<Vec<Calculation>> {
    let _max_length = input.lines().map(|line| line.len()).max().unwrap();
    let mut data = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for row in &mut data {
        if row.len() < _max_length {
            row.extend(vec![' '; _max_length - row.len()]);
        }
    }

    let mut operator_idx = data
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .filter(|&(_, &op)| op == '+' || op == '*')
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();
    operator_idx.push(_max_length + 1);

    let mut calculations = Vec::new();
    for w in operator_idx.windows(2).map(|w| (w[0], w[1] - 1)) {
        let mut numbers = Vec::new();
        for row in &data {
            numbers.push(row[w.0..w.1].to_vec());
        }

        let op = match numbers.last().unwrap()[0] {
            '+' => Operation::Add,
            '*' => Operation::Multiply,
            _ => unreachable!("Invalid operation"),
        };

        calculations.push(Calculation {
            numbers: numbers[..numbers.len() - 1].to_vec(),
            operation: op,
        });
    }
    Ok(calculations)
}

pub fn part1(input: &[Calculation]) -> u64 {
    let mut acc = 0;
    for calculation in input {
        let terms: Vec<u64> = calculation
            .numbers
            .iter()
            .map(String::from_iter)
            .map(|s| s.trim().to_string().parse().unwrap())
            .collect();
        acc += match calculation.operation {
            Operation::Add => terms.iter().sum::<u64>(),
            Operation::Multiply => terms.iter().product::<u64>(),
        }
    }
    acc
}

pub fn part2(_input: &[Calculation]) -> u64 {
    let mut acc = 0;
    for calculation in _input {
        let mut terms: Vec<u64> = Vec::new();
        for c in (0..calculation.numbers[0].len()).rev() {
            let term = String::from_iter(calculation.numbers.iter().map(|row| row[c].to_string()))
                .trim()
                .parse::<u64>()
                .unwrap();
            terms.push(term);
        }

        acc += match calculation.operation {
            Operation::Add => terms.iter().sum::<u64>(),
            Operation::Multiply => terms.iter().product::<u64>(),
        }
    }
    acc
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
pub struct Calculation {
    numbers: Vec<Vec<char>>,
    operation: Operation,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 3263827);
    }
}
