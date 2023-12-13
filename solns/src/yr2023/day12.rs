use std::cmp;
use std::fmt;
use std::iter;
use std::str::FromStr;

use crate::Soln;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = usize;

    fn part1(input: &str) -> Self::Answer {
        let records: Vec<Record> = input.lines()
            .map(|line| line.parse().unwrap())
            .collect();

        records.iter().map(record_arrs_dp).sum()
    }

    fn part2(input: &str) -> Self::Answer {
        let mut records: Vec<Record> = input.lines()
            .map(|line| line.parse().unwrap())
            .collect();

        records.iter_mut()
            .map(Record::expand)
            .map(|record| record_arrs(&record))
            //.map(|record| record_arrs(record.expand()))
            .sum()
    }
}

fn record_arrs(record: &Record) -> usize {
    let len = record.springs.len();

    let mut arrs = 0;
    let mut front = vec![(0, record.springs.clone())];

    while let Some((i, mut springs)) = front.pop() {
        if check(&springs, &record.group_lens) {
            if i == len {
                arrs += 1;
            } else {
                let i = if springs[i] == Spring::Unk {
                    i
                } else {
                    let j = (i+1..len).find(|&i| springs[i] == Spring::Unk);

                    if let Some(j) = j {
                        j
                    } else {
                        // reached end of record
                        arrs += 1;
                        continue;
                    }
                };

                let mut springs_dmg = springs.clone();
                springs_dmg[i] = Spring::Dmg;
                front.push((i + 1, springs_dmg));

                springs[i] = Spring::Ok;
                front.push((i + 1, springs));
            }
        }
    }

    println!("{arrs}");
    arrs
}

fn check(springs: &[Spring], lens: &[usize]) -> bool {
    let mut groups = vec![];
    let mut group = 0;
    let mut seen_unk = false;
    for &spring in springs {
        match spring {
            Spring::Dmg => group += 1,
            Spring::Ok => if group > 0 {
                groups.push(group);
                group = 0;
            },
            Spring::Unk => {
                seen_unk = true;
                break;
            }
        }
    }
    if group > 0 {
        groups.push(group);
    }

    let n = groups.len();
    (seen_unk || n == lens.len()) &&
        groups.iter().enumerate().zip(lens.iter()).all(|((i, &a), &e)| {
            if seen_unk && i == n - 1 {
                // last group can still grow if there are remaining unknowns
                a <= e
            } else {
                a == e
            }
        })
}

fn record_arrs_dp(record: &Record) -> usize {
    let s = record.springs.len();
    let n = record.group_lens.len();

    // arrs[j][i] = # of consistent arr's up to group j and character i
    // that end at i (NOTE: one-based indexing)
    let mut arrs = vec![vec![0; s+1]; n+1];

    let mut l_tot = 0;
    for j in 1..=n {
        let l = record.group_lens[j-1];
        l_tot += l;

        let i0 = cmp::max(l_tot,
            (1..=s).find(|&i| arrs[j-1][i] > 0)
                .map(|i| i + l + 1)
                .unwrap_or(1));

        // we can start at l_tot because fewer characters couldn't work
        for i in i0..=s {
            if j == n && i+1 < s {
                if record.springs[i+1..].iter().any(|&s| s == Spring::Dmg) {
                    // in the final row, there cannot be any # after this point
                    continue;
                }
            }

            if record.springs[i-1] != Spring::Ok {
                // spring at this index is # or ?
                if i == s || record.springs[i] != Spring::Dmg {
                    // right edge of window is the edge of the grid/?/.
                    if i - l == 0 || record.springs[i-l-1] != Spring::Dmg {
                        // left edge of window is the edge of the grid/?/.
                        let window = &record.springs[i-l..i];
                        let (ok, _, _) = classify_window(&window);
                        if ok == 0 {
                            // no spring in window is .
                            let prev = if i >= l + 1 {
                                // start summing at the most recent #
                                // entries before that are no longer possible
                                // as the known # must occur in the previous
                                // group
                                let start = (1..i-l).rev()
                                    .find(|i| record.springs[i-1] == Spring::Dmg)
                                    .unwrap_or(1);

                                arrs[j-1][start..i-l].iter().sum()
                            } else {
                                0
                            };

                            // if this is the first group or we can add ourself
                            // to any previous arrangements
                            if j == 1 || prev > 0 {
                                arrs[j][i] = cmp::max(1, prev);
                            }
                        }
                    }
                }
            }

            //if i - l > 0 && record.springs[i-l-1] == Spring::Dmg && arrs[j-1][i] == 0 {
            //    // once we have seen a known # on the left edge, it must be
            //    // part of this group
            //    break;
            //}
        }
    }

    let mut buf = String::new();
    buf.push_str("   ");
    for i in 0..s {
        buf.push_str(format!(" {} ", record.springs[i]).as_str());
    }
    buf.push_str("\n   ");
    for i in 1..=s {
        buf.push_str(format!("{:2} ", i).as_str());
    }
    buf.push('\n');
    for j in 1..=n {
        buf.push_str(format!("{:2} ", record.group_lens[j-1]).as_str());
        for i in 1..=s {
            buf.push_str(format!("{:2} ", arrs[j][i]).as_str());
        }
        buf.push('\n');
    }
    println!("\n{buf}");

    let res = arrs[n].iter().sum();
    println!("{res}");
    res
}

