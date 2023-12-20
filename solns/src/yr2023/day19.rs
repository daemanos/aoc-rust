use std::ops::{Index, IndexMut};

use crate::Soln;
use utils::prelude::*;

const START: &'static str = "in";

const MIN_RATING: Rating = 1;
const MAX_RATING: Rating = 4001;
const RATING_INT: Interval<Rating> = Interval(MIN_RATING, MAX_RATING);

type Rating = u64;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = Rating;

    fn part1(input: &str) -> Self::Answer {
        let (workflows, parts) = parse(input);

        parts.iter()
            .filter_map(|part| {
                let mut label = START;
                while let Some(workflow) = workflows.get(&label) {
                    match workflow.apply(part) {
                        Action::Send(new) => label = new,
                        Action::Accept => return Some(part.score()),
                        Action::Reject => return None,
                    };
                }

                None
            }).sum()
    }

    fn part2(input: &str) -> Self::Answer {
        let (workflows, _) = parse(input);
        let start = workflows.get(&START).unwrap();

        let mut valid = vec![];
        let mut stack = vec![(PartCombos::new(), start)];
        while let Some((parts, workflow)) = stack.pop() {
            workflow.rules.iter()
                .fold(Some(parts), |acc, rule| {
                    acc.and_then(|acc| {
                        let split = acc.split(rule);

                        let (accept, action) = split.accept;
                        if !accept.is_empty() {
                            match action {
                                Action::Send(new) => {
                                    let new = workflows.get(new).unwrap();
                                    stack.push((accept, new));
                                },
                                Action::Accept => valid.push(accept),
                                Action::Reject => (),
                            };
                        }

                        split.reject
                    })
                });
        }

        valid.iter()
            .map(PartCombos::score)
            .sum()
    }
}

fn parse(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let mut lines = input.lines().map(str::trim);

    // parse workflows
    let mut workflows = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        } else {
            let workflow: Workflow = line.into();
            workflows.insert(workflow.label, workflow);
        }
    }

    // parse parts
    let parts = lines
        .map(|line| line.parse().unwrap())
        .collect();

    (workflows, parts)
}

struct Workflow<'a> {
    label: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn apply(&self, part: &Part) -> &Action<'a> {
        self.rules.iter()
            .filter_map(|rule| rule.check(part))
            .next()
            .unwrap()
    }
}

impl<'a> From<&'a str> for Workflow<'a> {
    fn from(s: &'a str) -> Self {
        let (label, rules) = s.split_once('{').unwrap();
        let rules = rules.strip_suffix('}').unwrap()
            .split(',')
            .map(|word| word.into())
            .collect();

        Self { label, rules }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule<'a> {
    Checked {
        cat: Cat,
        oper: Oper,
        val: Rating,
        action: Action<'a>,
    },
    Unchecked(Action<'a>),
}

impl<'a> Rule<'a> {
    fn check(&self, part: &Part) -> Option<&Action<'a>> {
        match self {
            Self::Checked{cat, oper, val, action} => {
                let rating = part[*cat];
                if oper.apply(rating, *val) {
                    Some(action)
                } else {
                    None
                }
            },
            Self::Unchecked(action) => Some(action),
        }
    }
}

impl<'a> fmt::Display for Rule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Checked {cat, oper, val, action} =>
                write!(f, "{cat}{oper}{val}:{action}"),
            Self::Unchecked(action) => write!(f, "{action}"),
        }
    }
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        if let Some((rule, action)) = s.split_once(':') {
            let mut chars = rule.chars();
            let cat = Cat::try_from(chars.next().unwrap()).unwrap();
            let oper = Oper::try_from(chars.next().unwrap()).unwrap();
            let val = chars.as_str().parse().unwrap();

            let action = action.into();

            Self::Checked { cat, oper, val, action }
        } else {
            let action = s.into();
            Self::Unchecked(action)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PartCombos {
    x: Interval<Rating>,
    m: Interval<Rating>,
    a: Interval<Rating>,
    s: Interval<Rating>,
}

struct Split<'a> {
    accept: (PartCombos, &'a Action<'a>),
    reject: Option<PartCombos>,
}

