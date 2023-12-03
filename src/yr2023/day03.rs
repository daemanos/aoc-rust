use std::collections::{HashSet, VecDeque};
use std::fmt;

use colored::*;

use crate::Soln;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = u32;

    fn part1(input: &str) -> Self::Answer {
        let grid = Grid::parse(input);

        let mut sum = 0;
        let mut n = 0;
        let mut is_part = false;
        for row in 0..grid.height {
            for col in 0..grid.width {
                match grid.cells[row][col] {
                    Cell::Num(d) => {
                        n = 10*n + d;
                        is_part = is_part || grid.is_part(row, col);
                    }
                    _ => {
                        if is_part {
                            sum += n;
                        }

                        n = 0;
                        is_part = false;
                    }
                }
            }
        }

        if is_part {
            sum += n;
        }

        sum
    }

    fn part2(input: &str) -> Self::Answer {
        let grid = Grid::parse(input);

        let mut sum = 0;
        for row in 0..grid.height {
            for col in 0..grid.width {
                if let Cell::Sym('*') = grid.cells[row][col] {
                    let mut digitss = vec![];

                    grid.adj_row(&mut digitss, row - 1, col);
                    grid.adj_row(&mut digitss, row + 1, col);

                    if let Num(d) = grid.cells[row][col-1] {
                        digitss.push(Digits::new(vec![d], row, col-1));
                    }
                    if let Num(d) = grid.cells[row][col+1] {
                        digitss.push(Digits::new(vec![d], row, col+1));
                    }

                    if let [ref mut d1, ref mut d2] = digitss.as_mut_slice() {
                        d1.grow(&grid);
                        d2.grow(&grid);

                        sum += d1.to_int() * d2.to_int();
                    }
                }
            }
        }

        sum
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    mask: HashSet<(i32, i32)>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let cells: Vec<Vec<Cell>> = input.lines()
            .map(|line| line.trim().chars().map(|ch| Cell::new(ch)).collect())
            .collect();

        let height = cells.len();
        let width = cells[0].len();

        let mut mask = HashSet::new();
        for (i, row) in cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if let Cell::Sym(_) = cell {
                    for neighbor in neighbors(i, j) {
                        mask.insert(neighbor);
                    }
                }
            }
        }

        Self { cells, mask, width, height }
    }

    fn is_part(&self, row: usize, col: usize) -> bool {
        self.mask.contains(&(row as i32, col as i32))
    }

    fn _draw(&self) {
        let mut buf = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let s = self.cells[row][col].to_string();
                if self.is_part(row, col) {
                    buf.push_str(&s.bold());
                } else {
                    buf.push_str(&s);
                }
            }

            buf.push('\n');
        }

        println!("{buf}");
    }

    fn adj_row(&self, ns: &mut Vec<Digits>, row: usize, col: usize) {
        match &self.cells[row][col-1..=col+1] {
            &[Num(d1), Num(d2), Num(d3)] => ns.push(Digits::new(vec![d1, d2, d3], row, col-1)),
            &[Num(d1), _, Num(d2)] => {
                ns.push(Digits::new(vec![d1], row, col-1));
                ns.push(Digits::new(vec![d2], row, col+1));
            }
            &[_, Num(d1), Num(d2)] => ns.push(Digits::new(vec![d1, d2], row, col)),
            &[Num(d1), Num(d2), _] => ns.push(Digits::new(vec![d1, d2], row, col-1)),
            &[Num(d), _, _] => ns.push(Digits::new(vec![d], row, col-1)),
            &[_, Num(d), _] => ns.push(Digits::new(vec![d], row, col)),
            &[_, _, Num(d)] => ns.push(Digits::new(vec![d], row, col+1)),
            _ => (),
        };
    }
}

fn neighbors(row: usize, col: usize) -> Vec<(i32, i32)> {
    let mut vec = vec![];

    for drow in -1..=1 {
        for dcol in -1..=1 {
            let row = (row as i32) + drow;
            let col = (col as i32) + dcol;

            vec.push((row, col));
        }
    }

    vec
}

#[derive(Debug)]
struct Digits {
    ds: VecDeque<u32>,
    row: usize,
    span: (usize, usize),
}

impl Digits {
    fn new(ds: Vec<u32>, row: usize, col: usize) -> Self {
        let span = (col, col + ds.len());
        Self { ds: ds.into(), row, span }
    }

    fn grow(&mut self, grid: &Grid) {
        for col in (0..self.span.0).rev() {
            if let Num(d) = grid.cells[self.row][col] {
                self.ds.push_front(d);
            } else {
                break;
            }
        }

        for col in self.span.1..grid.width {
            if let Num(d) = grid.cells[self.row][col] {
                self.ds.push_back(d);
            } else {
                break;
            }
        }
    }

    fn to_int(&self) -> u32 {
        (0..).zip(self.ds.iter().rev())
            .map(|(place, digit)| 10_u32.pow(place) * digit)
            .sum()
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Num(u32),
    Sym(char),
    Empty,
}

use Cell::*;

impl Cell {
    fn new(ch: char) -> Self {
        match ch {
            '0' => Self::Num(0),
            '1' => Self::Num(1),
            '2' => Self::Num(2),
            '3' => Self::Num(3),
            '4' => Self::Num(4),
            '5' => Self::Num(5),
            '6' => Self::Num(6),
            '7' => Self::Num(7),
            '8' => Self::Num(8),
            '9' => Self::Num(9),
            '.' => Self::Empty,
            _ => Self::Sym(ch),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch = match self {
            Self::Num(d) => char::from_digit(*d, 10).unwrap(),
            Self::Sym(ch) => *ch,
            Self::Empty => '.',
        };

        write!(f, "{ch}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str =
        "467..114..
         ...*......
         ..35..633.
         ......#...
         617*......
         .....+.58.
         ..592.....
         ......755.
         ...$.*....
         .664.598..";

    #[test]
    fn part1() {
        assert_eq!(4361, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(467835, Puzzle::part2(&INPUT));
    }
}
