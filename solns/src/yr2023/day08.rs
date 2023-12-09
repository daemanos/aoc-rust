use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::Ordering;

use crate::Soln;
use utils::math;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = u64;

    fn part1(input: &str) -> Self::Answer {
        let (steps, nodes) = parse(input);

        let mut node = nodes.get("AAA").unwrap();
        for (n, step) in steps.chars().cycle().enumerate() {
            if node.label == "ZZZ" {
                return n.try_into().unwrap();
            } else {
                match step {
                    'L' => node = nodes.get(&node.left).unwrap(),
                    'R' => node = nodes.get(&node.right).unwrap(),
                    _ => panic!(),
                };
            }
        }

        0
    }

    fn part2(input: &str) -> Self::Answer {
        let (steps, nodes) = parse(input);

        let mut cycles = HashMap::new();
        for start in nodes.values() {
            let start_label = start.label.clone();
            let mut node = start;
            for (n, step) in steps.chars().cycle().enumerate() {
                let next = match step {
                    'L' => nodes.get(&node.left).unwrap(),
                    'R' => nodes.get(&node.right).unwrap(),
                    _ => panic!(),
                };

                if next.label.ends_with('Z') {
                    let end_label = next.label.clone();
                    let steps = (n + 1).try_into().unwrap();
                    cycles.insert(start_label, Cycle { end_label, steps });
                    break;
                } else if next == node {
                    break;
                } else {
                    node = next;
                }
            }
        }

        // shamelessly taking advantage of the fact that each A->Z cycle in the input is the
        // same length as the corresponding Z->Z self-cycle
        cycles.iter()
            .filter(|(start, _)| start.ends_with('Z'))
            .map(|(_, Cycle {steps, ..})| steps)
            .fold(1, |a, &b| math::lcm(a, b))
    }
}

fn parse(input: &str) -> (String, HashMap<String, Node>) {
    let mut lines = input.lines();

    let steps = lines.next().unwrap().trim();
    lines.next();

    let mut nodes = HashMap::new();
    for line in lines {
        let node: Node = line.trim().parse().unwrap();
        let label = node.label.clone();
        nodes.insert(label, node);
    }

    (steps.into(), nodes)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cycle {
    steps: u64,
    end_label: Label,
}

impl Ord for Cycle {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for Cycle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

type Label = String;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    label: Label,
    left: Label,
    right: Label,
}

impl FromStr for Node {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, succs) = s.split_once(" = ").unwrap();
        let (left, right) = succs
            .strip_prefix('(').unwrap()
            .strip_suffix(')').unwrap()
            .split_once(", ").unwrap();

        Ok(Self {
            label: label.to_string(),
            left: left.to_string(),
            right: right.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn part1() {
        //assert_eq!((), Puzzle::part1(""));
    }

    #[test]
    fn part2() {
        let input =
            "LR

             11A = (11B, XXX)
             11B = (XXX, 11Z)
             11Z = (11B, XXX)
             22A = (22B, XXX)
             22B = (22C, 22C)
             22C = (22Z, 22Z)
             22Z = (22B, 22B)
             XXX = (XXX, XXX)";
        assert_eq!(6, Puzzle::part2(input));
    }
}
