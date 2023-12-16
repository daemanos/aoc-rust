use crate::Soln;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = usize;

    fn part1(input: &str) -> Self::Answer {
        input.trim().split(',').map(hash).sum()
    }

    fn part2(input: &str) -> Self::Answer {
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
        for instr in input.trim().split(',') {
            match instr.into() {
                Instr::Ins(lens@Lens {label, ..}) => {
                    let bx = &mut boxes[hash(label)];
                    match bx.iter_mut().find(|lens| lens.label == label) {
                        Some(old) => *old = lens,
                        None => bx.push(lens),
                    }
                }
                Instr::Rem(label) => {
                    let bx = &mut boxes[hash(label)];
                    for i in 0..bx.len() {
                        if bx[i].label == label {
                            bx.remove(i);
                            break;
                        }
                    }
                }
            };
        }

        boxes.iter().enumerate()
            .map(|(i, bx)| {
                bx.iter().enumerate()
                    .map(|(j, Lens {focal_len, ..})| (i+1)*(j+1)*focal_len)
                    .sum::<usize>()
            }).sum()
    }
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |val, ch| ((val + ch as usize) * 17) % 256)
}

#[derive(Debug, Clone)]
enum Instr<'a> {
    Rem(&'a str),
    Ins(Lens<'a>),
}

impl<'a> From<&'a str> for Instr<'a> {
    fn from(s: &'a str) -> Self {
        if s.contains('=') {
            let (label, focal_len) = s.split_once('=').unwrap();
            let focal_len = focal_len.parse().unwrap();

            Self::Ins(Lens {label, focal_len})
        } else {
            let label = s.trim_end_matches('-');
            Self::Rem(label)
        }
    }
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_len: usize,
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
    #[ignore]
    fn part2() {
        //assert_eq!((), Puzzle::part2(""));
    }
}