impl PartCombos {
    fn new() -> Self {
        Self {
            x: RATING_INT,
            m: RATING_INT,
            a: RATING_INT,
            s: RATING_INT,
        }
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty()
            || self.m.is_empty()
            || self.a.is_empty()
            || self.s.is_empty()
    }

    fn score(&self) -> Rating {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    fn with_cat(&self, cat: Cat, val: Interval<Rating>) -> Self {
        let mut new = self.clone();
        new[cat] = val;
        new
    }

    fn split<'a>(self, rule: &'a Rule) -> Split<'a> {
        match rule {
            Rule::Checked {cat, oper, val, action} => {
                let Interval(min, max) = self[*cat];
                let (accept, reject) = match oper {
                    Oper::Lt => (Interval(min, *val), Interval(*val, max)),
                    Oper::Gt => (Interval(val+1, max), Interval(min, val+1)),
                };

                let accept = (self.with_cat(*cat, accept), action);
                let reject = Some(self.with_cat(*cat, reject));
                Split {accept, reject}
            },
            Rule::Unchecked(action) => {
                let accept = (self, action);
                let reject = None;
                Split {accept, reject}
            },
        }
    }
}

impl fmt::Display for PartCombos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{x={}, m={}, a={}, s={}}}", self.x, self.m, self.a, self.s)
    }
}

impl Index<Cat> for PartCombos {
    type Output = Interval<Rating>;
    fn index(&self, index: Cat) -> &Self::Output {
        match index {
            Cat::Xtreme => &self.x,
            Cat::Musical => &self.m,
            Cat::Aero => &self.a,
            Cat::Shiny => &self.s,
        }
    }
}

impl IndexMut<Cat> for PartCombos {
    fn index_mut(&mut self, index: Cat) -> &mut Self::Output {
        match index {
            Cat::Xtreme => &mut self.x,
            Cat::Musical => &mut self.m,
            Cat::Aero => &mut self.a,
            Cat::Shiny => &mut self.s,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Part {
    x: Rating,
    m: Rating,
    a: Rating,
    s: Rating,
}

impl Part {
    fn score(&self) -> Rating {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ratings: Vec<Rating> = s
            .strip_braces().unwrap()
            .split(',')
            .map(|word| {
                let (_, rating) = word.split_once('=').unwrap();
                rating.parse().unwrap()
            }).collect();

        match ratings.as_slice() {
            &[x, m, a, s] => Ok(Self{x, m, a, s}),
            _ => Err(())
        }
    }
}

impl Index<Cat> for Part {
    type Output = Rating;
    fn index(&self, index: Cat) -> &Self::Output {
        match index {
            Cat::Xtreme => &self.x,
            Cat::Musical => &self.m,
            Cat::Aero => &self.a,
            Cat::Shiny => &self.s,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Action<'a> {
    Send(&'a str),
    Accept,
    Reject,
}

impl<'a> fmt::Display for Action<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Send(label) => write!(f, "{label}"),
            Self::Accept => write!(f, "A"),
            Self::Reject => write!(f, "R"),
        }
    }
}

impl<'a> From<&'a str> for Action<'a> {
    fn from(s: &'a str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Send(s),
        }
    }
}

#[derive(Debug, Charnum)]
#[repr(u8)]
enum Oper {
    Lt = b'<',
    Gt = b'>',
}

impl Oper {
    fn apply(&self, a: Rating, b: Rating) -> bool {
        match *self {
            Oper::Lt => a < b,
            Oper::Gt => a > b,
        }
    }
}

#[derive(Debug, Charnum)]
#[repr(u8)]
enum Cat {
    Xtreme = b'x',
    Musical = b'm',
    Aero = b'a',
    Shiny = b's',
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn part1() {
        assert_eq!(19114, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(167409079868000, Puzzle::part2(&INPUT));
    }
}
