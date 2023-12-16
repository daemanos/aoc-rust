use std::str::Lines;

use crate::Soln;
use utils::prelude::*;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = u64;

    fn part1(input: &str) -> Self::Answer {
        let mut lines = input.lines();

        let seeds: Vec<u64> = lines.next().unwrap().trim()
            .strip_prefix("seeds: ").unwrap()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();

        lines.next().unwrap();
        let mut maps = Vec::new();
        while let Some(map) = Map::parse(&mut lines) {
            maps.push(map);
        }

        let locs: Vec<u64> = maps.iter()
            .fold(seeds, |source, map| {
                source.iter().map(|val| map.convert(*val)).collect()
            });

        locs.iter().min().copied().unwrap()
    }

    fn part2(input: &str) -> Self::Answer {
        let mut lines = input.lines();

        let seeds: Vec<u64> = lines.next().unwrap().trim()
            .strip_prefix("seeds: ").unwrap()
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect();
        let seeds: Vec<Interval<_>> = seeds.chunks(2)
            .map(|chunk| {
                match chunk {
                    &[s, l] => Interval(s, s + l),
                    _ => panic!(),
                }
            }).collect();

        lines.next().unwrap();
        let mut maps = Vec::new();
        while let Some(map) = Map::parse(&mut lines) {
            maps.push(map);
        }

        let locs: Vec<Interval<_>> = maps.iter()
            .fold(seeds, |is, map| map.convert_intervals(is));

        locs.iter().min_by_key(|i| i.0).unwrap().0
    }
}

#[derive(Debug)]
struct Map(Vec<(u64, u64, u64)>);

impl Map {
    fn parse(lines: &mut Lines) -> Option<Self> {
        lines.next()?;

        let mut ranges = Vec::new();
        while let Some(range) = lines.next() {
            let range = range.trim();
            if range.is_empty() {
                break;
            }

            let parts: Vec<u64> = range.split(' ').map(|n| n.parse().unwrap()).collect();
            match parts.as_slice() {
                &[dest, src, len] => {
                    ranges.push((src, dest, len));
                }
                _ => panic!(),
            }
        }

        Some(Self(ranges))
    }

    fn convert(&self, val: u64) -> u64 {
        for (src, dest, len) in &self.0 {
            if val >= *src && val < *src + *len {
                return *dest + (val - *src);
            }
        }

        val
    }

    fn convert_intervals(&self, is: Vec<Interval<u64>>) -> Vec<Interval<u64>> {
        let mut res = Vec::new();

        let mut stack = is.clone();
        while let Some(i @ Interval(s, e)) = stack.pop() {
            let mut overlaps = 0;

            for (src, dest, len) in &self.0 {
                let src = Interval(*src, *src + *len);
                let dest = Interval(*dest, *dest + *len);

                if src.overlaps(i) {
                    overlaps += 1;
                } else {
                    continue;
                }

                let (s, e) = if s < src.0 && e <= src.1 {
                    // left overlap
                    stack.push(Interval(s, src.0));
                    (src.0, e)
                } else if s >= src.0 && e > src.1 {
                    // right overlap
                    stack.push(Interval(src.1, e));
                    (s, src.1)
                } else if s < src.0 && e > src.1 {
                    // superset
                    stack.push(Interval(s, src.0));
                    stack.push(Interval(src.1, e));
                    (src.0, src.1)
                } else {
                    // subset
                    (s, e)
                };

                res.push(Interval(dest.0 + (s - src.0), dest.0 + (e - src.0)));
            }

            if overlaps == 0 {
                res.push(i);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "seeds: 79 14 55 13

         seed-to-soil map:
         50 98 2
         52 50 48

         soil-to-fertilizer map:
         0 15 37
         37 52 2
         39 0 15

         fertilizer-to-water map:
         49 53 8
         0 11 42
         42 0 7
         57 7 4

         water-to-light map:
         88 18 7
         18 25 70

         light-to-temperature map:
         45 77 23
         81 45 19
         68 64 13

         temperature-to-humidity map:
         0 69 1
         1 0 69

         humidity-to-location map:
         60 56 37
         56 93 4";

    #[test]
    fn part1() {
        assert_eq!(35, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(46, Puzzle::part2(&INPUT));
    }
}
