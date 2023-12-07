use std::{cmp::Ordering, collections::HashMap};

use aoc_parse::{
    parser,
    prelude::{lines, usize},
    Parser,
};
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum CardPartTwo {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Q,
    K,
    A,
}

impl From<Card> for CardPartTwo {
    fn from(value: Card) -> Self {
        match value {
            Card::A => CardPartTwo::A,
            Card::K => CardPartTwo::K,
            Card::Q => CardPartTwo::Q,
            Card::J => CardPartTwo::J,
            Card::Ten => CardPartTwo::Ten,
            Card::Nine => CardPartTwo::Nine,
            Card::Eight => CardPartTwo::Eight,
            Card::Seven => CardPartTwo::Seven,
            Card::Six => CardPartTwo::Six,
            Card::Five => CardPartTwo::Five,
            Card::Four => CardPartTwo::Four,
            Card::Three => CardPartTwo::Three,
            Card::Two => CardPartTwo::Two,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand<C> {
    cards: Vec<C>,
}

trait GetHandKind {
    fn kind(&self) -> HandKind;
}

impl GetHandKind for Hand<Card> {
    fn kind(&self) -> HandKind {
        let count_cards = self
            .cards
            .iter()
            .fold(HashMap::default(), |mut acc: HashMap<Card, usize>, curr| {
                let entry = acc.entry(*curr).or_default();
                *entry += 1;
                acc
            })
            .into_iter()
            .sorted_by_key(|(_, v)| *v)
            .map(|(_, v)| v)
            .rev()
            .collect::<Vec<_>>();
        match count_cards.as_slice() {
            [5, ..] => HandKind::FiveOfAKind,
            [4, ..] => HandKind::FourOfAKind,
            [3, 2, ..] => HandKind::FullHouse,
            [3, ..] => HandKind::ThreeOfAKind,
            [2, 2, ..] => HandKind::TwoPair,
            [2, ..] => HandKind::OnePair,
            _ => HandKind::HighCard,
        }
    }
}

impl GetHandKind for Hand<CardPartTwo> {
    fn kind(&self) -> HandKind {
        let count_cards: HashMap<CardPartTwo, usize> =
            self.cards.iter().fold(HashMap::default(), |mut acc, curr| {
                let entry = acc.entry(*curr).or_default();
                *entry += 1;
                acc
            });
        let count_j = count_cards.get(&CardPartTwo::J).copied().unwrap_or(0);
        if count_j == 5 {
            return HandKind::FiveOfAKind;
        }
        let mut count_cards = count_cards
            .into_iter()
            .filter(|(k, _)| k != &CardPartTwo::J)
            .sorted_by_key(|(_, v)| *v)
            .map(|(_, v)| v)
            .rev()
            .collect::<Vec<_>>();
        count_cards[0] += count_j;
        match count_cards.as_slice() {
            [5, ..] => HandKind::FiveOfAKind,
            [4, ..] => HandKind::FourOfAKind,
            [3, 2, ..] => HandKind::FullHouse,
            [3, ..] => HandKind::ThreeOfAKind,
            [2, 2, ..] => HandKind::TwoPair,
            [2, ..] => HandKind::OnePair,
            _ => HandKind::HighCard,
        }
    }
}

impl<C: PartialEq + Ord + std::fmt::Debug> PartialOrd for Hand<C>
where
    Hand<C>: GetHandKind,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.kind().cmp(&other.kind()).then_with(|| {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                match a.cmp(b) {
                    Ordering::Less => return Ordering::Less,
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Equal => (),
                }
            }
            Ordering::Equal
        }))
    }
}

impl<C: Eq> Ord for Hand<C>
where
    Hand<C>: PartialOrd,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
pub struct Draw {
    hand: Hand<Card>,
    bid: usize,
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<Draw> {
    let card = parser!({
        "A" => Card::A,
        "K" => Card::K,
        "Q" => Card::Q,
        "J" => Card::J,
        "T" => Card::Ten,
        "9" => Card::Nine,
        "8" => Card::Eight,
        "7" => Card::Seven,
        "6" => Card::Six,
        "5" => Card::Five,
        "4" => Card::Four,
        "3" => Card::Three,
        "2" => Card::Two,
    });
    let line = parser!(cards:card* " " bid:usize => Draw { hand: Hand { cards }, bid });
    lines(line).parse(input).expect("Failed to parse input")
}

#[aoc(day7, part1)]
fn part1(input: &[Draw]) -> usize {
    input
        .iter()
        .sorted_by_key(|draw| draw.hand.clone())
        .enumerate()
        .map(|(rank, draw)| (rank + 1) * draw.bid)
        .sum()
}

#[aoc(day7, part2)]
fn part2(input: &[Draw]) -> usize {
    input
        .iter()
        .sorted_by_key(|draw| Hand {
            cards: draw
                .hand
                .cards
                .iter()
                .cloned()
                .map(CardPartTwo::from)
                .collect::<Vec<_>>(),
        })
        .enumerate()
        .map(|(rank, draw)| (rank + 1) * draw.bid)
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    const INPUT: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    fn assert_hand_kind<C>(cards: Vec<C>, kind: HandKind)
    where
        Hand<C>: GetHandKind,
    {
        assert_eq!(Hand { cards }.kind(), kind);
    }

    #[test]
    fn day7_kind() {
        assert_hand_kind(
            vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::K],
            HandKind::OnePair,
        );
        assert_hand_kind(
            vec![Card::Ten, Card::Five, Card::Five, Card::J, Card::Five],
            HandKind::ThreeOfAKind,
        );
        assert_hand_kind(
            vec![Card::K, Card::K, Card::Six, Card::Seven, Card::Seven],
            HandKind::TwoPair,
        );
        assert_hand_kind(
            vec![Card::K, Card::Ten, Card::J, Card::J, Card::Ten],
            HandKind::TwoPair,
        );
        assert_hand_kind(
            vec![Card::Q, Card::Q, Card::Q, Card::J, Card::A],
            HandKind::ThreeOfAKind,
        );

        assert_hand_kind(
            vec![
                CardPartTwo::Three,
                CardPartTwo::Two,
                CardPartTwo::Ten,
                CardPartTwo::Three,
                CardPartTwo::K,
            ],
            HandKind::OnePair,
        );
        assert_hand_kind(
            vec![
                CardPartTwo::Ten,
                CardPartTwo::Five,
                CardPartTwo::Five,
                CardPartTwo::J,
                CardPartTwo::Five,
            ],
            HandKind::FourOfAKind,
        );
        assert_hand_kind(
            vec![
                CardPartTwo::K,
                CardPartTwo::K,
                CardPartTwo::Six,
                CardPartTwo::Seven,
                CardPartTwo::Seven,
            ],
            HandKind::TwoPair,
        );
        assert_hand_kind(
            vec![
                CardPartTwo::K,
                CardPartTwo::Ten,
                CardPartTwo::J,
                CardPartTwo::J,
                CardPartTwo::Ten,
            ],
            HandKind::FourOfAKind,
        );
        assert_hand_kind(
            vec![
                CardPartTwo::Q,
                CardPartTwo::Q,
                CardPartTwo::Q,
                CardPartTwo::J,
                CardPartTwo::A,
            ],
            HandKind::FourOfAKind,
        );
    }

    #[test]
    fn day7() {
        let input = parse(INPUT);
        assert_eq!(part1(&input), 6440);
        assert_eq!(part2(&input), 5905);
    }
}
