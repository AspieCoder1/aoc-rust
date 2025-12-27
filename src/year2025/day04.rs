use anyhow::Result;

const INPUT_NUM: usize = 0;

pub fn main(data: &str) -> Result<(u32, u32)> {
    let input = parse_input(data)?;

    Ok((part1(&input), part2(&input)))
}

pub fn parse_input(input: &str) -> Result<Vec<Vec<char>>> {
    // Read input
    let mut input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Add border to prevent boundary checks
    for row in &mut input {
        row.insert(0, '#');
        row.push('#');
    }
    input.insert(0, std::iter::repeat_n('#', input[0].len() + 2).collect());
    input.push(std::iter::repeat_n('#', input[0].len() + 2).collect());
    Ok(input)
}

pub fn part1(input: &[Vec<char>]) -> u32 {
    get_accessible_cells(input).len() as u32
}

pub fn part2(input: &[Vec<char>]) -> u32 {
    let mut acc = 0;
    let mut input = input.to_vec();
    loop {
        let cells = get_accessible_cells(&input);
        if cells.is_empty() {
            break;
        }

        for (r, c) in cells {
            input[r][c] = '.';
            acc += 1;
        }
    }
    acc
}

pub fn get_accessible_cells(input: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut cells = vec![];
    // Iterating through each cell in the matrix
    for (r, row) in input.iter().enumerate().skip(1).take(input.len() - 2) {
        for (c, cell) in row.iter().enumerate().skip(1).take(row.len() - 2) {
            if *cell != '@' {
                continue;
            }

            let num_neighbors = [
                input[r - 1][c - 1],
                input[r - 1][c],
                input[r - 1][c + 1],
                input[r][c - 1],
                input[r][c + 1],
                input[r + 1][c - 1],
                input[r + 1][c],
                input[r + 1][c + 1],
            ]
            .iter()
            .filter(|&&c| c == '@')
            .count();

            if num_neighbors < 4 {
                cells.push((r, c));
            }
        }
    }
    cells
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        let input = parse_input(1).unwrap();
        assert_eq!(part1(&input), 13);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(1).unwrap();
        assert_eq!(part2(&input), 43);
    }
}
