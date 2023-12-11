use std::collections::HashSet;

use itertools::Itertools;

#[aoc_generator(day11)]
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '.' => None,
                    '#' => Some(x as i64),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn distances_sum(galaxies: &[(i64, i64)]) -> i64 {
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            let a = galaxies[i];
            let b = galaxies[j];
            sum += a.0.max(b.0) - a.0.min(b.0) + a.1.max(b.1) - a.1.min(b.1);
        }
    }
    sum
}

fn expand(input: &[Vec<i64>], factor: i64) -> Vec<(i64, i64)> {
    // Because a factor of 2 means an expansion of 1 ("twice as big")
    let factor = factor - 1;
    let empty_y: Vec<i64> = input
        .iter()
        .enumerate()
        .filter_map(|(index, line)| line.is_empty().then_some(index as i64))
        .collect();
    let mut cols: HashSet<i64> = HashSet::default();
    let mut highest_x = 0;
    for line in input {
        for x in line {
            cols.insert(*x);
            highest_x = highest_x.max(*x);
        }
    }
    let empty_x: Vec<i64> = (0..=highest_x)
        .filter_map(|x| (!cols.contains(&x)).then_some(x))
        .sorted()
        .collect();

    input
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().map(move |x| (*x, y as i64)))
        // Yes, this could be a single flat_map call but we have ownership issues if we do
        .map(|(x, y)| {
            let count_x = empty_x.iter().filter(|xx| **xx < x).count() as i64;
            let count_y = empty_y.iter().filter(|yy| **yy < y).count() as i64;
            let x = x + count_x * factor;
            let y = y + count_y * factor;
            (x, y)
        })
        .collect()
}

#[aoc(day11, part1)]
fn part1(input: &[Vec<i64>]) -> i64 {
    let expanded = expand(input, 2);
    distances_sum(&expanded)
}

#[aoc(day11, part2)]
fn part2(input: &[Vec<i64>]) -> i64 {
    let expanded = expand(input, 1_000_000);
    distances_sum(&expanded)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn day11() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 374);

        let expanded = expand(&input, 10);
        assert_eq!(distances_sum(&expanded), 1030);

        let expanded = expand(&input, 100);
        assert_eq!(distances_sum(&expanded), 8410);
    }
}
