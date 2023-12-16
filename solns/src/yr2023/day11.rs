use std::collections::HashSet;

use crate::Soln;
use utils::prelude::*;

type Num = i64;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = Num;

    fn part1(input: &str) -> Self::Answer {
        solve(input, 1)
    }

    fn part2(input: &str) -> Self::Answer {
        solve(input, 999999)
    }
}

fn solve(input: &str, expansion: Num) -> Num {
    // parse grid
    let mut nonempty_rows = HashSet::new();
    let mut nonempty_cols = HashSet::new();
    let mut galaxies: Vec<Point<Num>> = vec![];
    let mut w = 0;
    let mut h = 0;
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if let Ok(Cell::Galaxy) = ch.try_into() {
                galaxies.push(Point(
                        row.try_into().unwrap(),
                        col.try_into().unwrap()));

                nonempty_rows.insert(row);
                nonempty_cols.insert(col);
            }
        }

        h += 1;
        w = line.len();
    }

    // expand universe
    let row_offsets = calculate_offsets(nonempty_rows, h, expansion);
    let col_offsets = calculate_offsets(nonempty_cols, w, expansion);
    for galaxy in galaxies.iter_mut() {
        galaxy.0 += row_offsets[galaxy.0 as usize];
        galaxy.1 += col_offsets[galaxy.1 as usize];
    }

    // calculate pairwise distances
    galaxies.combos()
        .map(|(g1, g2)| (g2.0 - g1.0).abs() + (g2.1 - g1.1).abs())
        .sum()
}

fn calculate_offsets(
    nonempty: HashSet<usize>,
    len: usize,
    inc: Num,
) -> Vec<Num> {
    let mut offsets = vec![];

    offsets.push(if nonempty.contains(&0) { 0 } else { inc });
    for i in 1..len {
        offsets.push(if nonempty.contains(&i) {
            offsets[i - 1]
        } else {
            offsets[i - 1] + inc
        });
    }

    offsets
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Galaxy,
    Space,
}

impl TryFrom<char> for Cell {
    type Error = ();
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '.' => Ok(Cell::Space),
            '#' => Ok(Cell::Galaxy),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    #[ignore]
    fn part1() {
        //assert_eq!((), Puzzle::part1(""));
    }

    #[test]
    #[ignore]
    fn part2() {
        //assert_eq!((), Puzzle::part2(""));
    }
}
