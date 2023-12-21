use crate::Soln;
use utils::prelude::*;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = usize;

    fn part1(input: &str) -> Self::Answer {
        let grid: Vec2D<Cell> = input.parse().unwrap();
        reachable_plots(&grid, 64)
    }

    fn part2(_input: &str) -> Self::Answer {
        unsolved!()
    }
}

fn reachable_plots(grid: &Vec2D<Cell>, steps: usize) -> usize {
    let Dim(h, w) = grid.dim();

    let mut front = HashSet::new();
    for row in 1..h {
        for col in 1..w {
            if grid[Point(row, col)] == Start {
                front.insert(Point(row, col));
                break;
            }
        }
    }

    for _ in 0..steps {
        let front2 = front.drain()
            .flat_map(|pt|
                pt.ortho_neighbors().filter(|&pt|
                    grid.in_bounds(pt) &&
                    grid[pt] != Rock))
            .collect();
        front = front2;
    }

    front.len()
}

fn reachable_plots_inf(grid: &Vec2D<Cell>, steps: usize) -> usize {
    let Dim(h, w) = grid.dim();

    let mut front = HashMap::new();
    for row in 1..h {
        for col in 1..w {
            if grid[Point(row, col)] == Start {
                front.insert(Point(row, col), 1);
                break;
            }
        }
    }

    //let mut seen = HashMap::new();
    //for _ in 0..steps {
    //    let front2 = front.drain()
    //        .flat_map(|(pt, mult)| {
    //            pt.ortho_neighbors()
    //                .filter_map(|pt| {
    //                    if grid.in_bounds(pt) {
    //                        if grid[pt] != Rock {
    //                            Some()
    //                        }
    //                    }
    //                })
    //        }).collect();
    //    front = front2;
    //}

    front.values().sum()
}

#[derive(Debug, Charnum)]
#[repr(u8)]
enum Cell {
    Ground = b'.',
    Rock = b'#',
    Start = b'S',
}
use Cell::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "...........
                     .....###.#.
                     .###.##..#.
                     ..#.#...#..
                     ....#.#....
                     .##..S####.
                     .##..#...#.
                     .......##..
                     .##.#.####.
                     .##..##.##.
                     ...........";
        let grid: Vec2D<Cell> = input.parse().unwrap();
        assert_eq!(4, reachable_plots(&grid, 2));
    }

    #[test]
    fn part2() {
        //assert_eq!((), Puzzle::part2(""));
    }
}
