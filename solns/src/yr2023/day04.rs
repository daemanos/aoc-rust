use std::str::FromStr;
use std::collections::HashSet;

use crate::Soln;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = u32;

    fn part1(input: &str) -> Self::Answer {
        input.lines()
            .map(|line| line.parse::<Card>().unwrap().score())
            .sum()
    }

    fn part2(input: &str) -> Self::Answer {
        let cards: Vec<Card> = input.lines()
            .map(|line| line.parse::<Card>().unwrap())
            .collect();

        let mut copies: Vec<u32> = (0..cards.len()).map(|_| 1).collect();
        for (i, card) in cards.iter().enumerate() {
            let matches = card.num_matches();
            for j in 1..=matches {
                let copy = i + j;
                if copy < copies.len() {
                    copies[copy] += copies[i];
                }
            }
        }

        copies.iter().sum()
    }
}

struct Card {
    winning: HashSet<u32>,
    owned: Vec<u32>,
}

impl Card {
    fn num_matches(&self) -> usize {
        self.owned.iter().filter(|&n| self.winning.contains(n)).count()
    }

    fn score(&self) -> u32 {
        let exp = self.num_matches();
        match exp {
            0 => 0,
            _ => 2_u32.pow((exp - 1).try_into().unwrap()),
        }
    }
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, s) = s.split_once(':').unwrap();
        let (winning, owned) = s.split_once('|').unwrap();

        let winning = winning.trim()
            .split(char::is_whitespace)
            .filter_map(|n| n.parse().ok())
            .collect();

        let owned = owned.trim()
            .split(char::is_whitespace)
            .filter_map(|n| n.parse().ok())
            .collect();

        Ok(Self { winning, owned })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1() {
        assert_eq!(13, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(30, Puzzle::part2(&INPUT));
    }
}