fn classify_window(window: &[Spring]) -> (usize, usize, usize) {
    window.iter().fold((0, 0, 0), |(ok, dmg, unk), s| {
        match s {
            Spring::Ok => (ok + 1, dmg, unk),
            Spring::Dmg => (ok, dmg + 1, unk),
            Spring::Unk => (ok, dmg, unk + 1),
        }
    })
}

struct Record {
    springs: Vec<Spring>,
    group_lens: Vec<usize>,
}

impl Record {
    fn expand(&mut self) -> Self {
        let springs = iter::repeat(self.springs.clone())
            .take(5)
            .intersperse(vec![Spring::Unk])
            .fold(Vec::new(), |mut acc, mut e| {
                acc.append(&mut e);
                acc
            });
        let group_lens = iter::repeat(self.group_lens.clone())
            .take(5)
            .fold(Vec::new(), |mut acc, mut e| {
                acc.append(&mut e);
                acc
            });

        Self { springs, group_lens }
    }
}

impl FromStr for Record {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, groups) = s.trim().split_once(' ').unwrap();
        let springs = springs.chars().map(Spring::from).collect();
        let group_lens = groups.split(',').map(|d| d.parse().unwrap())
            .collect();

        Ok(Self { springs, group_lens })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Ok,
    Dmg,
    Unk,
}

impl From<char> for Spring {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Spring::Ok,
            '#' => Spring::Dmg,
            '?' => Spring::Unk,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Spring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Spring::Ok => write!(f, "."),
            Spring::Dmg => write!(f, "#"),
            Spring::Unk => write!(f, "?"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1, Puzzle::part1("???.### 1,1,3"));
        assert_eq!(4, Puzzle::part1(".??..??...?##. 1,1,3"));
        assert_eq!(1, Puzzle::part1("?#?#?#?#?#?#?#? 1,3,1,6"));
        assert_eq!(1, Puzzle::part1("????.#...#... 4,1,1"));
        assert_eq!(4, Puzzle::part1("????.######..#####. 1,6,5"));
        assert_eq!(10, Puzzle::part1("?###???????? 3,2,1"));

        assert_eq!(19, Puzzle::part1("????#?#???????#????? 6,5"));

        assert_eq!(1, Puzzle::part1(".##.?#??.#.?# 2,1,1,1"));
        assert_eq!(0, Puzzle::part1("###.### 3"));

        assert_eq!(20, Puzzle::part1("?#???????.#??? 2,1,2,1"));
        assert_eq!(6, Puzzle::part1("?.#??.????..?.??? 1,2"));
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(0, Puzzle::part2(""));
    }
}
