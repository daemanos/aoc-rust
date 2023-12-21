use std::ops::Neg;

use crate::Soln;
use utils::prelude::*;

const START_LBL: &'static str = "broadcaster";
const END_LBL: &'static str = "rx";

const START_PULSE: (Pulse, &'static str, &'static str) = (Lo, "", START_LBL);

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = usize;

    fn part1(input: &str) -> Self::Answer {
        let ModuleGraph {mut modules, ..} = parse(input);

        let mut lo = 0;
        let mut hi = 0;
        let mut pulses = VecDeque::new();
        for _ in 0..1000 {
            pulses.push_back(START_PULSE);
            while let Some((pulse, from, to)) = pulses.pop_front() {
                match pulse {
                    Lo => lo += 1,
                    Hi => hi += 1,
                };

                if let Some(module) = modules.get_mut(to) {
                    if let Some(pulse) = module.kind.process(from, pulse) {
                        for dest in &module.dests {
                            pulses.push_back((pulse, to, dest));
                        }
                    }
                }
            }
        }

        lo * hi
    }

    fn part2(input: &str) -> Self::Answer {
        let ModuleGraph {modules, predss} = parse(input);

        let mut min_presses = HashMap::new();
        min_presses.insert((START_LBL, Lo), Some(1));
        min_presses.insert((START_LBL, Hi), None);

        let mut front: VecDeque<_> = modules.get(START_LBL).unwrap().dests
            .clone().into();

        while let Some(label) = front.pop_front() {
            let module = modules.get(label).unwrap();
        }

        min_presses[&(END_LBL, Lo)].unwrap()

        //graph.predss.get(END_LBL).unwrap().iter()
        //    .filter_map(|pred| min_presses(&graph, pred, Lo))
        //    .min()
        //    .unwrap()
    }
}

fn min_presses(
    graph: &ModuleGraph,
    label: &str,
    target: Pulse,
) -> Option<usize> {
    if let Some(module) = graph.modules.get(label) {
        if module.kind == Broadcast {
            match target {
                Lo => Some(1),
                Hi => None,
            }
        } else {
            graph.predss.get(label).and_then(|preds| {
                match module.kind {
                    FlipFlop(_, _) => {
                        min_presses_preds(graph, preds, Lo).map(|min| {
                            match target {
                                Lo => 2*min,
                                Hi => min,
                            }
                        })
                    },
                    Conjunction(_, _, _) => {
                        if target == Lo {
                            // all inputs must be high
                            preds.iter().fold(Some(1), |min, pred| {
                                min.and_then(|min| {
                                    min_presses(&graph, pred, Hi)
                                        .map(|min_pred| {
                                            math::lcm(min, min_pred)
                                        })
                                })
                            })
                        } else {
                            // one input must be low
                            min_presses_preds(graph, preds, Lo)
                        }
                    },
                    Broadcast => panic!("impossible"),
                }
            })
        }
    } else {
        None
    }
}

fn min_presses_preds(
    graph: &ModuleGraph,
    preds: &[&str],
    target: Pulse,
) -> Option<usize> {
    preds.iter()
        .fold(None, |min, pred| {
            let min_pred = min_presses(&graph, pred, target);
            match (min, min_pred) {
                (None, Some(m)) => Some(m),
                (Some(m), None) => Some(m),
                (Some(m1), Some(m2)) => Some(cmp::min(m1, m2)),
                _ => None,
            }
        })
}

#[derive(Debug)]
struct ModuleGraph<'a> {
    modules: HashMap<&'a str, Module<'a>>,
    predss: HashMap<&'a str, Vec<&'a str>>,
}

