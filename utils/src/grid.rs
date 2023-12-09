use super::{PeekFrom, Point};

use std::str::{Chars, FromStr};

pub trait Grid<T> {
    fn dim(&self) -> (usize, usize);
    fn get(&self, point: Point<usize>) -> Option<&T>;

    fn neighbors(&self, point: Point<usize>) -> Option<Vec<&T>> {
        let (w, h) = self.dim();
        if point.0 >= h || point.1 >= w {
            return None;
        }

        Some(match point {
            // corners
            Point(0, 0) => vec![
                self.get(Point(0, 1)).unwrap(),
                self.get(Point(1, 0)).unwrap(),
                self.get(Point(1, 1)).unwrap(),
            ],
            Point(0, col) if col == w - 1 => vec![
                self.get(Point(0, w - 2)).unwrap(),
                self.get(Point(1, w - 1)).unwrap(),
                self.get(Point(1, w - 2)).unwrap(),
            ],
            Point(row, 0) if row == h - 1 => vec![
                self.get(Point(h - 2, 0)).unwrap(),
                self.get(Point(h - 1, 1)).unwrap(),
                self.get(Point(h - 2, 1)).unwrap(),
            ],
            Point(row, col) if row == h - 1 && col == w - 1 => vec![
                self.get(Point(h - 1, w - 2)).unwrap(),
                self.get(Point(h - 2, w - 1)).unwrap(),
                self.get(Point(h - 2, w - 2)).unwrap(),
            ],

            // edges
            Point(0, col) => vec![
                self.get(Point(0, col - 1)).unwrap(),
                self.get(Point(0, col + 1)).unwrap(),
                self.get(Point(1, col - 1)).unwrap(),
                self.get(Point(1, col    )).unwrap(),
                self.get(Point(1, col + 1)).unwrap(),
            ],
            Point(row, 0) => vec![
                self.get(Point(row - 1, 0)).unwrap(),
                self.get(Point(row + 1, 0)).unwrap(),
                self.get(Point(row - 1, 1)).unwrap(),
                self.get(Point(row,     1)).unwrap(),
                self.get(Point(row + 1, 1)).unwrap(),
            ],
            Point(row, col) if row == h - 1 => vec![
                self.get(Point(row,     col - 1)).unwrap(),
                self.get(Point(row - 1, col - 1)).unwrap(),
                self.get(Point(row - 1, col    )).unwrap(),
                self.get(Point(row - 1, col + 1)).unwrap(),
                self.get(Point(row,     col + 1)).unwrap(),
            ],
            Point(row, col) if col == w - 1 => vec![
                self.get(Point(row - 1, col    )).unwrap(),
                self.get(Point(row - 1, col - 1)).unwrap(),
                self.get(Point(row,     col - 1)).unwrap(),
                self.get(Point(row + 1, col - 1)).unwrap(),
                self.get(Point(row + 1, col    )).unwrap(),
            ],

            // interior
            Point(row, col) => vec![
                self.get(Point(row - 1, col - 1)).unwrap(),
                self.get(Point(row - 1, col    )).unwrap(),
                self.get(Point(row - 1, col + 1)).unwrap(),
                self.get(Point(row    , col - 1)).unwrap(),
                self.get(Point(row    , col + 1)).unwrap(),
                self.get(Point(row + 1, col - 1)).unwrap(),
                self.get(Point(row + 1, col    )).unwrap(),
                self.get(Point(row + 1, col + 1)).unwrap(),
            ],
        })
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
}

impl<T> Grid<T> for Vec2D<T> {
    fn dim(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn get(&self, point: Point<usize>) -> Option<&T> {
        self.cells.get(point.0)?.get(point.1)
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

        for row in 0..height {
            for col in 0..width {
                let cell = Cell((3*row + col + 1).try_into().unwrap());
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
