use crate::Soln;
use utils::prelude::*;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = i64;

    fn part1(input: &str) -> Self::Answer {
        let mut pos = Point(1, 1);
        let mut grid = HashSet::from([pos]);

        let mut rows = (i32::MAX, i32::MIN);
        let mut cols = (i32::MAX, i32::MIN);
        for line in input.lines() {
            let instr: Instr = line.trim().parse().unwrap();
            for _ in 0..instr.steps {
                pos = pos + instr.dir;
                grid.insert(pos);

                rows = (cmp::min(rows.0, pos.0), cmp::max(rows.1, pos.0));
                cols = (cmp::min(cols.0, pos.1), cmp::max(cols.1, pos.1));
            }
        }

        let mut fill = 0;
        for row in rows.0..=rows.1 {
            let mut crossings = 0;
            for col in cols.0..=cols.1 {
                let dug = grid.contains(&Point(row, col));
                let is_crossing = grid.contains(&Point(row - 1, col));

                if dug && is_crossing {
                    crossings += 1;
                }

                if dug || crossings % 2 == 1 {
                    fill += 1;
                }
            }
        }

        fill
    }

    fn part2(input: &str) -> Self::Answer {
        let mut corners = vec![Point(0, 0)];
        for line in input.lines() {
            let offset = parse_part2(line);
            corners.push(*corners.last().unwrap() + offset);
        }
        let n = corners.len();

        // normalize points so that rows all are >= 0
        let min_row = corners.iter().map(|p| p.0).min().unwrap();
        for corner in corners.iter_mut() {
            corner.0 -= min_row;
        }

        // note: the first and last corners in a valid input will always
        // be (0, 0), which are irrelevant to the calculation because they
        // will always contribute 0 area to any pair, so I don't bother
        // calculating what they should be expanded to

        // calculate directions from/to each point
        let mut dirs = vec![];
        for i in 1..n-1 {
            let p = corners[i-1];
            let q = corners[i];
            let r = corners[(i+1)%n];

            let from = (q-p).direction().unwrap();
            let to = (r-q).direction().unwrap();
            dirs.push((from, to));
        }

        // expand points into cells
        for i in 1..n-1 {
            let p = corners[i-1];
            let q = corners[i];
            let (from, to) = dirs[i-1];

            let (cand1, cand2) = match (from, to) {
                (E, S) | (S, E) | (W, N) | (N, W) =>
                    (q + Point(1,0), q + Point(0,1)),
                (W, S) | (S, W) | (E, N) | (N, E) => (q, q + Point(1,1)),
                _ => panic!(),
            };

            if cand1.colinear(&p) && !cand2.colinear(&p) {
                corners[i] = cand1;
            } else if !cand1.colinear(&p) && cand2.colinear(&p) {
                corners[i] = cand2;
            } else {
                panic!("ambiguous corner");
            }
        }

        let mut area = 0;
        for i in 0..n-1 {
            let p = corners[i];
            let q = corners[i+1];
            if p.1 != q.1 {
                area += (p.0 + q.0) * (p.1 - q.1);
            }
        }

        area.abs() / 2
    }
}

fn parse_part2(line: &str) -> Point<i64> {
    let (_, line) = line.split_once('#').unwrap();
    let ds: Vec<i64> = line.strip_suffix(')').unwrap().chars()
        .map(|ch| ch.to_digit(16).unwrap() as i64)
        .collect();

    let steps = ds[0..5].iter().rev().enumerate()
        .fold(0, |n, (i, d)| n + 16_i64.pow(i as u32)*d);

    match ds[5] {
        0 => Point(0, steps),
        1 => Point(steps, 0),
        2 => Point(0, -steps),
        3 => Point(-steps, 0),
        _ => panic!(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Instr {
    dir: Direction,
    steps: usize,
}

impl FromStr for Instr {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let dir = match parts[0] {
            "R" => E,
            "L" => W,
            "U" => N,
            "D" => S,
            _ => panic!(),
        };
        let steps = parts[1].parse().unwrap();

        Ok(Self { dir, steps })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn part1() {
        assert_eq!(62, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(952408144115, Puzzle::part2(&INPUT));
    }
}
