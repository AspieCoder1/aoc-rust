use crate::utils::grid::{Grid, Pos};
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

pub fn main(data: &str) -> Result<(usize, usize)> {
    let input = parse_input(data)?;

    Ok((part1(&input), part2(&input)))
}

type Tile = (usize, usize);

pub fn parse_input(input: &str) -> Result<Vec<Tile>> {
    Ok(input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>())
}

pub fn part1(input: &[Tile]) -> usize {
    let mut acc = 0;
    for (i, &(x0, y0)) in input.iter().enumerate() {
        for &(x1, y1) in input.iter().skip(i + 1) {
            let length = x0.abs_diff(x1) + 1;
            let height = y0.abs_diff(y1) + 1;
            acc = acc.max(length * height);
        }
    }
    acc
}

struct Compressed {
    map: HashMap<usize, usize>,
    sizes: Vec<usize>,
}

impl Compressed {
    fn new(points: &[usize]) -> Self {
        // double up each point so there is an entry for every gap
        let doubled = points
            .iter()
            .flat_map(|&point| [point, point + 1])
            .collect::<Vec<_>>();
        let map: HashMap<_, _> = doubled
            .iter()
            .copied()
            .enumerate()
            .map(|(i, pt)| (pt, i))
            .collect();
        let sizes: Vec<_> = doubled
            .iter()
            .copied()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect();
        Self { map, sizes }
    }

    fn get(&self, point: usize) -> usize {
        self.map[&point]
    }

    fn len(&self) -> usize {
        self.sizes.len()
    }
}

pub fn part2(input: &[Tile]) -> usize {
    let points = input.to_vec();

    let x_coords = points
        .iter()
        .map(|&(x, _)| x)
        .sorted()
        .dedup()
        .collect::<Vec<_>>();
    let y_coords = points
        .iter()
        .map(|&(_, y)| y)
        .sorted()
        .dedup()
        .collect::<Vec<_>>();

    let xcomp = Compressed::new(&x_coords);
    let ycomp = Compressed::new(&y_coords);

    let mut g = Grid::<bool>::new(false, xcomp.len(), ycomp.len());
    // draw the contour in compressed coordinates.
    for ((x1, y1), (x2, y2)) in points
        .iter()
        .chain(std::iter::once(&points[0]))
        .map(|&(x, y)| (xcomp.get(x), ycomp.get(y)))
        .tuple_windows()
    {
        if x1 == x2 {
            for i in y1.min(y2)..=y1.max(y2) {
                g[(i, x1)] = true;
            }
        } else {
            for j in x1.min(x2)..=x1.max(x2) {
                g[(y1, j)] = true;
            }
        }
    }
    let (x0, y0) = points[0];
    let (cx0, cy0) = (xcomp.get(x0), ycomp.get(y0));

    let fill_start = g
        .all_neighbors(Pos(cy0, cx0))
        .find(|&pos| g.is_inside_polygon(pos, |b| *b))
        .unwrap();

    g.flood_fill(fill_start, true, |b| *b);

    let mut psum = Grid::<usize>::new(0, g.width, g.height);
    psum[(0, 0)] = g[(0, 0)] as usize;
    for i in 1..psum.height {
        psum[(i, 0)] = psum[(i - 1, 0)] + g[(i, 0)] as usize;
    }
    for j in 1..psum.width {
        psum[(0, j)] = psum[(0, j - 1)] + g[(0, j)] as usize;
    }
    for i in 1..psum.height {
        for j in 1..psum.width {
            psum[(i, j)] =
                psum[(i - 1, j)] + psum[(i, j - 1)] - psum[(i - 1, j - 1)] + g[(i, j)] as usize;
        }
    }

    points
        .iter()
        .tuple_combinations()
        .filter_map(|(&(x1, y1), &(x2, y2))| {
            let (i1, j1) = (ycomp.get(y1.min(y2)), xcomp.get(x1.min(x2)));
            let (i2, j2) = (ycomp.get(y1.max(y2)), xcomp.get(x1.max(x2)));
            let expected = (i2 - i1 + 1) * (j2 - j1 + 1);
            let mut actual = psum[(i2, j2)];
            if i1 > 0 && j1 > 0 {
                actual += psum[(i1 - 1, j1 - 1)];
            }
            if i1 > 0 {
                actual -= psum[(i1 - 1, j2)];
            }
            if j1 > 0 {
                actual -= psum[(i2, j1 - 1)];
            }
            if expected == actual {
                Some((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1))
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part1(&input), 50);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE).unwrap();
        assert_eq!(part2(&input), 24);
    }
}
