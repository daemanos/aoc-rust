use std::iter;
use std::str::{Chars, FromStr};
use std::ops::Index;

use super::{PeekFrom, Point};

pub type IdxPoint = Point<usize>;

pub trait Grid<T> : Index<IdxPoint, Output = T> {
    fn dim(&self) -> (usize, usize);

    fn in_bounds(&self, point: IdxPoint) -> bool {
        let (w, h) = self.dim();
        point.0 > 0 && point.0 <= h && point.1 > 0 && point.1 <= w
    }

    fn get(&self, point: IdxPoint) -> Option<&T> {
        if self.in_bounds(point) {
            Some(&self[point])
        } else {
            None
        }
    }

    fn ortho_neighbors(&self, point: IdxPoint) -> Vec<&T> {
        point.ortho_neighbors()
            .filter_map(|p| self.get(p))
            .collect()
    }

    fn neighbors(&self, point: IdxPoint) -> Vec<&T> {
        point.neighbors()
            .filter_map(|p| self.get(p))
            .collect()

        //let (w, h) = self.dim();
        //if point.0 >= h || point.1 >= w {
        //    return None;
        //}

        //Some(match point {
        //    // corners
        //    Point(0, 0) => vec![
        //        &self[Point(0, 1)],
        //        &self[Point(1, 0)],
        //        &self[Point(1, 1)],
        //    ],
        //    Point(0, col) if col == w - 1 => vec![
        //        &self[Point(0, w - 2)],
        //        &self[Point(1, w - 1)],
        //        &self[Point(1, w - 2)],
        //    ],
        //    Point(row, 0) if row == h - 1 => vec![
        //        &self[Point(h - 2, 0)],
        //        &self[Point(h - 1, 1)],
        //        &self[Point(h - 2, 1)],
        //    ],
        //    Point(row, col) if row == h - 1 && col == w - 1 => vec![
        //        &self[Point(h - 1, w - 2)],
        //        &self[Point(h - 2, w - 1)],
        //        &self[Point(h - 2, w - 2)],
        //    ],

        //    // edges
        //    Point(0, col) => vec![
        //        &self[Point(0, col - 1)],
        //        &self[Point(0, col + 1)],
        //        &self[Point(1, col - 1)],
        //        &self[Point(1, col    )],
        //        &self[Point(1, col + 1)],
        //    ],
        //    Point(row, 0) => vec![
        //        &self[Point(row - 1, 0)],
        //        &self[Point(row + 1, 0)],
        //        &self[Point(row - 1, 1)],
        //        &self[Point(row,     1)],
        //        &self[Point(row + 1, 1)],
        //    ],
        //    Point(row, col) if row == h - 1 => vec![
        //        &self[Point(row,     col - 1)],
        //        &self[Point(row - 1, col - 1)],
        //        &self[Point(row - 1, col    )],
        //        &self[Point(row - 1, col + 1)],
        //        &self[Point(row,     col + 1)],
        //    ],
        //    Point(row, col) if col == w - 1 => vec![
        //        &self[Point(row - 1, col    )],
        //        &self[Point(row - 1, col - 1)],
        //        &self[Point(row,     col - 1)],
        //        &self[Point(row + 1, col - 1)],
        //        &self[Point(row + 1, col    )],
        //    ],

        //    // interior
        //    Point(row, col) => vec![
        //        &self[Point(row - 1, col - 1)],
        //        &self[Point(row - 1, col    )],
        //        &self[Point(row - 1, col + 1)],
        //        &self[Point(row    , col - 1)],
        //        &self[Point(row    , col + 1)],
        //        &self[Point(row + 1, col - 1)],
        //        &self[Point(row + 1, col    )],
        //        &self[Point(row + 1, col + 1)],
        //    ],
        //})
    }
}

#[derive(Debug, PartialEq)]
pub enum GridParseError {
    Empty,
    NonRect,
}

#[derive(Debug, PartialEq)]
pub struct Vec2D<T> {
    cells: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Vec2D<T> {
    fn new() -> Self {
        Self {
            cells: vec![],
            width: 0,
            height: 0,
        }
    }

    fn push_row(&mut self, row: Vec<T>) {
        if row.len() == self.width {
            self.cells.push(row);
            self.height += 1;
        } else {
            panic!("argument to push_row must have the same length as the grid width");
        }
    }

    fn push_col(&mut self, col: Vec<T>) {
        if col.len() == self.height {
            self.cells.iter_mut().zip(col)
                .for_each(|(row, cell)| row.push(cell));
            self.width += 1;
        } else {
            panic!("argument to push_col must have the same length as the grid height");
        }
    }

    //fn iter(&self) -> impl Iterator<Item = &T> {
    //    self.cells.iter()
    //        .map(|row| row.iter())
    //        .reduce(|acc, e| acc.chain(e).into_iter())
    //        .unwrap()
    //}
}

impl<T> Index<IdxPoint> for Vec2D<T> {
    type Output = T;
    fn index(&self, index: IdxPoint) -> &Self::Output {
        &self.cells[index.0 - 1][index.1 - 1]
    }
}

impl<T> Grid<T> for Vec2D<T> {
    fn dim(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl<T> FromStr for Vec2D<T>
where T: for<'a> PeekFrom<Chars<'a>>
{
    type Err = GridParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = Vec::new();
        let mut width = None;

        for line in s.lines() {
            let mut row = Vec::new();
            let mut chars = line.trim().chars().peekable();

            while let Some(cell) = T::peek_from(&mut chars) {
                row.push(cell);
            }

            width = match width {
                None => Some(row.len()),
                Some(width) => {
                    if width == row.len() {
                        Some(width)
                    } else {
                        return Err(GridParseError::NonRect);
                    }
                }
            };
            cells.push(row);
        }

        let width = width.unwrap_or(0);
        if width > 0 {
            let height = cells.len();
            Ok(Self { cells, width, height })
        } else {
            Err(GridParseError::Empty)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::Peekable;

    #[derive(Debug, PartialEq, Eq, Copy, Clone)]
    struct Cell(u32);
    impl TryFrom<char> for Cell {
        type Error = ();
        fn try_from(ch: char) -> Result<Self, Self::Error> {
            ch.to_digit(10).map(Self).ok_or(())
        }
    }

    #[test]
    fn vec2d_from_str_ok() {
        let s = "123\n456\n789";
        let grid: Vec2D<Cell> = s.parse().unwrap();

        let (width, height) = grid.dim();
        assert_eq!(3, width);
        assert_eq!(3, height);

        for row in 1..=height {
            for col in 1..=width {
                let cell = Cell((3*(row-1) + col).try_into().unwrap());
                let point = Point(row, col);
                assert_eq!(Some(cell), grid.get(point).copied());
            }
        }
    }

    #[test]
    fn vec2d_from_str_empty() {
        let s = "\n\n\n";
        assert_eq!(Err(GridParseError::Empty), s.parse::<Vec2D<Cell>>());
    }

    #[test]
    fn vec2d_from_str_nonrect() {
        let s = "1\n23\n45";
        assert_eq!(Err(GridParseError::NonRect), s.parse::<Vec2D<Cell>>());
    }
}
