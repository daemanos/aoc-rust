use std::cmp::Ordering;
use std::collections::HashMap;

use crate::Soln;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = u64;

    fn part1(input: &str) -> Self::Answer {
        let mut hands = parse(input);
        hands.sort_by_key(|(hand, _)| hand.clone());

        hands.iter()
            .zip(1..)
            .map(|((_, bid), rank)| bid * rank)
            .sum()
    }

    fn part2(input: &str) -> Self::Answer {
        let mut hands = parse(input);
        hands.sort_by(|(h1, _), (h2, _)| h1.joker_cmp(&h2));

        hands.iter()
            .zip(1..)
            .map(|((_, bid), rank)| bid * rank)
            .sum()
    }
}

fn parse(input: &str) -> Vec<(Hand, u64)> {
    input.lines().map(|line| {
        let (hand, bid) = line.trim().split_once(' ').unwrap();
        let hand = Hand(hand.chars().map(Card).collect());
        let bid = bid.parse().unwrap();

        (hand, bid)
    }).collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand(Vec<Card>);

impl Hand {
    fn kind(&self) -> Kind {
        let mut counts = HashMap::new();
        for card in &self.0 {
            counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut counts: Vec<u32> = counts.into_values().collect();
        counts.sort_unstable();
        match counts.as_slice() {
            &[5] => Kind::FiveKind,
            &[1, 4] => Kind::FourKind,
            &[2, 3] => Kind::FullHouse,
            &[1, 1, 3] => Kind::ThreeKind,
            &[1, 2, 2] => Kind::TwoPair,
            &[1, 1, 1, 2] => Kind::OnePair,
            _ => Kind::HighCard,
        }
    }

    fn joker_kind(&self) -> Kind {
        let mut counts = HashMap::new();
        let mut jokers = 0;
        for card in &self.0 {
            if card.is_joker() {
                jokers += 1;
            } else {
                counts.entry(card).and_modify(|c| *c += 1).or_insert(1);
            }
        }

        let mut counts: Vec<u32> = counts.into_values().collect();
        counts.sort_unstable();
        match counts.as_slice() {
            &[] if jokers == 5 => Kind::FiveKind,
            &[c1] if c1 + jokers == 5 => Kind::FiveKind,
            &[1, c1] if c1 + jokers == 4 => Kind::FourKind,
            &[2, c1] if c1 + jokers == 3 => Kind::FullHouse,
            &[1, 1, c1] if c1 + jokers == 3 => Kind::ThreeKind,
            &[1, c1, c2] if c1 + c2 + jokers == 4 => Kind::TwoPair,
            &[1, 1, 1, c1] if c1 + jokers == 2 => Kind::OnePair,
            _ => Kind::HighCard,
        }
    }

    fn joker_cmp(&self, other: &Self) -> Ordering {
        self.joker_kind().cmp(&other.joker_kind()).then_with(|| {
            self.0.iter().zip(other.0.iter())
                .map(|(c1, c2)| c1.joker_value().cmp(&c2.joker_value()))
                .fold(Ordering::Equal, Ordering::then)
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind().cmp(&other.kind()).then_with(|| {
            self.0.iter().zip(other.0.iter())
                .map(|(c1, c2)| c1.cmp(&c2))
                .fold(Ordering::Equal, Ordering::then)
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Card(char);

impl Card {
    fn value(self) -> u8 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!(),
        }
    }

    fn is_joker(self) -> bool {
        match self.0 {
            'J' => true,
            _ => false,
        }
    }

    fn joker_value(self) -> u8 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            _ => panic!(),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "32T3K 765
         T55J5 684
         KK677 28
         KTJJT 220
         QQQJA 483";

    #[test]
    fn part1() {
        assert_eq!(6440, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(5905, Puzzle::part2(&INPUT));
    }
}
