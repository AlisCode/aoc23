use std::collections::HashSet;

use aoc_parse::{
    parser,
    prelude::{i32, repeat_sep, usize},
    Parser,
};

#[derive(Debug, PartialEq)]
pub struct Card {
    winning: HashSet<i32>,
    have: HashSet<i32>,
}

impl Card {
    pub fn matching(&self) -> usize {
        self.winning.intersection(&self.have).count()
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Card> {
    let number = parser!(" "? n:i32 => n);
    let list_numbers = parser!(repeat_sep(number, " "));
    let line = parser!("Card " " "? " "? usize ": " list_numbers " | " list_numbers);
    input
        .lines()
        .map(|l| {
            let (_, _, _, winning, have) = line.parse(l).expect("Failed to parse line");
            Card {
                winning: winning.into_iter().collect(),
                have: have.into_iter().collect(),
            }
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|card| {
            let count = card.matching() as u32;
            if count == 0 {
                return 0;
            }
            2u32.pow(count - 1)
        })
        .sum()
}

#[aoc(day4, part2)]
fn part2(input: &[Card]) -> usize {
    let mut copies: Vec<usize> = (0..input.len()).map(|_| 1).collect();
    for (index, card) in input.iter().enumerate() {
        let matching = card.matching();
        for i in 1..=matching {
            copies[index + i] += copies[index];
        }
    }
    copies.into_iter().sum()
}

#[cfg(test)]
pub mod tests {
    use maplit::hashset;

    use super::*;

    const INPUT: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn day4_parse() {
        let input = parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(
            input,
            vec![Card {
                winning: hashset![41, 48, 83, 86, 17],
                have: hashset![83, 86, 6, 31, 17, 9, 48, 53],
            }]
        );
    }

    #[test]
    fn day4() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 13);
        assert_eq!(part2(&input), 30);
    }
}
