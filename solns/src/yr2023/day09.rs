use std::iter;

use crate::Soln;
use utils::FromWords;

type Num = i64;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = Num;

    fn part1(input: &str) -> Self::Answer {
        extrapolate(input, |acc, diffs| diffs.last() + acc)
    }

    fn part2(input: &str) -> Self::Answer {
        extrapolate(input, |acc, diffs| diffs.first() - acc)
    }
}

fn extrapolate<F>(input: &str, f: F) -> Num
where F: Fn(Num, &Hist) -> Num
{
    input.lines()
        .map(|line| Hist::successors(line.parse().unwrap())
            .iter().rev()
            .fold(0, &f))
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, FromWords)]
struct Hist(Vec<Num>);

impl Hist {
    fn successors(start: Self) -> Vec<Self> {
        iter::successors(Some(start), |prev| {
            let next = prev.diffs();
            if next.is_const() {
                None
            } else {
                Some(next)
            }
        }).collect()
    }

    fn diffs(&self) -> Self {
        Self(self.0.as_slice()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect())
    }

    fn is_const(&self) -> bool {
        self.0.iter().all(|&x| x == 0)
    }

    fn first(&self) -> Num {
        self.0[0]
    }

    fn last(&self) -> Num {
        *self.0.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "0 3 6 9 12 15
         1 3 6 10 15 21
         10 13 16 21 30 45";

    #[test]
    fn part1() {
        assert_eq!(114, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(2, Puzzle::part2(&INPUT));
    }
}