fn parse(input: &str) -> ModuleGraph {
    let mut modules: HashMap<&str, Module> = input.lines()
        .map(|line| {
            let module: Module = line.trim().into();
            let label = module.kind.label();
            (label, module)
        }).collect();

    let sigs: Vec<(&str, &str)> = modules.iter()
        .flat_map(move |(&label, module)|
            module.dests.iter().map(move |&dest| (label, dest)))
        .collect();

    let mut predss = HashMap::new();
    for (from, to) in sigs {
        if let Some(module) = modules.get_mut(to) {
            if let Conjunction(_, _, _) = module.kind {
                module.kind.process(from, Pulse::Lo);
            }
        }

        let preds = predss.entry(to).or_insert(Vec::new());
        preds.push(from);
    }

    ModuleGraph {modules, predss}
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Module<'a> {
    kind: ModuleKind<'a>,
    dests: Vec<&'a str>,
}

impl<'a> From<&'a str> for Module<'a> {
    fn from(s: &'a str) -> Self {
        let (kind, dests) = s.split_once(" -> ").unwrap();
        let kind = kind.into();
        let dests = dests.split(", ").collect();

        Self {kind, dests}
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ModuleKind<'a> {
    FlipFlop(&'a str, Pulse),
    Conjunction(&'a str, HashMap<&'a str, Pulse>, usize),
    Broadcast,
}
use ModuleKind::*;

impl<'a> ModuleKind<'a> {
    fn label(&self) -> &'a str {
        match self {
            Self::FlipFlop(lbl, _) => lbl,
            Self::Conjunction(lbl, _, _) => lbl,
            Self::Broadcast => START_LBL,
        }
    }

    //fn test(&self, from: &'a str, sig: Pulse) -> Option<Pulse> {
    //    match self {
    //        Self::FlipFlop(_, send) => {
    //            if sig == Pulse::Lo {
    //                Some(*send)
    //            } else {
    //                None
    //            }
    //        },
    //        Self::Conjunction(_, mem, lo) => {
    //            let lo = match (mem.get(from), sig) {
    //                (None | Some(Pulse::Hi), Pulse::Lo) => lo + 1,
    //                (Some(Pulse::Lo), Pulse::Hi) => lo - 1,
    //                _ => *lo,
    //            };

    //            Some(if lo == 0 {
    //                Pulse::Lo
    //            } else {
    //                Pulse::Hi
    //            })
    //        },
    //        Self::Broadcast => Some(sig),
    //    }
    //}

    fn process(&mut self, from: &'a str, sig: Pulse) -> Option<Pulse> {
        match self {
            Self::FlipFlop(_, send) => {
                if sig == Pulse::Lo {
                    let out = *send;
                    *send = -*send;
                    Some(out)
                } else {
                    None
                }
            },
            Self::Conjunction(_, mem, lo) => {
                match (mem.insert(from, sig), sig) {
                    (None | Some(Pulse::Hi), Pulse::Lo) => *lo += 1,
                    (Some(Pulse::Lo), Pulse::Hi) => *lo -= 1,
                    _ => (),
                };

                Some(if *lo == 0 {
                    Pulse::Lo
                } else {
                    Pulse::Hi
                })
            },
            Self::Broadcast => Some(sig),
        }
    }
}

impl<'a> From<&'a str> for ModuleKind<'a> {
    fn from(s: &'a str) -> Self {
        if let Some(lbl) = s.strip_prefix('%') {
            Self::FlipFlop(lbl, Pulse::Hi)
        } else if let Some(lbl) = s.strip_prefix('&') {
            Self::Conjunction(lbl, HashMap::new(), 0)
        } else if s == START_LBL {
            Self::Broadcast
        } else {
            panic!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pulse {
    Lo,
    Hi,
}
use Pulse::*;

impl Neg for Pulse {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Self::Lo => Self::Hi,
            Self::Hi => Self::Lo,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(32000000, Puzzle::part1(
                "broadcaster -> a, b, c
                 %a -> b
                 %b -> c
                 %c -> inv
                 &inv -> a"));
    }

    #[test]
    fn part2() {
        assert_eq!(2, Puzzle::part2(
                "broadcaster -> a
                 %a -> rx"));

        assert_eq!(4, Puzzle::part2(
                "broadcaster -> a
                 %a -> b
                 %b -> rx"));

        assert_eq!(1, Puzzle::part2(
                "broadcaster -> a
                 %a -> inv, con
                 &inv -> b
                 %b -> con
                 &con -> rx"));
    }
}
