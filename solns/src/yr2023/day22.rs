use crate::Soln;
use utils::prelude::*;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = ();

    fn part1(_input: &str) -> Self::Answer {
        unsolved!()
    }

    fn part2(_input: &str) -> Self::Answer {
        unsolved!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn part1() {
        assert_eq!((), Puzzle::part1(""));
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!((), Puzzle::part2(""));
    }
}
