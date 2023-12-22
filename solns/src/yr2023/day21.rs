use std::mem;

use crate::Soln;
use utils::prelude::*;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = usize;

    fn part1(input: &str) -> Self::Answer {
        let grid: Vec2D<Cell> = input.parse().unwrap();
        reachable_plots(&grid, 64)
    }

    fn part2(input: &str) -> Self::Answer {
        let grid: Vec2D<Cell> = input.parse().unwrap();
        reachable_plots_inf(&grid, 26501365)
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
    let dim@Dim(h, w) = grid.dim();

    let mut front = HashSet::new();
    for row in 1..h {
        for col in 1..w {
            if grid[Point(row, col)] == Start {
                front.insert(Point(row as isize, col as isize));
                break;
            }
        }
    }

    let mut seen = front.clone();
    let mut front2 = HashSet::new();
    let mut reachable = 0;
    //let mut reachable = HashSet::new();
    for s in 0..=steps {
        for pt in front.drain() {
            if (steps - s) % 2 == 0 {
                //println!("{pt} in {s} steps -> reachable");
                reachable += 1;
                //dbg!(reachable.insert(dbg!(pt)));
            } else {
                //println!("{pt} in {s} steps -> unreachable");
            }

            for pt in pt.ortho_neighbors() {
                if !seen.contains(&pt) {
                    seen.insert(pt);
                    let pt_inf = point_inf(pt, dim);
                    //println!("moving to {}", pt_inf);
                    if grid[pt_inf] != Rock {
                        front2.insert(pt);
                    }
                }
            }
        }

        mem::swap(&mut front, &mut front2);
    }

    //let mut buf = String::new();
    //for row in 1..=h {
    //    for col in 1..=w {
    //        let pt = Point(row, col);
    //        if reachable.contains(&pt) {
    //            buf.push('O');
    //        } else {
    //            buf.push(grid[pt].into());
    //        }
    //    }
    //    buf.push('\n');
    //}
    //println!("{buf}");

    reachable
}

fn point_inf(pt: Point<isize>, dim: Dim) -> IdxPoint {
    let h = dim.0 as isize;
    let w = dim.1 as isize;

    let row = match pt.0.rem_euclid(h) {
        0 => h,
        r => r,
    };
    let col = match pt.1.rem_euclid(w) {
        0 => w,
        c => c,
    };

    Point(row as usize, col as usize)
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

    static INPUT: &str =
        "...........
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


    #[test]
    fn part1() {
        let grid: Vec2D<Cell> = INPUT.parse().unwrap();
        assert_eq!(4, reachable_plots(&grid, 2));
    }

    #[test]
    fn part2() {
        let grid: Vec2D<Cell> = INPUT.parse().unwrap();
        assert_eq!(16, reachable_plots_inf(&grid, 6));
        assert_eq!(50, reachable_plots_inf(&grid, 10));
        assert_eq!(1594, reachable_plots_inf(&grid, 50));
        assert_eq!(6536, reachable_plots_inf(&grid, 100));
    }
}
