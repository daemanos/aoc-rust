use std::ops::{Index, IndexMut, Deref};
use std::str::{Chars, FromStr};
use std::hash::{Hash, Hasher};

use super::{PeekFrom, Point};

pub type IdxPoint = Point<usize>;

pub struct Dim(pub usize, pub usize);

/// A two-dimensional grid structure with 1-based indexing.
///
/// # Note on indexing
///
/// Grid types use 1-based indexing so that the notion of a "neighbor" can
/// be naturally implemented with unsigned indices.
pub trait Grid<T> : Index<IdxPoint, Output = T> {
    fn dim(&self) -> Dim;

    fn in_bounds(&self, point: IdxPoint) -> bool {
        let Dim(h, w) = self.dim();
        point.0 > 0 && point.0 <= h && point.1 > 0 && point.1 <= w
    }

    fn get(&self, point: IdxPoint) -> Option<&T> {
        if self.in_bounds(point) {
            Some(&self[point])
        } else {
            None
        }
    }

    fn iter(&self) -> Iter<T> where Self: Sized {
        Iter::new(self)
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
    }
}

pub struct Iter<'a, T> {
    curr: IdxPoint,
    dim: Dim,
    grid: &'a dyn Grid<T>,
}

impl<'a, T> Iter<'a, T> {
    fn new(grid: &'a dyn Grid<T>) -> Self {
        let curr = Point(1, 1);
        let dim = grid.dim();

        Self { curr, dim, grid }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let Point(row, col) = self.curr;
        let Dim(h, w) = self.dim;

        if row > h && col > w {
            None
        } else {
            let res = &self.grid[self.curr];

            self.curr = if col == w {
                Point(row + 1, 1)
            } else {
                Point(row, col + 1)
            };

            Some(res)
        }
    }
}

pub struct Cell<'a, T> {
    point: IdxPoint,
    grid : &'a dyn Grid<T>,
}

//impl<'a, T> Cell<'a, T> {
//    fn neighbors(&self) -> Vec<Self> {
//
//    }
//}

impl<'a, T> Deref for Cell<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.grid[self.point]
    }
}

#[derive(Debug, PartialEq)]
pub enum GridParseError {
    Empty,
    NonRect,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Vec2D<T> {
    cells: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Vec2D<T> {
    pub fn new() -> Self {
        Self {
            cells: vec![],
            width: 0,
            height: 0,
        }
    }

    pub fn from_rows<I: Iterator<Item = Vec<T>>>(rows: I) -> Self {
        rows.fold(Self::new(), |mut acc, row| {
            acc.push_row(row);
            acc
        })
    }

    pub fn from_cols<I: Iterator<Item = Vec<T>>>(cols: I) -> Self {
        cols.fold(Self::new(), |mut acc, col| {
            acc.push_col(col);
            acc
        })
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        if row.len() == self.width {
            self.cells.push(row);
            self.height += 1;
        } else {
            panic!("argument to push_row must have the same length as the grid width");
        }
    }

    pub fn push_col(&mut self, col: Vec<T>) {
        if col.len() == self.height {
            self.cells.iter_mut().zip(col)
                .for_each(|(row, cell)| row.push(cell));
            self.width += 1;
        } else {
            panic!("argument to push_col must have the same length as the grid height");
        }
    }

    pub fn row(&self, row: usize) -> Option<&[T]> {
        if row > 0 && row <= self.height {
            Some(self.cells[row - 1].as_slice())
        } else {
            None
        }
    }

    pub fn col(&self, col: usize) -> Option<Vec<&T>> {
        if col > 0 && col <= self.width {
            Some(self.cells.iter().map(|row| &row[col - 1]).collect())
        } else {
            None
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.cells.iter().map(Vec::as_slice)
    }

    pub fn cols(&self) -> impl Iterator<Item = Vec<&T>> {
        (0..self.width)
            .map(|col| self.cells.iter()
                .map(|row| &row[col]).collect())
    }
}

impl<T> Hash for Vec2D<T>
where T: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        for row in self.rows() {
            row.hash(state);
        }
    }
}

impl<T> Index<IdxPoint> for Vec2D<T> {
    type Output = T;
    fn index(&self, index: IdxPoint) -> &Self::Output {
        &self.cells[index.0 - 1][index.1 - 1]
    }
}

impl<T> IndexMut<IdxPoint> for Vec2D<T> {
    fn index_mut(&mut self, index: IdxPoint) -> &mut Self::Output {
        &mut self.cells[index.0 - 1][index.1 - 1]
    }
}

impl<T> Grid<T> for Vec2D<T> {
    fn dim(&self) -> Dim {
        Dim(self.height, self.width)
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

        let Dim(width, height) = grid.dim();
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
