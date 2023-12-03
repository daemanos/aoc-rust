use std::cmp;
use std::collections::HashMap;
use std::str::FromStr;

use crate::Soln;

pub struct Puzzle;
impl Soln for Puzzle {
    type Answer = u32;

    fn part1(input: &str) -> Self::Answer {
        let actual = Cubes { red: 12, green: 13, blue: 14 };

        input.lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.trim().split(':').collect();
                let id: u32 = parts[0].strip_prefix("Game ").unwrap().parse().unwrap();

                for cubes in parts[1].trim().split(';') {
                    let cubes: Cubes = cubes.parse().unwrap();
                    if !cubes.check(&actual) {
                        return None;
                    }
                }

                Some(id)
            }).sum()
    }

    fn part2(input: &str) -> Self::Answer {
        input.lines()
            .map(|line| {
                let parts: Vec<&str> = line.split(':').collect();

                parts[1].trim().split(';')
                    .map(|cubes| cubes.parse::<Cubes>().unwrap())
                    .reduce(Cubes::max)
                    .unwrap()
                    .power()
            }).sum()
    }
}

#[derive(Debug, Clone, Copy)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn check(&self, actual: &Cubes) -> bool {
        self.red <= actual.red && self.green <= actual.green && self.blue <= actual.blue
    }

    fn max(self, other: Self) -> Self {
        let red = cmp::max(self.red, other.red);
        let green = cmp::max(self.green, other.green);
        let blue = cmp::max(self.blue, other.blue);

        Self { red, green, blue }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for Cubes {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colors: HashMap<&str, u32> = HashMap::new();
        for part in s.split(',') {
            let words: Vec<&str> = part.trim().split(' ').collect();
            match words.as_slice() {
                &[n, color] => colors.insert(color, n.parse().unwrap()),
                _ => panic!(),
            };
        }

        let red = colors.get("red").copied().unwrap_or(0);
        let green = colors.get("green").copied().unwrap_or(0);
        let blue = colors.get("blue").copied().unwrap_or(0);
        Ok(Cubes { red, green, blue })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str =
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
         Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
         Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
         Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
         Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part1() {
        assert_eq!(8, Puzzle::part1(&INPUT));
    }

    #[test]
    fn part2() {
        assert_eq!(2286, Puzzle::part2(&INPUT));
    }
}
