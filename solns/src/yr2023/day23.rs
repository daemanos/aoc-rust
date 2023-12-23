use crate::Soln;
use utils::prelude::*;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = usize;

    fn part1(input: &str) -> Self::Answer {
        let grid: Vec2D<Cell> = input.parse().unwrap();
        let Dim(_, w) = grid.dim();

        let mut lens = HashMap::new();
        let mut trails = BinaryHeap::new();
        for col in 1..=w {
            let pt = Point(1, col);
            if grid[pt] == Path {
                trails.push(Trail(pt, 0));
                lens.insert(pt, 0);
            }
        }

        let mut max = 0;
        while let Some(Trail(pt, len)) = trails.pop() {
            // TODO would have to abandon dead ends somehow
            match grid[pt] {
                Path => todo!(),
                Slope(dir) => todo!(),
                Forest => panic!(),
            }
        }

        max
    }

    fn part2(_input: &str) -> Self::Answer {
        unsolved!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Trail(IdxPoint, usize);

impl Ord for Trail {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.1.cmp(&rhs.1)
    }
}

impl PartialOrd for Trail {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

//struct Trails {
//    grid: Vec2D<Cell>,
//    lens: HashMap<IdxPoint, usize>,
//}
//
//impl Trails {
//    fn parse(input: &str) -> Self {
//        let grid = input.parse().unwrap();
//        let seen = HashSet::new();
//        Self{grid, seen}
//    }
//}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Path,
    Forest,
    Slope(Direction),
}
use Cell::*;

impl TryFrom<char> for Cell {
    type Error = ();
    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            '.' => Ok(Path),
            '#' => Ok(Forest),
            '>' => Ok(Slope(E)),
            '<' => Ok(Slope(W)),
            '^' => Ok(Slope(N)),
            'v' => Ok(Slope(S)),
            _ => Err(())
        }
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
