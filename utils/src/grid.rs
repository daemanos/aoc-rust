use std::ops::{Index, IndexMut, Deref};
use std::str::{Chars, FromStr};
use std::hash::{Hash, Hasher};
use std::fmt;

use crate::convert::PeekFrom;
use crate::geom::{Direction, Point};

use Direction::*;

pub type IdxPoint = Point<usize>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Dim(pub usize, pub usize);

impl Dim {
    pub fn in_bounds(&self, point: IdxPoint) -> bool {
        point.0 > 0 && point.0 <= self.0 && point.1 > 0 && point.1 <= self.1
    }
}

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
        if self.dim().in_bounds(point) {
            Some(&self[point])
        } else {
            None
        }
    }

    fn iter<W: Walk>(&self, walk: W) -> Iter<T, W> where Self: Sized {
        Iter::new(self, walk)
    }

    fn walk(&self, dir0: Direction, dir1: Direction) -> Iter<T, DirectedWalk>
        where Self: Sized,
    {
        let dim = self.dim();
        let heading = (dir0, dir1);
        Iter::new(self, DirectedWalk { dim, heading })
    }

    fn walk_rows(&self) -> Iter<T, DirectedWalk> where Self: Sized {
        self.walk(E, S)
    }

    fn walk_cols(&self) -> Iter<T, DirectedWalk> where Self: Sized {
        self.walk(S, E)
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

pub trait Walk {
    fn start(&self) -> IdxPoint;
    fn succ(&self, curr: IdxPoint) -> Option<IdxPoint>;
}

pub struct DirectedWalk {
    dim: Dim,
    heading: (Direction, Direction),
}

impl Walk for DirectedWalk {
    fn start(&self) -> IdxPoint {
        let Dim(h, w) = self.dim;
        match self.heading {
            (S, E) | (E, S) => Point(0,     0),
            (S, W) | (W, S) => Point(0,     w + 1),
            (N, E) | (E, N) => Point(h + 1, 0),
            (N, W) | (W, N) => Point(h + 1, w + 1),
            _ => panic!("invalid heading: {:?}", self.heading),
        }
    }

    fn succ(&self, curr: IdxPoint) -> Option<IdxPoint> {
        let next = curr + self.heading.0;
        if self.dim.in_bounds(next) {
            Some(next)
        } else {
            let Point(row, col) = curr;
            let Dim(h, w) = self.dim;
            let next = match self.heading {
                (S, E) => Point(1, col + 1),
                (S, W) => Point(1, col - 1),
                (N, E) => Point(h, col + 1),
                (N, W) => Point(h, col - 1),
                (E, S) => Point(row + 1, 1),
                (E, N) => Point(row - 1, 1),
                (W, S) => Point(row + 1, w),
                (W, N) => Point(row - 1, w),
                _ => panic!("invalid heading: {:?}", self.heading),
            };

            if self.dim.in_bounds(next) {
                Some(next)
            } else {
                None
            }
        }
    }
}

pub struct Iter<'a, T, W: Walk> {
    curr: IdxPoint,
    walk: W,
    grid: &'a dyn Grid<T>,
}

impl<'a, T, W: Walk> Iter<'a, T, W> {
    fn new(grid: &'a dyn Grid<T>, walk: W) -> Self {
        let curr = walk.start();
        Self { curr, walk, grid }
    }
}

impl<'a, T, W: Walk> Iterator for Iter<'a, T, W> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.walk.succ(self.curr).map(|succ| {
            self.curr = succ;
            &self.grid[succ]
        })
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

    pub fn from_rows<I: IntoIterator<Item = Vec<T>>>(rows: I) -> Self {
        let cells: Vec<Vec<_>> = rows.into_iter().collect();
        let height = cells.len();
        let width = cells.iter().map(Vec::len).reduce(|old, new| {
            if old != new {
                panic!("non-rectangular argument to from_rows");
            }
            new
        }).unwrap_or(0);

        Self { cells, width, height }
    }

    pub fn from_cols<I: IntoIterator<Item = Vec<T>>>(cols: I) -> Self {
        let mut cols = cols.into_iter();
        if let Some(col) = cols.next() {
            let height = col.len();
            let mut width = 1;

            let mut cells: Vec<Vec<T>> = col.into_iter()
                .map(|x| vec![x])
                .collect();

            for col in cols {
                for (row, x) in col.into_iter().enumerate() {
                    match cells.get_mut(row) {
                        Some(row) => row.push(x),
                        None => panic!("non-rectangular argument to from_cols"),
                    }
                }

                width += 1;
            }

            Self { cells, width, height }
        } else {
            Self::new()
        }
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

impl<T> fmt::Display for Vec2D<T>
where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 1..=self.height {
            for col in 1..=self.width {
                write!(f, "{}", &self[Point(row, col)])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
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
    fn grid_walk() {
        let rows = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let grid = Vec2D::from_rows(rows.into_iter());

        let xs: Vec<_> = grid.walk_rows().copied().collect();
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], xs);

        let xs: Vec<_> = grid.walk_cols().copied().collect();
        assert_eq!(vec![1, 4, 7, 2, 5, 8, 3, 6, 9], xs);
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
